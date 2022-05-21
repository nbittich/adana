use nom::combinator::{cut, verify};

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
}

fn remove_spaces<'a, P>(parser: P) -> impl Fn(&'a str) -> Res<&'a str>
where
    P: Fn(&'a str) -> IResult<&'a str, &'a str>,
{
    move |s| delimited(multispace0, &parser, multispace0)(s)
}

fn add_command(command: &str) -> Res<CacheCommand> {
    map(
        pair(
            preceded(
                remove_spaces(tag_no_case("ADD")),
                many0(preceded(
                    remove_spaces(tag_no_case("-a")),
                    preceded(
                        multispace0,
                        cut(verify(
                            take_while(|c: char| c.is_alphanumeric()),
                            |s: &str| s.len() != 0,
                        )),
                    ),
                )),
            ),
            cut(verify(rest.map(|s: &str| s.trim()), |s: &str| s.len() != 0)),
        ),
        |(aliases, value)| CacheCommand::Add { aliases, value },
    )(command)
}
fn del_command(command: &str) -> Res<CacheCommand> {
    map(
        preceded(
            alt((tag_no_case("DEL"), tag_no_case("DELETE"))),
            preceded(
                multispace0,
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
                multispace0,
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
                multispace0,
                take_while1(|s: char| s.is_alphanumeric() || s == '-'),
            ),
        ),
        CacheCommand::Using,
    )(command)
}

pub fn parse_command(command: &str) -> Res<CacheCommand> {
    preceded(
        multispace0,
        alt((add_command, del_command, get_command, using_command)),
    )(command)
}
