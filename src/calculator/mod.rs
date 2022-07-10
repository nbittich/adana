mod ast;
mod compute;
mod number;
mod parser;

#[cfg(test)]
mod tests;

use crate::prelude::{Deserialize, Serialize};
pub use compute::compute;
pub use number::Number;

// region: structs
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(super) enum Value<'a> {
    Expression(Vec<Value<'a>>),
    Operation(Operator),
    Decimal(f64),
    Integer(i128),
    BlockParen(Vec<Value<'a>>),
    Variable(&'a str),
    VariableNegate(&'a str),
    VariableExpr { name: Box<Value<'a>>, expr: Box<Value<'a>> },
}

#[derive(Debug, Eq, Copy, Clone, PartialEq, Serialize, Deserialize)]
pub(super) enum Operator {
    Add,
    Subtr,
    Mult,
    Div,
    Mod,
    Exp,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(super) enum TreeNodeValue {
    VariableAssign(String),
    Ops(Operator),
    Primitive(Number),
}
// endregion: structs
