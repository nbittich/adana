#![feature(let_chains, btree_drain_filter, exitcode_exit_method)]

mod cache;
mod editor;
mod os_command;
mod parser;
mod prelude;
mod utils;
use std::{fs::OpenOptions, path::Path};

use cache::CacheManager;
use nom::error::ErrorKind;
use os_command::exec_command;
pub use parser::{parse_command, CacheCommand};
pub use prelude::*;
use rustyline::error::ReadlineError;

use crate::utils::clear_terminal;

const CACHE_COMMAND_DOC: &[(&[&str], &str)] = CacheCommand::doc();

lazy_static::lazy_static! {
    static ref CONFIG_FILE_PATH: PathBuf = {
        let mut conf_dir = dirs::config_dir().expect("conf dir not found");
        conf_dir.push(".karsher.conf.json");
        conf_dir
    };

}

fn main() -> anyhow::Result<()> {
    let mut cache_manager = {
        let f = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(CONFIG_FILE_PATH.as_path())
            .expect("cannot open config");

        let reader = BufReader::new(f);
        let cache: Option<CacheManager> = serde_json::from_reader(reader).ok();
        if let Some(manager) = cache {
            manager
        } else {
            CacheManager::default()
        }
    };

    let mut current_cache = {
        cache_manager
            .get_default_cache()
            .as_ref()
            .map_or("DEFAULT".into(), |v| v.clone())
    };

    clear_terminal();

    let mut rl = editor::build_editor();

    loop {
        let readline = editor::read_line(&mut rl, &current_cache);

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str());
                process_command(&mut cache_manager, &mut current_cache, &line)?;
            }
            Err(ReadlineError::Interrupted) | Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                eprintln!("Error: {:?}", err);
                ExitCode::FAILURE.exit_process();
            }
        }
    }

    if let Ok(json) = serde_json::to_string_pretty(&cache_manager) {
        if std::fs::write(CONFIG_FILE_PATH.as_path(), json).is_err() {
            eprintln!("could not write to target conf file. gomenasai");
        }
    } else {
        eprintln!("could not acquire lock or could not serialize to json. sorry! bye.");
    }

    editor::save_history(&mut rl)?;

    println!("BYE");
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

                if CACHE_COMMAND_DOC.iter().flat_map(|c| c.0).any(|c| aliases.contains(c)) {
                    eprintln!("You cannot use a reserved command name as an alias. check help for list of reserved names.");
                } else {
                    if let Some(cache) = cache_manager
                    .get_mut_or_insert(current_cache) {
                        let key = cache.insert(aliases, value);
                        println!("added {value} with hash key {key}");
                    }
                }

            },
            CacheCommand::Remove(key) => {
                if let Some(cache) = cache_manager
                .get_mut_or_insert(current_cache) && let Some(v) = cache.remove(key) {
                    println!("removed {v} with hash key {key}");
                } else  {
                    println!("key {key} not found in current cache {current_cache}");
                }
            },
            CacheCommand::Get(key) => {
                if let Some(cache) = cache_manager
                .get_mut_or_insert(current_cache) && let Some(value) = cache.get(key) {
                    println!("found {value}");
                } else{
                    println!("{key} not found");
                }
            },
            CacheCommand::Exec{key, args} => {
                if let Some(cache) = cache_manager
                .get_mut_or_insert(current_cache) && let Some(value) = cache.get(key) {
                   let _ = exec_command(value, &args).map_err(|e| anyhow::Error::msg(e.to_string()))?;

                } else if !key.trim().is_empty(){
                    println!("{key} not found");
                }
            },
            CacheCommand::Using(key) => {
                current_cache.clear();
                current_cache.push_str(key);
                cache_manager.set_default_cache(current_cache);
                println!("current cache: {key}");
            },
            CacheCommand::ListCache => {
                println!(">> {:?}", cache_manager.get_cache_names());
            }
            CacheCommand::CurrentCache => {
                println!(">> {current_cache}");
            }
            CacheCommand::Dump(key) => {
                if let Some(key) = key {
                        if let Some(cache) = cache_manager.get(key) {
                               let cache = serde_json::to_string_pretty(&cache)?;
                               println!("{cache}")
                        }else{
                            eprintln!("cache {key} not found");
                        }
                } else{
                    let caches = serde_json::to_string_pretty(&*cache_manager)?;
                    println!("{caches}")
                }
            },
            CacheCommand::RemoveCache(key) => {
                if let Some(cache_name) = key && cache_name != current_cache {
                        println!("remove {cache_name}: {}", cache_manager.remove_cache(cache_name).is_some());
                } else{
                   println!("clear all values from {current_cache}: {}",cache_manager.remove_cache(current_cache).is_some());
                }
            },
            CacheCommand::List => {
                if let Some(cache) = cache_manager.get(current_cache){
                    for (key, value, aliases) in cache.list() {
                        println!(">> Key: {key}, Aliases: {aliases:?} => Value: {value}");
                    }
                }
            },
            CacheCommand::Cd(path) => {
                if Path::new(path).exists() {
                    std::env::set_current_dir(path)?;
                    println!(">> working directory {path}");
                } else {
                    eprintln!("path {path} doesn't exist");
                }
            }
            CacheCommand::Help => {
                for doc in CACHE_COMMAND_DOC {
                    let (command, doc) = doc;
                    println!(">> {} : {doc}", command.join("/"));
                }
            },
            CacheCommand::Clear => {
               clear_terminal();
            }

        },
        Err(e) => {
            match e {
                nom::Err::Failure(failure) if failure.code == ErrorKind::Verify => {
                    eprintln!("invalid command: {failure}")
                },
                _ => eprintln!("error parsing command: {e}")
            }
        },
    }
    Ok(())
}
