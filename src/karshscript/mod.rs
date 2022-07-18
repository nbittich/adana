mod ast;
mod compute;
mod parser;
mod primitive;

pub use compute::compute;
pub use primitive::Primitive;
use strum::EnumCount;

#[macro_use]
pub mod constants {
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
    pub const TRUE: &str = "true";
    pub const FALSE: &str = "false";
    pub const IF: &str = "if";
    pub const TAU: &str = concat!(tau!());
    pub const PI: &str = concat!(pi!());
    pub const EULER_NUMBER: &str = concat!(euler_number!());
    pub const SQRT: &str = "sqrt";
    pub const ABS: &str = "abs";
    pub const LOG: &str = "log";
    pub const LN: &str = "ln";
    pub const SIN: &str = "sin";
    pub const COS: &str = "cos";
    pub const TAN: &str = "tan";
    pub const K_LOAD: &str = "k_load";
}

#[derive(Debug, EnumCount)]
pub(super) enum MathConstants {
    Pi,
    EulerNumber,
    Tau,
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

#[derive(Debug, PartialEq, Clone)]
pub(super) enum Value<'a> {
    Expression(Vec<Value<'a>>),
    Operation(Operator),
    BuiltInFunction { fn_type: BuiltInFunctionType, expr: Box<Value<'a>> },
    Decimal(f64),
    Integer(i128),
    Bool(bool),
    String(&'a str),
    BlockParen(Vec<Value<'a>>),
    Variable(&'a str),
    Const(char),
    VariableNegate(&'a str),
    VariableExpr { name: Box<Value<'a>>, expr: Box<Value<'a>> },
    IfExpr { cond: Box<Value<'a>>, exprs: Vec<Value<'a>> },
    WhileExpr { cond: Box<Value<'a>>, exprs: Vec<Value<'a>> },
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
    Not,
    Less,
    Greater,
    LessOrEqual,
    GreaterOrEqual,
    Equal,
    NotEqual,
    And,
    Or,
}

#[derive(Debug)]
pub(super) enum TreeNodeValue {
    VariableAssign(String),
    Ops(Operator),
    Primitive(Primitive),
    BuiltInFunction(BuiltInFunctionType),
}

#[cfg(test)]
mod tests;

#[cfg(test)]
mod tests_file;
