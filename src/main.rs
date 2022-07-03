mod args;
mod cache;
mod db;
mod editor;
mod os_command;
mod parser;
mod prelude;
mod programs;
mod utils;

use args::*;
use cache::*;
use colors::*;
use db::DbOp;
use nom::error::ErrorKind;
use os_command::exec_command;
use rustyline::error::ReadlineError;
use signal_hook::{consts::SIGINT, iterator::Signals};
use std::{collections::HashMap, path::Path, thread};

pub use parser::{parse_command, CacheCommand};
pub use prelude::*;

use crate::{
    db::{Config, Db},
    utils::clear_terminal,
};

const CACHE_COMMAND_DOC: &[(&[&str], &str)] = CacheCommand::doc();

const VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_NAME: &str = env!("CARGO_PKG_NAME");

const BACKUP_FILE_NAME: &str = "karsherdb.json";

// TODO to be removed
lazy_static::lazy_static! {
    static ref DB_FILE_PATH: PathBuf = {
        let mut db_dir = dirs::data_dir().expect("db not found");
        debug!("db dir: {}", db_dir.as_path().to_string_lossy());
        db_dir.push(".karsherdb");
        if !db_dir.exists() {
            std::fs::create_dir(&db_dir).expect("could not create db directory");
        }
        db_dir.push("karsher.db");
        db_dir
    };
}

fn main() -> anyhow::Result<()> {
    env_logger::init();
    // trap SIGINT when CTRL+C for e.g with docker-compose logs -f
    let mut signals = Signals::new(&[SIGINT])?;

    thread::spawn(move || {
        for sig in signals.forever() {
            debug!("Received signal {:?}", sig);
        }
    });

    let _args = parse_args(std::env::args())?;

    // todo

    clear_terminal();
    println!("{PKG_NAME} v{VERSION}");
    println!("Db Path: {}", DB_FILE_PATH.as_path().to_string_lossy());
    println!();

    match Db::open(Config::default()) {
        Ok(Db::InMemory(mut db)) => start_app(&mut db),
        Ok(Db::FileBased(mut db)) => start_app(&mut db),
        Err(e) => Err(e),
    }
}

fn start_app(db: &mut impl DbOp<String, String>) -> anyhow::Result<()> {
    let mut current_cache = {
        get_default_cache(db).as_ref().map_or("DEFAULT".into(), |v| v.clone())
    };
    let mut rl = editor::build_editor();
    let mut math_ctx = HashMap::new();
    loop {
        let readline = editor::read_line(&mut rl, &current_cache);

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if process_repl(&line, &mut math_ctx).is_err() {
                    process_command(db, &mut current_cache, &line)?;
                }
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                std::process::exit(1);
            }
        }
    }

    editor::save_history(&mut rl)?;

    println!("{}", Style::new().bold().fg(LightBlue).paint("BYE"));
    Ok(())
}

fn process_command(
    db: &mut impl DbOp<String, String>,
    current_cache: &mut String,
    line: &str,
) -> anyhow::Result<()> {
    match parse_command(line) {
        Ok((_, command)) => match command {
            CacheCommand::Add { aliases, value } => {
                if CACHE_COMMAND_DOC
                    .iter()
                    .flat_map(|c| c.0.iter().map(|comm| comm.to_uppercase()))
                    .any(|c| aliases.iter().any(|al| al.to_uppercase() == c))
                {
                    eprintln!("You cannot use a reserved command name as an alias. check help for list of reserved names.");
                } else if let Some(key) =
                    insert_value(db, current_cache, aliases, value)
                {
                    println!(
                        "added {} with hash keys {}",
                        Yellow.paint(value),
                        Red.paint(key)
                    );
                } else {
                    eprintln!("could not insert!");
                }
            }
            CacheCommand::Remove(key) => {
                if let Some(v) = remove_value(db, current_cache, key) {
                    println!(
                        "removed {} with hash key {}",
                        Yellow.paint(v),
                        Red.paint(key.to_string())
                    );
                } else {
                    println!(
                        "key {key} not found in current cache {current_cache}"
                    );
                }
            }
            CacheCommand::Get(key) => {
                if let Some(value) = get_value(db, current_cache, key) {
                    println!("found '{}'", Yellow.paint(value));
                } else {
                    println!("{key} not found");
                }
            }
            CacheCommand::Exec { key, args } => {
                if let Some(value) = get_value(db, current_cache, key) {
                    let _ = exec_command(&value, &args)
                        .map_err(|e| anyhow::Error::msg(e.to_string()))?;
                } else if !key.trim().is_empty() {
                    println!("{key} not found");
                }
            }
            CacheCommand::Using(key) => {
                if set_default_cache(db, key).is_some() {
                    println!(
                        "previous: {}",
                        LightCyan.paint(current_cache.as_str())
                    );
                    current_cache.clear();
                    current_cache.push_str(key);
                }
            }
            CacheCommand::ListCache => {
                println!(
                    ">> [ {} ]",
                    get_cache_names(db)
                        .iter()
                        .map(|c| Red.bold().paint(c).to_string())
                        .collect::<Vec<_>>()
                        .join(", ")
                );
            }
            CacheCommand::CurrentCache => {
                println!(
                    ">> {}",
                    LightBlue.bold().paint(current_cache.to_string())
                );
            }
            CacheCommand::Concat(key) if key == current_cache => {
                eprintln!("You cannot merge a cache with itself!")
            }
            CacheCommand::Concat(key) => {
                if merge(db, key, current_cache).is_some() {
                    println!(
                        "cache {} has been merged with cache {}.",
                        Red.bold().paint(&current_cache.to_string()),
                        Yellow.bold().paint(key)
                    );
                } else {
                    eprintln!("something went wrong!");
                }
            }
            CacheCommand::Dump(key) => {
                if let Some(json) = dump(db, key) {
                    println!("{json}");
                } else {
                    println!("cache doesn't exist!");
                }
            }
            CacheCommand::RemoveCache(key) => {
                if let Some(cache_name) = key {
                    if cache_name != current_cache {
                        println!(
                            "remove {cache_name}: {}",
                            remove_cache(db, cache_name)
                        );
                    } else {
                        clear_values(db, current_cache);
                        println!("clear all values from {current_cache}",);
                    }
                }
            }
            CacheCommand::List => {
                if let Some(values) = list_values(db, current_cache) {
                    for (key, value) in values {
                        println!(
                            ">> Key: {} => Value: '{}'",
                            Red.paint(key),
                            LightCyan.paint(value)
                        );
                    }
                }
            }
            CacheCommand::Backup => {
                let backup_path =
                    std::env::current_dir()?.join(BACKUP_FILE_NAME);
                let backup_path = backup_path.as_path();
                if let Some(()) = backup(db, backup_path) {
                    println!(
                        "db backed up to {}",
                        Red.paint(backup_path.to_string_lossy())
                    );
                }
            }
            CacheCommand::Restore => {
                let backup_path =
                    std::env::current_dir()?.join(BACKUP_FILE_NAME);
                let backup_path = backup_path.as_path();
                if let Some(()) = restore(db, backup_path) {
                    println!(
                        "db restored from {}",
                        Red.paint(backup_path.to_string_lossy())
                    );
                }
            }
            CacheCommand::Cd(path) => {
                if Path::new(path).exists() {
                    std::env::set_current_dir(path)?;
                    println!(
                        ">> working directory {}",
                        LightMagenta.paint(path)
                    );
                } else {
                    eprintln!("path {} doesn't exist", Red.paint(path));
                }
            }
            CacheCommand::Help => {
                for doc in CACHE_COMMAND_DOC {
                    let (command, doc) = doc;
                    println!(
                        ">> {} : {}",
                        command
                            .iter()
                            .map(|c| Yellow.paint(*c).to_string())
                            .collect::<Vec<_>>()
                            .join("/"),
                        LightBlue.paint(*doc)
                    );
                }
            }
            CacheCommand::Clear => {
                clear_terminal();
            }
        },
        Err(e) => match e {
            nom::Err::Failure(failure) if failure.code == ErrorKind::Verify => {
                eprintln!("invalid command: {}", Red.paint(failure.to_string()))
            }
            nom::Err::Error(err) if err.input.trim().is_empty() => {}
            _ => {
                eprintln!("error parsing command: {}", Red.paint(e.to_string()))
            }
        },
    }
    Ok(())
}

fn process_repl(
    line: &str,
    ctx: &mut HashMap<String, f64>,
) -> anyhow::Result<()> {
    let calc = crate::programs::compute(line, ctx)?;
    println!("{calc}");
    Ok(())
}
