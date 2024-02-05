use crate::cache_command::CacheCommand;

use adana_script_core::FORBIDDEN_VARIABLE_NAME as SCRIPT_RESERVED_KEYWORDS;
pub const CACHE_COMMAND_DOC: &[(&[&str], &str)] = CacheCommand::doc();

pub fn check_reserved_keyword(aliases: &[&str]) -> bool {
    CACHE_COMMAND_DOC
        .iter()
        .flat_map(|c| c.0.iter())
        .chain(SCRIPT_RESERVED_KEYWORDS.iter())
        .any(|c| aliases.iter().any(|al| al.eq_ignore_ascii_case(c)))
}
