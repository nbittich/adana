use crate::{adana_script::constants::*, cache_command::CacheCommand};

pub const CACHE_COMMAND_DOC: &[(&[&str], &str)] = CacheCommand::doc();

pub const FORBIDDEN_VARIABLE_NAME: &[&str] = &[
    TRUE,
    FALSE,
    TAU,
    IF,
    PI,
    PRINT_LN,
    PRINT,
    LENGTH,
    EULER_NUMBER,
    ABS,
    LOG,
    TO_INT,
    TO_DOUBLE,
    TO_STRING,
    EVAL,
    TO_BOOL,
    SQRT,
    BREAK,
    NULL,
    FOR,
    IN,
    DROP,
    READ_LINES,
    RETURN,
    LN,
    SIN,
    COS,
    TYPE_OF,
    TAN,
    INCLUDE,
    WHILE,
    ELSE,
    MULTILINE,
    STRUCT,
];

pub fn check_reserved_keyword(aliases: &[&str]) -> bool {
    CACHE_COMMAND_DOC
        .iter()
        .flat_map(|c| c.0.iter())
        .chain(FORBIDDEN_VARIABLE_NAME.iter())
        .any(|c| aliases.iter().any(|al| al.eq_ignore_ascii_case(c)))
}
