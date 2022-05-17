#![feature(let_chains)]
use nom::{
    branch::alt,
    bytes::complete::{tag_no_case, take_while, take_while1},
    character::complete::multispace0,
    combinator::{map, rest},
    multi::many0,
    sequence::{delimited, pair, preceded},
    IResult, Parser,
};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::{
    collections::BTreeMap,
    io::{stdout, Write},
};

fn main() {
    let mut cache: BTreeMap<u64, String> = BTreeMap::new();
    let mut cache_aliases: BTreeMap<u64, u64> = BTreeMap::new();
    write_cursor_and_flush();
    let stdin = std::io::stdin();
    loop {
        let mut line = String::new();
        stdin.read_line(&mut line).expect("read error");
        match parse_command(&line) {
            Ok((_, command)) => match command {
                CacheCommand::Add { aliases, value } => {
                    let key = calculate_hash(&value);
                    cache.insert(key, value.to_owned());

                    let aliases: Vec<(u64, &str)> = aliases
                        .iter()
                        .filter_map(|alias| {
                            let k = calculate_hash(alias);
                            if !cache_aliases.contains_key(&k) {
                                Some((k, *alias))
                            } else {
                                None
                            }
                        })
                        .collect();

                    for (hash_alias, _) in &aliases {
                        cache_aliases.insert(*hash_alias, key);
                    }
                    println!("added {value} with hash key {key} and aliases {aliases:?}");
                }
                CacheCommand::Remove(key) => {
                    let key = {
                        if let Some(actual_key) = cache_aliases.remove(&calculate_hash(&key)) {
                            Some(actual_key)
                        } else {
                            key.parse::<u64>().ok()
                        }
                    };

                    if let Some(key) = key && let Some(v) = cache.remove(&key) {
                        cache_aliases = cache_aliases.into_iter().filter(|e| e.1 != key).collect();
                        println!("removed {v} with hash key {key}");
                    }
                }
                CacheCommand::Get(key) => {
                    let parsed_key = {
                        if let Some(actual_key) = cache_aliases.get(&calculate_hash(&key)) {
                            Some(*actual_key)
                        } else {
                            key.parse::<u64>().ok()
                        }
                    };

                    if let Some(key) = parsed_key && let Some(value) = cache.get(&key) {
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

fn write_cursor_and_flush() {
    print!("> ");
    let _ = stdout().flush();
}

fn parse_command(command: &str) -> IResult<&str, CacheCommand> {
    preceded(
        multispace0,
        alt((
            map(
                pair(
                    preceded(
                        delimited(multispace0, tag_no_case("ADD"), multispace0),
                        many0(preceded(
                            delimited(multispace0, tag_no_case("-a"), multispace0),
                            preceded(multispace0, take_while(|c: char| c.is_alphanumeric())),
                        )),
                    ),
                    rest.map(|s: &str| s.trim()),
                ),
                |(aliases, value)| CacheCommand::Add { aliases, value },
            ),
            map(
                preceded(
                    alt((tag_no_case("DEL"), tag_no_case("DELETE"))),
                    preceded(
                        multispace0,
                        take_while1(|s: char| s.is_alphanumeric() || s == '-'),
                    ),
                ),
                CacheCommand::Remove,
            ),
            map(
                preceded(
                    tag_no_case("GET"),
                    preceded(
                        multispace0,
                        take_while1(|s: char| s.is_alphanumeric() || s == '-'),
                    ),
                ),
                CacheCommand::Get,
            ),
        )),
    )(command)
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

enum CacheCommand<'a> {
    Add {
        aliases: Vec<&'a str>,
        value: &'a str,
    },
    Remove(&'a str),
    Get(&'a str),
}
