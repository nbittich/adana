mod ast;
mod compute;
mod number;
mod parser;

#[cfg(test)]
mod tests;

pub use compute::compute;
pub use number::Number;
use strum::EnumCount;

#[derive(Debug, EnumCount)]
pub(super) enum MathConstants {
    Pi,
    EulerNumber,
    Tau,
}

macro_rules! pi {
    () => {
        'π'
    };
}
macro_rules! euler_number {
    () => {
        'γ'
    };
}
macro_rules! tau {
    () => {
        'τ'
    };
}

impl MathConstants {
    pub(super) const fn get_symbol(&self) -> char {
        match self {
            MathConstants::Pi => pi!(),
            MathConstants::EulerNumber => euler_number!(),
            MathConstants::Tau => tau!(),
        }
    }
    pub(super) const fn get_symbols() -> &'static str {
        concat!(pi!(), euler_number!(), tau!())
    }

    pub(super) const fn _get_variants(
    ) -> &'static [&'static MathConstants; MathConstants::COUNT] {
        &[&MathConstants::Pi, &MathConstants::EulerNumber, &MathConstants::Tau]
    }
}

#[derive(Debug, PartialEq)]
pub(super) enum Value<'a> {
    Expression(Vec<Value<'a>>),
    Operation(Operator),
    BuiltInFunction { fn_type: BuiltInFunctionType, expr: Box<Value<'a>> },
    Decimal(f64),
    Integer(i128),
    BlockParen(Vec<Value<'a>>),
    Variable(&'a str),
    Const(char),
    VariableNegate(&'a str),
    VariableExpr { name: Box<Value<'a>>, expr: Box<Value<'a>> },
}
#[derive(Debug, Copy, Clone, PartialEq)]
pub(super) enum BuiltInFunctionType {
    Sqrt,
    Abs,
    Log,
    Ln,
    Sin,
    Cos,
    Tan,
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub(super) enum Operator {
    Add,
    Subtr,
    Mult,
    Div,
    Mod,
    Pow,
}

#[derive(Debug)]
pub(super) enum TreeNodeValue {
    VariableAssign(String),
    Ops(Operator),
    Primitive(Number),
    BuiltInFunction(BuiltInFunctionType),
}
