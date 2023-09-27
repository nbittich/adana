use crate::cache_command::CacheCommand;

use adana_script_core::{constants::*, Operator};
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
    //READ_LINES,
    RETURN,
    LN,
    SIN,
    COS,
    TYPE_OF,
    TAN,
    INCLUDE,
    WHILE,
    ELSE,
    REQUIRE,
    MULTILINE,
    STRUCT,
    Operator::Add.as_str(),
    Operator::Subtr.as_str(),
    Operator::Div.as_str(),
    Operator::Mult.as_str(),
    Operator::Pow.as_str(),
    Operator::Pow2.as_str(),
    Operator::Pow3.as_str(),
    Operator::Not.as_str(),
    Operator::Mod.as_str(),
    Operator::Less.as_str(),
    Operator::Greater.as_str(),
    Operator::LessOrEqual.as_str(),
    Operator::GreaterOrEqual.as_str(),
    Operator::Equal.as_str(),
    Operator::NotEqual.as_str(),
    Operator::And.as_str(),
    Operator::Or.as_str(),
];

pub fn check_reserved_keyword(aliases: &[&str]) -> bool {
    CACHE_COMMAND_DOC
        .iter()
        .flat_map(|c| c.0.iter())
        .chain(FORBIDDEN_VARIABLE_NAME.iter())
        .any(|c| aliases.iter().any(|al| al.eq_ignore_ascii_case(c)))
}
