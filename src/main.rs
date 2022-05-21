#![feature(let_chains, btree_drain_filter)]
#![allow(dead_code)]

mod cache;
mod parser;
mod prelude;
mod utils;

use cache::CacheManager;
use nom::error::ErrorKind;
pub use parser::{parse_command, CacheCommand};
pub use prelude::*;
use utils::write_cursor_and_flush;

fn main() -> anyhow::Result<()> {
    let mut cache_manager: CacheManager = Default::default();
    let mut current_cache = String::from("DEFAULT");
    write_cursor_and_flush();
    let stdin = std::io::stdin();
    loop {
        let mut line = String::new();
        stdin.read_line(&mut line).expect("read error");
        match parse_command(&line) {
            Ok((_, command)) => match command {
                CacheCommand::Add { aliases, value } => {
                    if let Some(cache) =cache_manager
                    .get_mut_or_insert(&current_cache) {
                        let key = cache.insert(aliases, value);
                        println!("added {value} with hash key {key}");
                    }
                },
                CacheCommand::Remove(key) => {
                    if let Some(cache) =cache_manager
                    .get_mut_or_insert(&current_cache) && let Some(v) = cache.remove(key) {
                        println!("removed {v} with hash key {key}");
                    }else {
                        println!("key {key} not found in current cache {current_cache}");
                    }
                },
                CacheCommand::Get(key) => {
                    if let Some(cache) =cache_manager
                    .get_mut_or_insert(&current_cache) && let Some(value) = cache.get(key) {
                        println!("found {value}");
                    } else {
                        println!("{key} not found");
                    }
                },
                CacheCommand::Using(key) => {
                    current_cache = key.into();
                    println!("current cache: {key}");
                },
                CacheCommand::Dump(key) => {
                    if let Some(key) = key {
                            if let Some(cache) = cache_manager.get(key) {
                                   let cache = serde_json::to_string_pretty(&cache)?;
                                   println!("{cache}")
                            }else{
                                eprintln!("cache {key} not found");
                            }
                    } else{
                        let caches = serde_json::to_string_pretty(&cache_manager)?;
                        println!("{caches}")
                    }
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

        write_cursor_and_flush();
    }
}
