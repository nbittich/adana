use crate::prelude::*;
use strum::{EnumString, EnumVariantNames, VariantNames};

const PUT: &str = "put";
const GET: &str = "get";
const DESCRIBE: &str = "describe";
const DESCRIBE_ALT: &str = "ds";
const LIST_CACHE: &str = "listcache";
const LIST_CACHE_ALT: &str = "lsch";
const CURR_CACHE: &str = "currch";
const CURR_CACHE_ALT: &str = "currentcache";
const RESTORE: &str = "restore";
const BACKUP: &str = "backup";
const BACKUP_ALT: &str = "bckp";
const DEL_CACHE: &str = "delch";
const DEL_CACHE_ALT: &str = "deletecache";
const MERGE_CACHE: &str = "merge";
const MERGE_CACHE_ALT: &str = "mergecache";
const DEL: &str = "del";
const DEL_ALT: &str = "delete";
const EXEC: &str = "exec";
const CD: &str = "cd";
const USE: &str = "use";
const DUMP: &str = "dump";
const CLEAR: &str = "clear";
const CLEAR_ALT: &str = "cls";
const HELP: &str = "help";

#[derive(Debug, EnumString, EnumVariantNames)]
pub enum CacheCommand<'a> {
    Put { aliases: Vec<&'a str>, value: &'a str },
    Describe,
    ListCache,
    CurrentCache,
    Backup,
    Restore,
    DeleteCache(Option<&'a str>),
    Merge(&'a str),
    Del(&'a str),
    Get(&'a str),
    Exec { key: &'a str, args: Option<&'a str> },
    Cd(&'a str),
    Using(&'a str),
    Dump(Option<&'a str>),
    Clear,
    Help,
}

impl CacheCommand<'_> {
    pub const fn doc() -> &'static [(&'static [&'static str], &'static str)] {
        const VARIANTS: &[&str] = CacheCommand::VARIANTS;
        assert!(16 == VARIANTS.len(), "enum doc no longer valid!");
        &[
            (&[PUT], "Put a new value to current cache. can have multiple aliases with option '-a'. e.g `put -a drc -a drcomp docker-compose`"),
            (&[DESCRIBE,DESCRIBE_ALT], "List values within the cache."),
            (&[LIST_CACHE, LIST_CACHE_ALT], "List available caches."),
            (&[CURR_CACHE, CURR_CACHE_ALT], "Current cache."),
            (&[BACKUP, BACKUP_ALT], "Backup the database of caches to the current directory"),
            (&[RESTORE], "Restore the database from current directory"),
            (&[DEL_CACHE,DEL_CACHE_ALT], "Delete cache or clear current cache value."),
            (&[MERGE_CACHE,MERGE_CACHE_ALT], "Merge current with a given cache"),
            (&[DEL,DEL_ALT], "Remove value from cache. Accept either a hashkey or an alias. e.g `del drc`"),
            (&[GET], "Get value from cache. Accept either a hashkey or an alias. e.g `get drc`"),
            (&[EXEC], "Run a value from the cache as an OS command. Accept either a hashkey or an alias. e.g `run drc`"),
            (&[CD], "Navigate to a directory"),
            (&[USE], "Use another cache context default cache is DEFAULT. e.g `use linux`"),
            (&[DUMP], "Dump cache(s) as json. Take an optional parameter, the cache name. e.g `dump linux`"),
            (&[CLEAR, CLEAR_ALT], "Clear the terminal."),
            (&[HELP], "Display Help."),
        ]
    }
}

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

fn concat_command(command: &str) -> Res<CacheCommand> {
    map(
        alt((
            extract_key(tag_no_case(MERGE_CACHE)),
            extract_key(tag_no_case(MERGE_CACHE_ALT)),
        )),
        CacheCommand::Merge,
    )(command)
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
            preceded(multispace1, rest.map(|r: &str| r.trim())),
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
    move |s: &str| {
        preceded(
            &parser,
            preceded(
                multispace1,
                take_while1(|s: char| {
                    s.is_alphanumeric() || s == '-' || s == '_'
                }),
            ),
        )(s)
    }
}

fn using_command(command: &str) -> Res<CacheCommand> {
    map(extract_key(tag_no_case(USE)), CacheCommand::Using)(command)
}

fn dump_command(command: &str) -> Res<CacheCommand> {
    map(
        preceded(
            tag_no_case(DUMP),
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
            restore_command,
            exec_command,
        )),
    )(command)
}
