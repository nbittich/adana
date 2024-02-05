mod args;
mod cache_command;
mod db;
mod editor;
mod prelude;
mod reserved_keywords;

use adana_script_core::primitive::Primitive;
use anyhow::Context;
use args::*;
use db::DbOp;
use log::debug;
use rustyline::error::ReadlineError;
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

use crate::{
    cache_command::{clear_terminal, get_default_cache, process_command},
    db::{Config, Db},
    prelude::get_path_to_shared_libraries,
};
use adana_script::compute;
use prelude::{colors::LightBlue, colors::Style, BTreeMap};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const RUST_VERSION: &str = std::env!("CARGO_PKG_RUST_VERSION");
const PKG_NAME: &str = env!("CARGO_PKG_NAME");

fn main() -> anyhow::Result<()> {
    env_logger::init();

    let args = parse_args(std::env::args())?;

    let config = if args.is_empty() {
        Config::default()
    } else {
        let in_memory = args.iter().any(|a| matches!(a, Argument::InMemory));
        let fallback_in_memory =
            args.iter().any(|a| !matches!(a, Argument::NoFallbackInMemory));
        let db_path = args.iter().find_map(|a| {
            if let Argument::DbPath(path) = a {
                Some(path)
            } else {
                None
            }
        });
        Config::new(db_path, in_memory, fallback_in_memory)
    };

    let history_path = args.iter().find_map(|a| {
        if let Argument::HistoryPath(path) = a {
            Some(path)
        } else {
            None
        }
    });

    let default_cache = args.iter().find_map(|a| {
        if let Argument::DefaultCache(dc) = a {
            Some(dc.clone())
        } else {
            None
        }
    });

    let path_to_shared_lib: PathBuf = args
        .iter()
        .find_map(|a| {
            if let Argument::SharedLibPath(slp) = a {
                Some(PathBuf::from(&slp))
            } else {
                None
            }
        })
        .or_else(|| {
            let path_so = get_path_to_shared_libraries()
                .context("couldn't determine shared library path")
                .unwrap();
            if !path_so.exists() {
                std::fs::create_dir_all(path_so.as_path())
                    .context("could not create directory for shared lib")
                    .unwrap();
            }
            Some(path_so)
        })
        .context("ERR: shared lib path could not be built")?;

    let script_path = args.iter().find_map(|a| {
        if let Argument::ScriptPath(path) = a {
            Some(path)
        } else {
            None
        }
    });

    let mut direct_execution_script = args.iter().find_map(|a| {
        if let Argument::Execute(script) = a {
            Some(Cow::Borrowed(script))
        } else {
            None
        }
    });

    let script = if let Some(script_path) = script_path {
        let pb = PathBuf::from(&script_path);
        if !pb.exists() {
            return Err(anyhow::anyhow!(
                "script path {script_path} doesn't exist"
            ));
        }
        if !pb.is_file() {
            return Err(anyhow::anyhow!(
                "script path {script_path} is not a file"
            ));
        }
        if pb.extension().and_then(|e| e.to_str()).unwrap_or("") != "adana" {
            return Err(anyhow::anyhow!(
                "wrong extension {script_path}. extension must end with .adana"
            ));
        }
        let canon = pb.canonicalize()?;

        let parent = &canon
            .parent()
            .context("no parent directory found for {script_path}")?;
        std::env::set_current_dir(parent)?;
        let script = std::fs::read_to_string(canon)?;
        Some(Cow::Owned(script))
    } else {
        direct_execution_script.take()
    };
    if let Some(script) = script {
        let mut script_context = BTreeMap::new();

        let script_res = {
            match compute(&script, &mut script_context, &path_to_shared_lib) {
                Ok(Primitive::Error(e)) => Err(anyhow::Error::msg(e)),
                Ok(calc) => Ok(calc),
                e @ Err(_) => e,
            }
        };
        match script_res {
            Ok(Primitive::Unit) => {}
            Ok(calc) => println!("{calc}"),
            Err(calc_err) => eprintln!("Error: {calc_err:?}"),
        }
        return Ok(());
    }

    ctrlc::set_handler(|| {
        debug!("catch CTRL-C! DO NOT REMOVE this. receive ctrl+c signal 2")
    })?;
    clear_terminal();
    println!("{PKG_NAME} v{VERSION} (rust version: {RUST_VERSION})");
    println!("shared lib path: {path_to_shared_lib:?}");

    println!();
    match Db::open(config) {
        Ok(Db::InMemory(mut db)) => {
            start_app(&mut db, history_path, &path_to_shared_lib, default_cache)
        }
        Ok(Db::FileBased(mut db)) => {
            println!("Db Path: {}", db.get_path().display());
            start_app(&mut db, history_path, &path_to_shared_lib, default_cache)
        }
        Err(e) => Err(e),
    }
}

fn start_app(
    db: &mut impl DbOp<String, String>,
    history_path: Option<impl AsRef<Path> + Copy>,
    shared_lib_path: impl AsRef<Path> + Copy,
    default_cache: Option<String>,
) -> anyhow::Result<()> {
    let mut current_cache = {
        get_default_cache(db).as_ref().map_or("DEFAULT".into(), |v| v.clone())
    };
    let mut rl = editor::build_editor(history_path);
    let mut script_context = BTreeMap::new();
    let mut previous_dir = std::env::current_dir()?;

    if let Some(dc) = default_cache {
        process_command(
            db,
            &mut script_context,
            &mut current_cache,
            &mut previous_dir,
            &format!("use {dc}"),
        )?;
    }
    loop {
        let readline = editor::read_line(&mut rl, &current_cache);

        match readline {
            Ok(line) => {
                if let Err(e) = rl.add_history_entry(line.as_str()) {
                    debug!("could not write history entry! {e}");
                }

                let script_res = {
                    match compute(&line, &mut script_context, shared_lib_path) {
                        Ok(Primitive::Error(e)) => Err(anyhow::Error::msg(e)),
                        Ok(calc) => Ok(calc),
                        e @ Err(_) => e,
                    }
                };
                match script_res {
                    Ok(Primitive::Unit) => {}
                    Ok(calc) => println!("{calc}"),
                    Err(calc_err) => {
                        if cfg!(debug_assertions) {
                            eprintln!("Error: {calc_err:?}");
                        }
                        match process_command(
                            db,
                            &mut script_context,
                            &mut current_cache,
                            &mut previous_dir,
                            &line,
                        ) {
                            Ok(_) => (),
                            Err(err) => {
                                eprintln!("Error: {calc_err:?}");
                                eprintln!("Err: {err}");
                            }
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                eprintln!("Error: {err:?}");
                std::process::exit(1);
            }
        }
    }

    editor::save_history(&mut rl, history_path)?;

    println!("{}", Style::new().bold().fg(LightBlue).paint("BYE"));
    Ok(())
}
