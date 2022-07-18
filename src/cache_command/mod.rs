mod cache;
mod os_command;
mod parser;
mod process;
pub use cache::get_default_cache;
pub use process::process_command;
use strum::EnumCount;

pub mod constants {
    pub const PUT: &str = "put";
    pub const GET: &str = "get";
    pub const DESCRIBE: &str = "describe";
    pub const DESCRIBE_ALT: &str = "ds";
    pub const LIST_CACHE: &str = "listcache";
    pub const LIST_CACHE_ALT: &str = "lsch";
    pub const CURR_CACHE: &str = "currch";
    pub const CURR_CACHE_ALT: &str = "currentcache";
    pub const RESTORE: &str = "restore";
    pub const BACKUP: &str = "backup";
    pub const BACKUP_ALT: &str = "bckp";
    pub const DEL_CACHE: &str = "delch";
    pub const DEL_CACHE_ALT: &str = "deletecache";
    pub const MERGE_CACHE: &str = "merge";
    pub const MERGE_CACHE_ALT: &str = "mergecache";
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
    pub const HELP: &str = "help";
}

pub use constants::*;

#[derive(Debug, EnumCount)]
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
    PrintScriptContext,
    Help,
}

impl CacheCommand<'_> {
    pub const fn doc() -> &'static [(&'static [&'static str], &'static str)] {
        if CacheCommand::COUNT != 17 {
            panic!("CacheCommand::doc() no longer valid!");
        }
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
            (&[PRINT_SCRIPT_CONTEXT, PRINT_SCRIPT_CONTEXT_ALT], "Print script context"),

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
