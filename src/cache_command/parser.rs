use crate::prelude::*;

use super::{constants::*, CacheCommand};

fn add_command(command: &str) -> Res<CacheCommand> {
    map(
        pair(
            preceded(
                preceded(multispace0, tag_no_case(PUT)),
                many0(preceded(
                    preceded(multispace1, tag_no_case("-a")),
                    preceded(
                        multispace1,
                        cut(verify(
                            take_while(|c: char| {
                                c.is_alphanumeric() || c == '_'
                            }),
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
        |(aliases, value)| CacheCommand::Put { aliases, value },
    )(command)
}

fn del_command(command: &str) -> Res<CacheCommand> {
    map(
        alt((extract_key(tag_no_case(DEL)), extract_key(tag_no_case(DEL_ALT)))),
        CacheCommand::Del,
    )(command)
}

fn get_command(command: &str) -> Res<CacheCommand> {
    map(extract_key(tag_no_case(GET)), CacheCommand::Get)(command)
}

fn alias_command(command: &str) -> Res<CacheCommand> {
    map(
        pair(extract_key(tag_no_case(ALIAS)), extract_command),
        |(left, right)| CacheCommand::Alias((left, right)),
    )(command)
}

fn concat_command(command: &str) -> Res<CacheCommand> {
    map(
        alt((
            extract_key(tag_no_case(MERGE_CACHE)),
            extract_key(tag_no_case(MERGE_CACHE_ALT)),
        )),
        CacheCommand::Merge,
    )(command)
}

fn flush_command(command: &str) -> Res<CacheCommand> {
    extract_no_args(|s| tag_no_case(FLUSH)(s), |_| CacheCommand::Flush)(command)
}

fn backup_command(command: &str) -> Res<CacheCommand> {
    extract_no_args(
        |s| alt((tag_no_case(BACKUP_ALT), tag_no_case(BACKUP)))(s),
        |_| CacheCommand::Backup,
    )(command)
}

fn restore_command(command: &str) -> Res<CacheCommand> {
    extract_no_args(tag_no_case(RESTORE), |_| CacheCommand::Restore)(command)
}
fn store_script_context_command(command: &str) -> Res<CacheCommand> {
    map(
        preceded(
            tag_no_case(STORE_SCRIPT_CONTEXT),
            cut(verify(rest, |s: &str| {
                s.is_empty() || s.starts_with(' ') || s == "\n"
            }))
            .and_then(opt(preceded(
                multispace1,
                take_while1(|s: char| s.is_alphanumeric() || s == '-'),
            ))),
        ),
        CacheCommand::StoreScriptContext,
    )(command)
}
fn load_script_context_command(command: &str) -> Res<CacheCommand> {
    map(
        preceded(
            tag_no_case(LOAD_SCRIPT_CONTEXT),
            cut(verify(rest, |s: &str| {
                s.is_empty() || s.starts_with(' ') || s == "\n"
            }))
            .and_then(opt(preceded(
                multispace1,
                take_while1(|s: char| s.is_alphanumeric() || s == '-'),
            ))),
        ),
        CacheCommand::LoadScriptContext,
    )(command)
}
fn print_script_context_command(command: &str) -> Res<CacheCommand> {
    extract_no_args(
        |s| {
            alt((
                tag_no_case(PRINT_SCRIPT_CONTEXT),
                tag_no_case(PRINT_SCRIPT_CONTEXT_ALT),
            ))(s)
        },
        |_| CacheCommand::PrintScriptContext,
    )(command)
}
fn exec_command(command: &str) -> Res<CacheCommand> {
    map(
        pair(
            alt((
                extract_key(tag_no_case(EXEC)),
                take_till1(|s: char| s.is_whitespace()),
            )),
            opt(rest.map(|r: &str| r.trim())),
        ),
        |(key, args)| CacheCommand::Exec { key, args },
    )(command)
}

fn cd_command(command: &str) -> Res<CacheCommand> {
    map(
        preceded(
            tag_no_case(CD),
            opt(preceded(
                multispace1,
                verify(rest.map(|r: &str| r.trim()), |s: &str| !s.is_empty()),
            )),
        ),
        CacheCommand::Cd,
    )(command)
}

fn list_command(command: &str) -> Res<CacheCommand> {
    extract_no_args(
        |s| alt((tag_no_case(DESCRIBE), tag_no_case(DESCRIBE_ALT)))(s),
        |_| CacheCommand::Describe,
    )(command)
}

fn current_cache_command(command: &str) -> Res<CacheCommand> {
    extract_no_args(
        |s| alt((tag_no_case(CURR_CACHE_ALT), tag_no_case(CURR_CACHE)))(s),
        |_| CacheCommand::CurrentCache,
    )(command)
}

fn help_command(command: &str) -> Res<CacheCommand> {
    extract_no_args(tag_no_case(HELP), |_| CacheCommand::Help)(command)
}

fn clear_command(command: &str) -> Res<CacheCommand> {
    extract_no_args(
        |s| alt((tag_no_case(CLEAR), tag_no_case(CLEAR_ALT)))(s),
        |_| CacheCommand::Clear,
    )(command)
}

fn list_cache_command(command: &str) -> Res<CacheCommand> {
    extract_no_args(
        |s| alt((tag_no_case(LIST_CACHE), tag_no_case(LIST_CACHE_ALT)))(s),
        |_| CacheCommand::ListCache,
    )(command)
}

fn extract_no_args<'a, F, M>(
    parser: F,
    mapper: M,
) -> impl Fn(&'a str) -> Res<'a, CacheCommand<'a>>
where
    F: Fn(&'a str) -> Res<&'a str>,
    M: Fn(&'a str) -> CacheCommand<'a>,
{
    move |s| {
        map(
            preceded(
                &parser,
                verify(rest, |s: &str| s.trim().is_empty() || s == "\n"),
            ),
            &mapper,
        )(s)
    }
}
fn extract_key<'a, F>(parser: F) -> impl Fn(&'a str) -> Res<&'a str>
where
    F: Fn(&'a str) -> Res<&'a str>,
{
    move |s: &str| preceded(&parser, extract_command)(s)
}

fn using_command(command: &str) -> Res<CacheCommand> {
    map(extract_key(tag_no_case(USE)), CacheCommand::Using)(command)
}

fn extract_command(command: &str) -> Res<&str> {
    preceded(
        multispace1,
        take_while1(|s: char| s.is_alphanumeric() || s == '-' || s == '_'),
    )(command)
}
fn dump_command(command: &str) -> Res<CacheCommand> {
    map(
        preceded(
            tag_no_case(DUMP),
            cut(verify(rest, |s: &str| {
                s.is_empty() || s.starts_with(' ') || s == "\n"
            }))
            .and_then(opt(extract_command)),
        ),
        CacheCommand::Dump,
    )(command)
}
fn del_cache_command(command: &str) -> Res<CacheCommand> {
    map(
        preceded(
            alt((tag_no_case(DEL_CACHE), tag_no_case(DEL_CACHE_ALT))),
            cut(verify(rest, |s: &str| {
                s.is_empty() || s.starts_with(' ') || s == "\n"
            }))
            .and_then(opt(preceded(multispace1, rest.map(|s: &str| s.trim())))),
        ),
        CacheCommand::DeleteCache,
    )(command)
}

pub fn parse_command(command: &str) -> Res<CacheCommand> {
    preceded(
        multispace0,
        alt((
            add_command,
            cd_command,
            del_command,
            get_command,
            alias_command,
            using_command,
            dump_command,
            list_cache_command,
            concat_command,
            current_cache_command,
            del_cache_command,
            list_command,
            help_command,
            clear_command,
            backup_command,
            flush_command,
            restore_command,
            print_script_context_command,
            store_script_context_command,
            load_script_context_command,
            exec_command,
        )),
    )(command)
}
