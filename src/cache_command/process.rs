use std::{collections::BTreeMap, path::PathBuf};

use crate::{
    prelude::colors::*,
    reserved_keywords::{check_reserved_keyword, CACHE_COMMAND_DOC},
};
use adana_db::{DbOp, SCRIPT_CACHE_KEY};
use adana_script::print_ast;
use adana_script_core::primitive::RefPrimitive;
use anyhow::Context;
use nom::error::ErrorKind;
use regex::Regex;

use super::{
    cache::*, clear_terminal, os_command::exec_command, parser::parse_command,
    CacheCommand,
};

const BACKUP_FILE_NAME: &str = "adanadb.json";

pub fn process_command(
    db: &mut impl DbOp<String, String>,
    script_context: &mut BTreeMap<String, RefPrimitive>,
    current_cache: &mut String,
    previous_dir: &mut PathBuf,
    line: &str,
) -> anyhow::Result<()> {
    match parse_command(line) {
        Ok((_, command)) => match command {
            CacheCommand::Put { aliases, value } => {
                if check_reserved_keyword(&aliases) {
                    return Err(anyhow::Error::msg(
                        format!("{}",Red.paint("You cannot use a reserved keyword name as an alias.")),
                    ));
                } else if let Some(key) =
                    insert_value(db, current_cache, aliases, value, false)
                {
                    println!(
                        "added {} with keys {}",
                        Yellow.paint(value),
                        Red.paint(key)
                    );
                } else {
                    return Err(anyhow::Error::msg(format!(
                        "{}",
                        Red.paint("could not insert! Key already exists")
                    )));
                }
            }
            CacheCommand::Alias((left, right)) => {
                if check_reserved_keyword(&[right]) {
                    return Err(anyhow::Error::msg(
                        format!("{}",Red.paint("You cannot use a reserved keyword name as an alias.")),
                    ));
                }
                match (
                    get_value(db, current_cache, left),
                    get_value(db, current_cache, right),
                ) {
                    (Some(value), None) => {
                        if let Some(key) = insert_value(
                            db,
                            current_cache,
                            vec![right],
                            &value,
                            false,
                        ) {
                            println!(
                                "aliased {} with keys {}",
                                Yellow.paint(value),
                                Red.paint(key)
                            );
                        } else {
                            return Err(anyhow::Error::msg(format!(
                                "{}",
                                Red.paint(
                                    "could not alias! Right Key already exists"
                                )
                            )));
                        }
                    }
                    _ => {
                        return Err(anyhow::Error::msg(format!(
                            "{}",
                            Red.paint("could not alias! Wrong combination")
                        )))
                    }
                }
            }
            CacheCommand::Del(key) => {
                if let Some(v) = remove_value(db, current_cache, key, false) {
                    println!(
                        "removed {} with hash key {}",
                        Yellow.paint(v),
                        Red.paint(key.to_string())
                    );
                } else {
                    return Err(anyhow::Error::msg(format!(
                        "key {key} not found in current cache {current_cache}"
                    )));
                }
            }
            CacheCommand::Get(key) => {
                if let Some(value) = get_value(db, current_cache, key) {
                    println!("found '{}'", Yellow.paint(value));
                } else {
                    return Err(anyhow::Error::msg(format!("{key} not found")));
                }
            }
            CacheCommand::Clip(key) => {
                if let Some(value) = get_value(db, current_cache, key) {
                    let mut clipboard = arboard::Clipboard::new()?;
                    clipboard.set_text(&value)?;
                    println!("Copied.");
                } else {
                    return Err(anyhow::Error::msg(format!("{key} not found")));
                }
            }
            CacheCommand::Exec { key, args } => {
                if let Some(value) = get_value(db, current_cache, key) {
                    let _ = exec_command(&value, &args, false)
                        .map_err(|e| anyhow::Error::msg(e.to_string()))?;
                } else if !key.trim().is_empty() {
                    exec_command(key, &args, true)
                        .map_err(|e| anyhow::Error::msg(e.to_string()))?;
                    //return Err(anyhow::Error::msg(format!("{key} not found")));
                }
            }
            CacheCommand::Using(key) => {
                if set_default_cache(db, key).is_some() {
                    // println!(
                    //     "previous: {}",
                    //     LightCyan.paint(current_cache.as_str())
                    // );
                    current_cache.clear();
                    current_cache.push_str(key);
                }
            }
            CacheCommand::ListCache => {
                println!(
                    "{}",
                    get_cache_names(db)
                        .iter()
                        .map(|c| Red.bold().paint(c).to_string())
                        .collect::<Vec<_>>()
                        .join("\n")
                );
            }
            CacheCommand::CurrentCache => {
                println!(
                    "{}",
                    LightBlue.bold().paint(current_cache.to_string())
                );
            }
            CacheCommand::Merge(key) if key == current_cache => {
                return Err(anyhow::Error::msg(
                    "You cannot merge a cache with itself!",
                ));
            }
            CacheCommand::Merge(key) => {
                if merge(db, key, current_cache).is_some() {
                    println!(
                        "cache {} has been merged with cache {}.",
                        Red.bold().paint(&current_cache.to_string()),
                        Yellow.bold().paint(key)
                    );
                } else {
                    return Err(anyhow::Error::msg("something went wrong!"));
                }
            }
            CacheCommand::Dump(key) => {
                if let Some(json) = dump(db, key) {
                    println!("{json}");
                } else {
                    return Err(anyhow::Error::msg("cache doesn't exist!"));
                }
            }
            CacheCommand::DeleteCache(key) => {
                if let Some(cache_name) = key {
                    if cache_name != current_cache {
                        println!(
                            "remove {cache_name}: {}",
                            remove_cache(db, cache_name, false)
                                .unwrap_or(false)
                        );
                    } else {
                        clear_values(db, current_cache, false);
                        println!("clear all values from {current_cache}",);
                    }
                }
            }
            CacheCommand::Describe(regex) => {
                fn print_fn(
                    values: Vec<(String, String)>,
                    pred: impl Fn(&str) -> bool,
                ) {
                    for (key, value) in values {
                        if pred(&key) {
                            println!(
                                "{}\n{}",
                                Red.bold().underline().paint(key),
                                LightCyan.italic().paint(value)
                            );
                        }
                    }
                }

                if let Some(values) = list_values(db, current_cache) {
                    if let Some(regex) = regex {
                        let re = Regex::new(regex)?;
                        print_fn(values, |s| re.is_match(s));
                    } else {
                        print_fn(values, |_| true);
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
            CacheCommand::Flush => match flush(db) {
                Ok(msg) => println!("{msg}"),
                Err(err) => eprintln!("Error: {err:?}"),
            },
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
            CacheCommand::Cd(cdt) => {
                let path_buf = {
                    match cdt {
                        super::ChangeDirectoryType::HomeDirectory(path) => {
                             path.and_then(|p| {
                                dirs::home_dir().map(|hd| hd.join(p))
                            })
                            .or_else(dirs::home_dir)
                            .context(
                              "could not change directory. path {path:?} not found!",
                            )?
                        }
                        super::ChangeDirectoryType::Path(path) => {
                            PathBuf::from(path)
                        }
                        super::ChangeDirectoryType::Previous => {
                            previous_dir.clone()
                        },
                    }
                };
                if path_buf.exists() {
                    let current_dir = std::env::current_dir()?;
                    if current_dir != path_buf {
                        *previous_dir = current_dir;
                    }
                    std::env::set_current_dir(path_buf.as_path())?;
                } else {
                    return Err(anyhow::Error::msg(format!(
                        "path {} doesn't exist",
                        Red.paint(path_buf.to_string_lossy())
                    )));
                }
            }
            CacheCommand::Help => {
                for doc in CACHE_COMMAND_DOC {
                    let (command, doc) = doc;
                    println!(
                        "{}\n{}",
                        command
                            .iter()
                            .map(|c| Yellow
                                .bold()
                                .underline()
                                .paint(*c)
                                .to_string())
                            .collect::<Vec<_>>()
                            .join("/"),
                        White.italic().paint(*doc)
                    );
                }
            }
            CacheCommand::Clear => {
                clear_terminal();
            }
            CacheCommand::PrintScriptContext => {
                let json = serde_json::to_string_pretty(&script_context)?;
                println!("{json}")
            }
            CacheCommand::StoreScriptContext(name) => {
                let name = name.unwrap_or("latest.json");
                let binary = bincode::serialize(&script_context)?;
                remove_value(db, SCRIPT_CACHE_KEY, name, true);

                if insert_value(
                    db,
                    SCRIPT_CACHE_KEY,
                    vec![name],
                    &String::from_utf8_lossy(&binary),
                    true,
                )
                .is_none()
                {
                    return Err(anyhow::Error::msg(format!(
                        "{}",
                        Red.paint("could not insert/update script context!")
                    )));
                } else {
                    println!("Script context stored with key {name}");
                }
            }
            CacheCommand::LoadScriptContext(name) => {
                let name = name.unwrap_or("latest.json");
                let value = get_value(db, SCRIPT_CACHE_KEY, name);
                if let Some(value) = value.and_then(|v| {
                    bincode::deserialize::<BTreeMap<String, RefPrimitive>>(
                        v.as_bytes(),
                    )
                    .ok()
                }) {
                    *script_context = value;
                    println!(
                        "{}",
                        Green.paint("Script context restored from cache.")
                    );
                }
            }
            CacheCommand::PrintAst(script) => {
                print_ast(script)?;
            }
        },
        Err(e) => match e {
            nom::Err::Failure(failure) if failure.code == ErrorKind::Verify => {
                return Err(anyhow::Error::msg(format!(
                    "invalid command: {}",
                    Red.paint(failure.to_string())
                )));
            }
            nom::Err::Error(err) if err.input.trim().is_empty() => {}
            _ => {
                return Err(anyhow::Error::msg(format!(
                    "error parsing command: {}",
                    Red.paint(e.to_string())
                )));
            }
        },
    }
    Ok(())
}
