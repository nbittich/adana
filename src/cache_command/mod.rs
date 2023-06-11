mod cache;
mod os_command;
mod parser;
mod process;
pub use cache::get_default_cache;
pub use process::process_command;
use strum::EnumCount;

pub mod constants {
    pub const PUT: &str = "put";
    pub const ALIAS: &str = "alias";
    pub const GET: &str = "get";
    pub const DESCRIBE: &str = "describe";
    pub const DESCRIBE_ALT: &str = "ds";
    pub const LIST_CACHE: &str = "listns";
    pub const LIST_CACHE_ALT: &str = "lsns";
    pub const CURR_CACHE: &str = "currns";
    pub const CURR_CACHE_ALT: &str = "currentns";
    pub const RESTORE: &str = "restore";
    pub const BACKUP: &str = "backup";
    pub const BACKUP_ALT: &str = "bckp";
    pub const FLUSH: &str = "flush";
    pub const DEL_CACHE: &str = "delns";
    pub const DEL_CACHE_ALT: &str = "deletens";
    pub const MERGE_CACHE: &str = "merge";
    pub const MERGE_CACHE_ALT: &str = "mergens";
    pub const DEL: &str = "del";
    pub const DEL_ALT: &str = "delete";
    pub const EXEC: &str = "exec";
    pub const CD: &str = "cd";
    pub const USE: &str = "use";
    pub const DUMP: &str = "dump";
    pub const CLEAR: &str = "clear";
    pub const CLEAR_ALT: &str = "cls";
    pub const PRINT_SCRIPT_CONTEXT: &str = "print_script_ctx";
    pub const PRINT_SCRIPT_CONTEXT_ALT: &str = "script_ctx";
    pub const STORE_SCRIPT_CONTEXT: &str = "store_script_ctx";
    pub const LOAD_SCRIPT_CONTEXT: &str = "load_script_ctx";
    pub const PRINT_AST: &str = "ast";
    pub const HELP: &str = "help";
}

pub use constants::*;

#[derive(Debug, EnumCount)]
pub enum CacheCommand<'a> {
    Put { aliases: Vec<&'a str>, value: &'a str },
    Describe(Option<&'a str>),
    ListCache,
    CurrentCache,
    Backup,
    Flush,
    Restore,
    DeleteCache(Option<&'a str>),
    Merge(&'a str),
    Alias((&'a str, &'a str)),
    Del(&'a str),
    Get(&'a str),
    Exec { key: &'a str, args: Option<&'a str> },
    Cd(ChangeDirectoryType<'a>),
    Using(&'a str),
    Dump(Option<&'a str>),
    Clear,
    PrintScriptContext,
    StoreScriptContext(Option<&'a str>),
    LoadScriptContext(Option<&'a str>),
    PrintAst(&'a str),
    Help,
}
#[derive(Debug)]
pub enum ChangeDirectoryType<'a> {
    HomeDirectory(Option<&'a str>),
    Path(&'a str),
    Previous,
}

impl CacheCommand<'_> {
    pub const fn doc() -> &'static [(&'static [&'static str], &'static str)] {
        if CacheCommand::COUNT != 22 {
            panic!("CacheCommand::doc() no longer valid!");
        }
        &[
            (&[PUT], "Put a new value to current namespace. can have multiple aliases with option '-a'. e.g `put -a drc -a drcomp docker-compose`"),
            (&[DESCRIBE,DESCRIBE_ALT], "List values within the current namespace. Optional parameter for searching e.g `ds ssh`"),
            (&[LIST_CACHE, LIST_CACHE_ALT], "List available namespaces."),
            (&[CURR_CACHE, CURR_CACHE_ALT], "Print current namespace."),
            (&[BACKUP, BACKUP_ALT], "Backup the database of namespaces to the current directory"),
            (&[FLUSH], "Flush to database."),
            (&[RESTORE], "Restore the database from current directory"),
            (&[ALIAS], "Alias a key with another. e.g alias commit gc"),
            (&[DEL_CACHE,DEL_CACHE_ALT], "Delete namespace or clear current namespace values."),
            (&[MERGE_CACHE,MERGE_CACHE_ALT], "Merge current with a given namespace"),
            (&[DEL,DEL_ALT], "Remove value from namespace. Accept either a hashkey or an alias. e.g `del drc`"),
            (&[GET], "Get value from namespace. Accept either a hashkey or an alias. e.g `get drc`"),
            (&[EXEC], "Run a value from the namespace as an OS command. Accept either a hashkey or an alias. e.g `exec drc`"),
            (&[CD], "Navigate to a directory"),
            (&[USE], "Switch to another namespace. default ns is DEFAULT. e.g `use linux`"),
            (&[DUMP], "Dump namespace(s) as json. Take an optional parameter, the namespace name. e.g `dump linux`"),
            (&[CLEAR, CLEAR_ALT], "Clear the terminal."),
            (&[PRINT_SCRIPT_CONTEXT, PRINT_SCRIPT_CONTEXT_ALT], "Print script context"),
            (&[STORE_SCRIPT_CONTEXT], "Store script context (name optional), e.g store_script_ctx 12022023"),
            (&[LOAD_SCRIPT_CONTEXT], "Load script context (name optional), e.g load_script_ctx 12022023"),
            (&[PRINT_AST], "print ast for script code."),
            (&[HELP], "Display Help."),
        ]
    }
}

pub fn clear_terminal() {
    if cfg!(unix) {
        let _ = std::process::Command::new("clear").status();
    } else if cfg!(windows) {
        let _ = std::process::Command::new("cls").status();
    } else {
        eprintln!("cannot clear the terminal for the target os");
    };
}
