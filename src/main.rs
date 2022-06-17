mod cache;
mod editor;
mod os_command;
mod parser;
mod prelude;
mod programs;
mod utils;
use std::{path::Path, thread};

use cache::CacheManager;
use colors::*;
use nom::error::ErrorKind;
use os_command::exec_command;
pub use parser::{parse_command, CacheCommand};
pub use prelude::*;
use rustyline::error::ReadlineError;
use signal_hook::{consts::SIGINT, iterator::Signals};

use crate::utils::clear_terminal;

const CACHE_COMMAND_DOC: &[(&[&str], &str)] = CacheCommand::doc();

const VERSION: &str = env!("CARGO_PKG_VERSION");
const PKG_NAME: &str = env!("CARGO_PKG_NAME");

const BACKUP_FILE_NAME: &str = "karsherdb.json";

fn main() -> anyhow::Result<()> {
    // trap SIGINT when CTRL+C for e.g with docker-compose logs -f
    let mut signals = Signals::new(&[SIGINT])?;

    thread::spawn(move || {
        for sig in signals.forever() {
            if cfg!(debug_assertions) {
                println!("Received signal {:?}", sig);
            }
        }
    });

    clear_terminal();
    println!("{PKG_NAME} v{VERSION}\n");

    let mut cache_manager = CacheManager::default();

    let mut current_cache = {
        cache_manager
            .get_default_cache()
            .as_ref()
            .map_or("DEFAULT".into(), |v| v.clone())
    };

    let mut rl = editor::build_editor();

    loop {
        let readline = editor::read_line(&mut rl, &current_cache);

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                if process_repl(&line).is_err() {
                    process_command(
                        &mut cache_manager,
                        &mut current_cache,
                        &line,
                    )?;
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

    if let Err(e) = cache_manager.flush_sync() {
        eprintln!("could not write to flush db. gomenasai: {e}");
    }

    editor::save_history(&mut rl)?;

    println!("{}", Style::new().bold().fg(LightBlue).paint("BYE"));
    Ok(())
}

fn process_command(
    cache_manager: &mut CacheManager,
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
                    cache_manager.insert_value(current_cache, aliases, value)
                {
                    println!(
                        "added {} with hash key {}",
                        Yellow.paint(value),
                        Red.paint(key.to_string())
                    );
                } else {
                    eprintln!("could not insert!");
                }
            }
            CacheCommand::Remove(key) => {
                if let Some(v) = cache_manager.remove_value(current_cache, key)
                {
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
                if let Some(value) = cache_manager.get_value(current_cache, key)
                {
                    println!("found '{}'", Yellow.paint(value));
                } else {
                    println!("{key} not found");
                }
            }
            CacheCommand::Exec { key, args } => {
                if let Some(value) = cache_manager.get_value(current_cache, key)
                {
                    let _ = exec_command(&value, &args)
                        .map_err(|e| anyhow::Error::msg(e.to_string()))?;
                } else if !key.trim().is_empty() {
                    println!("{key} not found");
                }
            }
            CacheCommand::Using(key) => {
                if let Some(previous_cache) =
                    cache_manager.set_default_cache(key)
                {
                    current_cache.clear();
                    current_cache.push_str(key);
                    println!("previous: {}", LightCyan.paint(previous_cache));
                }
            }
            CacheCommand::ListCache => {
                println!(
                    ">> [ {} ]",
                    cache_manager
                        .get_cache_names()
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
                if cache_manager.merge(key, current_cache).is_some() {
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
                if let Some(json) = cache_manager.dump(key) {
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
                            cache_manager.remove_cache(cache_name).is_some()
                        );
                    } else {
                        println!(
                            "clear all values from {current_cache}: {}",
                            cache_manager.clear_values(current_cache).is_some()
                        );
                    }
                }
            }
            CacheCommand::List => {
                if let Some(values) = cache_manager.list_values(current_cache) {
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
                if let Some(()) = cache_manager.backup(backup_path) {
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
                if let Some(()) = cache_manager.restore(backup_path) {
                    println!(
                        "db restored from{}",
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

fn process_repl(line: &str) -> anyhow::Result<()> {
    let calc = crate::programs::compute(line)?;
    println!("{calc}");
    Ok(())
}
