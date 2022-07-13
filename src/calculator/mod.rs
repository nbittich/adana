mod ast;
mod compute;
mod number;
mod parser;

#[cfg(test)]
mod tests;

use crate::prelude::{Deserialize, Serialize};
pub use compute::compute;
pub use number::Number;

pub const PI: char = 'π';
pub const EULER_NUMBER: char = 'e';

pub const fn constants() -> &'static str {
    concat!('π', 'e')
} 

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub(super) enum Value<'a> {
    Expression(Vec<Value<'a>>),
    Operation(Operator),
    Function{
        fn_type: Function,
        expr: Box<Value<'a>>
    },
    Decimal(f64),
    Integer(i128),
    BlockParen(Vec<Value<'a>>),
    Variable(&'a str),
    Const(char),
    VariableNegate(&'a str),
    VariableExpr { name: Box<Value<'a>>, expr: Box<Value<'a>> },
}
#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub (super) enum Function {
    Sqrt,
    Abs,
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
    BuiltInFunction(Function),
}
