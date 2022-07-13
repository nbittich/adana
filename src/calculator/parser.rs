use crate::prelude::{
    all_consuming, alpha1, alphanumeric1, alt, cut, delimited, double, many1,
    map, map_parser, multispace0, one_of, preceded, recognize_float,
    separated_pair, tag, tag_no_case, terminated, verify, Res, I128,
};

use super::{BuiltInFunctionType, MathConstants, Operator, Value};

fn tag_no_space<'a>(t: &'a str) -> impl Fn(&'a str) -> Res<&'a str> {
    move |s: &str| delimited(multispace0, tag(t), multispace0)(s)
}
fn tag_no_space_no_case<'a>(t: &'a str) -> impl Fn(&'a str) -> Res<&'a str> {
    move |s: &str| delimited(multispace0, tag_no_case(t), multispace0)(s)
}

fn parse_number(s: &str) -> Res<Value> {
    map_parser(
        recognize_float,
        alt((
            map(all_consuming(I128), Value::Integer),
            map(all_consuming(double), Value::Decimal),
        )),
    )(s)
}

fn parse_bool(s: &str) -> Res<Value> {
    alt((
        map(tag_no_space("true"), |_| Value::Bool(true)),
        map(tag_no_space("false"), |_| Value::Bool(false)),
    ))(s)
}

fn parse_variable(s: &str) -> Res<Value> {
    map_parser(
        alpha1,
        map(
            verify(all_consuming(alphanumeric1), |s: &str| {
                s.len() != 1 || !MathConstants::get_symbols().contains(s)
            }),
            Value::Variable,
        ),
    )(s)
}
fn parse_constant(s: &str) -> Res<Value> {
    map(one_of(MathConstants::get_symbols()), Value::Const)(s)
}

fn parse_paren(s: &str) -> Res<Value> {
    preceded(
        multispace0,
        delimited(
            tag_no_space("("),
            map(many1(parse_value), |v| {
                if v.len() == 1 {
                    v.into_iter().next().unwrap()
                } else {
                    Value::BlockParen(v)
                }
            }),
            cut(tag_no_space(")")),
        ),
    )(s)
}

fn parse_fn(s: &str) -> Res<Value> {
    fn parse_fn<'a>(
        fn_type: BuiltInFunctionType,
    ) -> impl Fn(&'a str) -> Res<Value> {
        let fn_name = match &fn_type {
            BuiltInFunctionType::Sqrt => "sqrt",
            BuiltInFunctionType::Abs => "abs",
            BuiltInFunctionType::Log => "log",
            BuiltInFunctionType::Ln => "ln",
            BuiltInFunctionType::Sin => "sin",
            BuiltInFunctionType::Cos => "cos",
            BuiltInFunctionType::Tan => "tan",
        };
        move |s: &str| {
            map(preceded(tag_no_space_no_case(fn_name), parse_paren), |expr| {
                Value::BuiltInFunction { fn_type, expr: Box::new(expr) }
            })(s)
        }
    }
    alt((
        parse_fn(BuiltInFunctionType::Sqrt),
        parse_fn(BuiltInFunctionType::Abs),
        parse_fn(BuiltInFunctionType::Ln),
        parse_fn(BuiltInFunctionType::Log),
        parse_fn(BuiltInFunctionType::Sin),
        parse_fn(BuiltInFunctionType::Cos),
        parse_fn(BuiltInFunctionType::Tan),
    ))(s)
}

fn parse_value(s: &str) -> Res<Value> {
    preceded(
        multispace0,
        terminated(
            alt((
                parse_paren,
                parse_exp,
                parse_mult,
                parse_mod,
                parse_div,
                parse_add,
                parse_subtr,
                parse_number,
                parse_bool,
                parse_fn,
                parse_variable,
                parse_constant,
            )),
            multispace0,
        ),
    )(s)
}

fn parse_op<'a>(operation: Operator) -> impl Fn(&'a str) -> Res<Value> {
    let sep = match &operation {
        Operator::Add => "+",
        Operator::Subtr => "-",
        Operator::Div => "/",
        Operator::Mult => "*",
        Operator::Pow => "^",
        Operator::Mod => "%",
    };
    move |s| map(tag_no_space(sep), |_| Value::Operation(operation))(s)
}

fn parse_exp(s: &str) -> Res<Value> {
    parse_op(Operator::Pow)(s)
}

fn parse_mult(s: &str) -> Res<Value> {
    parse_op(Operator::Mult)(s)
}

fn parse_div(s: &str) -> Res<Value> {
    parse_op(Operator::Div)(s)
}

fn parse_mod(s: &str) -> Res<Value> {
    parse_op(Operator::Mod)(s)
}
fn parse_add(s: &str) -> Res<Value> {
    parse_op(Operator::Add)(s)
}

fn parse_subtr(s: &str) -> Res<Value> {
    parse_op(Operator::Subtr)(s)
}

fn parse_expression(s: &str) -> Res<Value> {
    map(many1(parse_value), Value::Expression)(s)
}

pub(super) fn parse_var_expr(s: &str) -> Res<Value> {
    preceded(
        multispace0,
        terminated(
            alt((
                map(
                    separated_pair(
                        parse_variable,
                        tag_no_space("="),
                        parse_expression,
                    ),
                    |(name, expr)| Value::VariableExpr {
                        name: Box::new(name),
                        expr: Box::new(expr),
                    },
                ),
                parse_expression,
            )),
            multispace0,
        ),
    )(s)
}

// endregion: parsers
