#![feature(let_chains, btree_drain_filter, exitcode_exit_method)]

mod cache;
mod os_command;
mod parser;
mod prelude;
mod utils;

use std::fs::OpenOptions;

use cache::CacheManager;
use nom::error::ErrorKind;
use os_command::exec_command;
pub use parser::{parse_command, CacheCommand};
pub use prelude::*;
use rustyline::{error::ReadlineError, Editor};

use crate::utils::clear_terminal;

const CACHE_COMMAND_DOC: &[(&str, &str)] = CacheCommand::doc();

lazy_static::lazy_static! {
    static ref CONFIG_FILE_PATH: PathBuf = {
        let mut conf_dir = dirs::config_dir().expect("conf dir not found");
        conf_dir.push(".karsher.conf.json");
        conf_dir
    };
    static ref HISTORY_FILE_PATH: PathBuf = {
        let mut home_dir = dirs::home_dir().expect("home dir not found");
        home_dir.push(".karsher.history.txt");
        home_dir
    };

}

fn main() -> anyhow::Result<()> {
    let mut cache_manager = {

        let f = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(CONFIG_FILE_PATH.as_path()).expect("cannot open config");

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

    println!(">> Welcome! Using default cache: '{current_cache}'");

    let mut rl = Editor::<()>::new();

    if rl.load_history(HISTORY_FILE_PATH.as_path()).is_err() {
        println!("No previous history.");
    }

    loop {
        let readline = rl.readline(">> ");
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

    rl.save_history(HISTORY_FILE_PATH.as_path())?;

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
                if let Some(cache) = cache_manager
                .get_mut_or_insert(current_cache) {
                    let key = cache.insert(aliases, value);
                    println!("added {value} with hash key {key}");
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
            CacheCommand::Exec(key) => {
                if let Some(cache) = cache_manager
                .get_mut_or_insert(current_cache) && let Some(value) = cache.get(key) {
                   let _ = exec_command(value).map_err(|e| anyhow::Error::msg(e.to_string()))?;

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
                println!("> {:?}", cache_manager.get_cache_names());
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
            CacheCommand::List => {
                if let Some(cache) = cache_manager.get(current_cache){
                    for ele in cache.list() {
                        println!("> {ele}");
                    }
                }
            },
            CacheCommand::Help => {
                for doc in CACHE_COMMAND_DOC {
                    let (command, doc) = doc;
                    println!("> {command} : {doc}");
                }
            },
            CacheCommand::Clear => {
               clear_terminal();
            }

        },
        Err(e) => {
            match e {
                nom::Err::Failure(failure) if failure.code == ErrorKind::Verify => {
                    eprintln!("invalid command")
                },
                _ => eprintln!("error parsing command: {e}")
            }
        },
    }
    Ok(())
}
