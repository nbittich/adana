#![feature(let_chains, btree_drain_filter)]
#![allow(dead_code)]

mod cache;
mod parser;
mod prelude;
mod utils;

use cache::Cache;
pub use parser::{parse_command, CacheCommand};
pub use prelude::*;
use utils::write_cursor_and_flush;

fn main() {
    let mut cache: Cache = Default::default();
    write_cursor_and_flush();
    let stdin = std::io::stdin();
    loop {
        let mut line = String::new();
        stdin.read_line(&mut line).expect("read error");
        match parse_command(&line) {
            Ok((_, command)) => match command {
                CacheCommand::Add { aliases, value } => {
                    let key = cache.insert(aliases, value);
                    println!("added {value} with hash key {key}");
                }
                CacheCommand::Remove(key) => {
                    if let Some(v) = cache.remove(key) {
                        println!("removed {v} with hash key {key}");
                    }
                }
                CacheCommand::Get(key) => {
                    if let Some(value) = cache.get(key) {
                        println!("found {value}");
                    } else {
                        println!("{key} not found");
                    }
                }
            },
            Err(e) => eprintln!("error parsing command: {e}"),
        }

        write_cursor_and_flush();
    }
}
