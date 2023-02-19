mod adana_script;
mod args;
mod cache_command;
mod db;
mod editor;
mod prelude;
mod reserved_keywords;

use adana_script::Primitive;
use args::*;
use db::DbOp;
use log::debug;
use rustyline::error::ReadlineError;
use std::path::Path;

use prelude::{colors::LightBlue, colors::Style, BTreeMap};

use crate::{
    adana_script::compute,
    cache_command::{clear_terminal, get_default_cache, process_command},
    db::{Config, Db},
};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_NAME: &str = env!("CARGO_PKG_NAME");

fn main() -> anyhow::Result<()> {
    env_logger::init();
    ctrlc::set_handler(|| {
        debug!("catch CTRL-C! DO NOT REMOVE this. receive ctrl+c signal 2")
    })?;
    let args = parse_args(std::env::args())?;

    clear_terminal();
    println!("{PKG_NAME} v{VERSION}");

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

    println!();

    match Db::open(config) {
        Ok(Db::InMemory(mut db)) => start_app(&mut db, history_path),
        Ok(Db::FileBased(mut db)) => start_app(&mut db, history_path),
        Err(e) => Err(e),
    }
}

fn start_app(
    db: &mut impl DbOp<String, String>,
    history_path: Option<impl AsRef<Path> + Copy>,
) -> anyhow::Result<()> {
    let mut current_cache = {
        get_default_cache(db).as_ref().map_or("DEFAULT".into(), |v| v.clone())
    };
    let mut rl = editor::build_editor(history_path);
    let mut script_context = BTreeMap::new();
    loop {
        let readline = editor::read_line(&mut rl, &current_cache);

        match readline {
            Ok(line) => {
                if let Err(e) = rl.add_history_entry(line.as_str()) {
                    debug!("could not write history entry! {e}");
                }

                let script_res = {
                    match compute(&line, &mut script_context) {
                        Ok(Primitive::Error(e)) => Err(anyhow::Error::msg(e)),
                        Ok(calc) => Ok(calc),
                        e @ Err(_) => e,
                    }
                };
                match script_res {
                    Ok(calc) => println!("{calc}"),
                    Err(calc_err) => {
                        match process_command(
                            db,
                            &mut script_context,
                            &mut current_cache,
                            &line,
                        ) {
                            Ok(_) => (),
                            Err(err) => {
                                eprintln!("{calc_err}");
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
