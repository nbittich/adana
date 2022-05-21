use nom::{
    character::complete::multispace1,
    combinator::{cut, opt, verify},
};

use crate::prelude::*;

type Res<'a, T> = IResult<&'a str, T>;

pub enum CacheCommand<'a> {
    Add {
        aliases: Vec<&'a str>,
        value: &'a str,
    },
    Remove(&'a str),
    Get(&'a str),
    Using(&'a str),
    Dump(Option<&'a str>),
}

fn add_command(command: &str) -> Res<CacheCommand> {
    map(
        pair(
            preceded(
                preceded(multispace0, tag_no_case("ADD")),
                many0(preceded(
                    preceded(multispace1, tag_no_case("-a")),
                    preceded(
                        multispace1,
                        cut(verify(
                            take_while(|c: char| c.is_alphanumeric()),
                            |s: &str| !s.is_empty(),
                        )),
                    ),
                )),
            ),
            preceded(
                multispace1,
                cut(verify(rest.map(|s: &str| s.trim()), |s: &str| {
                    !s.is_empty()
                })),
            ),
        ),
        |(aliases, value)| CacheCommand::Add { aliases, value },
    )(command)
}
fn del_command(command: &str) -> Res<CacheCommand> {
    map(
        preceded(
            alt((tag_no_case("DEL"), tag_no_case("DELETE"))),
            preceded(
                multispace1,
                take_while1(|s: char| s.is_alphanumeric() || s == '-'),
            ),
        ),
        CacheCommand::Remove,
    )(command)
}
fn get_command(command: &str) -> Res<CacheCommand> {
    map(
        preceded(
            tag_no_case("GET"),
            preceded(
                multispace1,
                take_while1(|s: char| s.is_alphanumeric() || s == '-'),
            ),
        ),
        CacheCommand::Get,
    )(command)
}

fn using_command(command: &str) -> Res<CacheCommand> {
    map(
        preceded(
            tag_no_case("USING"),
            preceded(
                multispace1,
                take_while1(|s: char| s.is_alphanumeric() || s == '-'),
            ),
        ),
        CacheCommand::Using,
    )(command)
}

fn dump_command(command: &str) -> Res<CacheCommand> {
    map(
        preceded(
            tag_no_case("DUMP"),
            cut(verify(rest, |s: &str| {
                s.is_empty() || s.starts_with(' ') || s == "\n"
            }))
            .and_then(opt(preceded(
                multispace1,
                take_while1(|s: char| s.is_alphanumeric() || s == '-'),
            ))),
        ),
        CacheCommand::Dump,
    )(command)
}

pub fn parse_command(command: &str) -> Res<CacheCommand> {
    preceded(
        multispace0,
        alt((
            add_command,
            del_command,
            get_command,
            using_command,
            dump_command,
        )),
    )(command)
}
