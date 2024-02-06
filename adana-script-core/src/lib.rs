pub mod primitive;
use std::collections::BTreeMap;

use constants::{
    BREAK, DROP, ELSE, EULER_NUMBER, FALSE, FOR, IF, IN, MULTILINE, NULL, PI,
    REQUIRE, RETURN, STRUCT, TAU, TO_BINARY, TO_HEX, TRUE, WHILE,
};
use primitive::Primitive;
use serde::{Deserialize, Serialize};
use strum::EnumCount;

use self::constants::TO_STRING;
use self::constants::{
    ABS, COS, EVAL, INCLUDE, LENGTH, LN, LOG, PRINT, PRINT_LN, SIN, SQRT, TAN,
    TO_BOOL, TO_DOUBLE, TO_INT, TYPE_OF,
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
    pub const TO_HEX: &str = "to_hex";
    pub const TO_BINARY: &str = "to_binary";
    pub const TO_INT: &str = "to_int";
    pub const TO_BOOL: &str = "to_bool";
    pub const TO_DOUBLE: &str = "to_double";
    pub const TO_STRING: &str = "to_string";
    pub const ABS: &str = "abs";
    pub const LENGTH: &str = "length";
    pub const LOG: &str = "log";
    pub const LN: &str = "ln";
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
    pub const REQUIRE: &str = "require";
    pub const NATIVE_LIB: &[u8; 14] = b"__native_lib__";
}

#[derive(Debug, EnumCount)]
pub enum MathConstants {
    Pi,
    EulerNumber,
    Tau,
}

impl MathConstants {
    pub const fn get_symbol(&self) -> char {
        match self {
            MathConstants::Pi => pi!(),
            MathConstants::EulerNumber => euler_number!(),
            MathConstants::Tau => tau!(),
        }
    }
    pub const fn get_symbols() -> &'static str {
        concat!(pi!(), euler_number!(), tau!())
    }

    pub const fn _get_variants(
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
    U8(u8),
    I8(i8),
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
    ToBinary,
    ToHex,
    ToDouble,
    ToBool,
    ToString,
    Tan,
    Println,
    Print,
    Eval,
    TypeOf,
    Length,
    Include,
    Require,
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
    BitwiseNot,
    BitwiseAnd,
    BitwiseOr,
    BitwiseXor,
    BitwiseLShift,
    BitwiseRShift,
}

#[derive(Debug)]
pub enum TreeNodeValue {
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
    pub fn as_str(&self) -> &'static str {
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
            BuiltInFunctionType::Require => REQUIRE,
            BuiltInFunctionType::ToInt => TO_INT,
            BuiltInFunctionType::ToHex => TO_HEX,
            BuiltInFunctionType::ToBinary => TO_BINARY,
            BuiltInFunctionType::ToDouble => TO_DOUBLE,
            BuiltInFunctionType::ToBool => TO_BOOL,
            BuiltInFunctionType::ToString => TO_STRING,
        }
    }
}
impl Operator {
    pub const fn as_str(&self) -> &'static str {
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
            Operator::BitwiseNot => "~",
            Operator::BitwiseAnd => "@",
            Operator::BitwiseOr => "|",
            Operator::BitwiseXor => "$",
            Operator::BitwiseLShift => "<<",
            Operator::BitwiseRShift => ">>",
        }
    }
}

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
    TO_HEX,
    TO_BINARY,
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
    Operator::BitwiseNot.as_str(),
    Operator::BitwiseAnd.as_str(),
    Operator::BitwiseOr.as_str(),
    Operator::BitwiseXor.as_str(),
    Operator::BitwiseLShift.as_str(),
    Operator::BitwiseRShift.as_str(),
];
