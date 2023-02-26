mod ast;
mod compute;

mod parser;
mod primitive;

use std::collections::BTreeMap;

pub use compute::compute;
pub use primitive::Primitive;
pub use primitive::RefPrimitive;

use serde::{Deserialize, Serialize};
use strum::EnumCount;

use self::constants::TO_STRING;
use self::constants::{
    ABS, COS, EVAL, INCLUDE, LENGTH, LN, LOG, PRINT, PRINT_LN, READ_LINES, SIN,
    SQRT, TAN, TO_BOOL, TO_DOUBLE, TO_INT, TYPE_OF,
};

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
    pub const ELSE: &str = "else";
    pub const WHILE: &str = "while";
    pub const TAU: &str = concat!(tau!());
    pub const PI: &str = concat!(pi!());
    pub const EULER_NUMBER: &str = concat!(euler_number!());
    pub const SQRT: &str = "sqrt";
    pub const TO_INT: &str = "to_int";
    pub const TO_BOOL: &str = "to_bool";
    pub const TO_DOUBLE: &str = "to_double";
    pub const TO_STRING: &str = "to_string";
    pub const ABS: &str = "abs";
    pub const LENGTH: &str = "length";
    pub const LOG: &str = "log";
    pub const LN: &str = "ln";
    pub const READ_LINES: &str = "read_lines";
    pub const SIN: &str = "sin";
    pub const COS: &str = "cos";
    pub const TAN: &str = "tan";
    pub const BREAK: &str = "break";
    pub const RETURN: &str = "return";
    pub const PRINT_LN: &str = "println";
    pub const PRINT: &str = "print";
    pub const INCLUDE: &str = "include";
    pub const DROP: &str = "drop";
    pub const NULL: &str = "null";
    pub const MULTILINE: &str = "multiline";
    pub const STRUCT: &str = "struct";
    pub const EVAL: &str = "eval";
    pub const TYPE_OF: &str = "type_of";
    pub const FOR: &str = "for";
    pub const IN: &str = "in";
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

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Value {
    Break,
    EarlyReturn(Box<Option<Value>>),
    Drop(Box<Value>),
    Expression(Vec<Value>),
    ImplicitMultiply(Box<Value>),
    Operation(Operator),
    BuiltInFunction {
        fn_type: BuiltInFunctionType,
        expr: Box<Value>,
    },
    Function {
        parameters: Box<Value>,
        exprs: Vec<Value>,
    },
    FunctionCall {
        parameters: Box<Value>,
        function: Box<Value>,
    },
    Null,
    Decimal(f64),
    Integer(i128),
    Bool(bool),
    Range {
        start: Box<Value>,
        incl_both_end: bool,
        end: Box<Value>,
    },
    NoOp,
    String(String),
    FString(String, Vec<(String, Value)>),
    BlockParen(Vec<Value>),
    Variable(String),
    VariableRef(String),
    VariableUnused,
    Const(char),
    VariableNegate(String),
    VariableExpr {
        name: Box<Value>,
        expr: Box<Value>,
    },
    IfExpr {
        cond: Box<Value>,
        exprs: Vec<Value>,
        else_expr: Option<Vec<Value>>,
    },
    WhileExpr {
        cond: Box<Value>,
        exprs: Vec<Value>,
    },
    ForeachExpr {
        var: String,
        index_var: Option<String>,
        iterator: Box<Value>,
        exprs: Vec<Value>,
    },
    Array(Vec<Value>),
    ArrayAccess {
        arr: Box<Value>,
        index: Box<Value>,
    },
    Struct(BTreeMap<String, Value>),
    StructAccess {
        struc: Box<Value>,
        key: String,
    },
}
#[derive(Debug, Copy, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub enum BuiltInFunctionType {
    Sqrt,
    Abs,
    Log,
    Ln,
    Sin,
    Cos,
    ToInt,
    ToDouble,
    ToBool,
    ToString,
    Tan,
    Println,
    ReadLines,
    Print,
    Eval,
    TypeOf,
    Length,
    Include,
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
pub enum Operator {
    Add,
    Subtr,
    Mult,
    Div,
    Mod,
    Pow,
    Pow2,
    Pow3,
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
    Break,
    EarlyReturn(Option<Value>),
    Drop(Vec<Value>),
    VariableUnused,
    VariableAssign(Option<String>),
    VariableArrayAssign { name: String, index: Value },
    Ops(Operator),
    Primitive(Primitive),
    VariableRef(String),
    BuiltInFunction(BuiltInFunctionType),
    IfExpr(Value),
    FString(String, Vec<(String, Value)>),
    WhileExpr(Value),
    Array(Vec<Value>),
    Struct(BTreeMap<String, Value>),
    StructAccess { struc: Value, key: Primitive },
    ArrayAccess { index: Value, array: Value },
    Function(Value),
    FunctionCall(Value),
    Foreach(Value),
    Null,
}

impl BuiltInFunctionType {
    pub(super) fn as_str(&self) -> &'static str {
        match self {
            BuiltInFunctionType::Sqrt => SQRT,
            BuiltInFunctionType::Abs => ABS,
            BuiltInFunctionType::Log => LOG,
            BuiltInFunctionType::Ln => LN,
            BuiltInFunctionType::Length => LENGTH,
            BuiltInFunctionType::Sin => SIN,
            BuiltInFunctionType::Cos => COS,
            BuiltInFunctionType::Tan => TAN,
            BuiltInFunctionType::TypeOf => TYPE_OF,
            BuiltInFunctionType::Println => PRINT_LN,
            BuiltInFunctionType::Print => PRINT,
            BuiltInFunctionType::Eval => EVAL,
            BuiltInFunctionType::Include => INCLUDE,
            BuiltInFunctionType::ReadLines => READ_LINES,
            BuiltInFunctionType::ToInt => TO_INT,
            BuiltInFunctionType::ToDouble => TO_DOUBLE,
            BuiltInFunctionType::ToBool => TO_BOOL,
            BuiltInFunctionType::ToString => TO_STRING,
        }
    }
}
impl Operator {
    pub(super) const fn as_str(&self) -> &'static str {
        match self {
            Operator::Add => "+",
            Operator::Subtr => "-",
            Operator::Div => "/",
            Operator::Mult => "*",
            Operator::Pow => "^",
            Operator::Pow2 => "²",
            Operator::Pow3 => "³",
            Operator::Not => "!",
            Operator::Mod => "%",
            Operator::Less => "<",
            Operator::Greater => ">",
            Operator::LessOrEqual => "<=",
            Operator::GreaterOrEqual => ">=",
            Operator::Equal => "==",
            Operator::NotEqual => "!=",
            Operator::And => "&&",
            Operator::Or => "||",
        }
    }
}

#[cfg(test)]
mod tests;
