use std::fs::read_to_string;

use nom::{combinator::rest, sequence::pair};

use crate::prelude::{
    all_consuming, alpha1, alphanumeric1, alt, cut, delimited, double, eof,
    many0, many1, map, map_parser, multispace0, one_of, opt, preceded,
    recognize_float, separated_pair, tag, tag_no_case, take_until, take_until1,
    terminated, verify, Res, I128,
};

use super::{
    BuiltInFunctionType, MathConstants, Operator, Value,
    FORBIDDEN_VARIABLE_NAME,
};

fn comments(s: &str) -> Res<Vec<&str>> {
    terminated(
        many0(preceded(tag_no_space("#"), take_until("\n"))),
        multispace0,
    )(s)
}

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

fn parse_string(s: &str) -> Res<Value> {
    map(
        delimited(tag_no_space("\""), take_until1("\""), tag_no_space("\"")),
        Value::String,
    )(s)
}

fn parse_variable(s: &str) -> Res<Value> {
    map_parser(
        alpha1,
        map(
            verify(all_consuming(alphanumeric1), |s: &str| {
                !FORBIDDEN_VARIABLE_NAME.contains(&s)
                    && (s.len() != 1
                        || !MathConstants::get_symbols().contains(s))
            }),
            Value::Variable,
        ),
    )(s)
}
fn parse_constant(s: &str) -> Res<Value> {
    map(one_of(MathConstants::get_symbols()), Value::Const)(s)
}

fn parse_paren(s: &str) -> Res<Value> {
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
    )(s)
}

fn parse_builtin_fn(s: &str) -> Res<Value> {
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
        alt((
            parse_paren,
            parse_operation,
            parse_number,
            parse_bool,
            parse_string,
            parse_builtin_fn,
            parse_variable,
            parse_constant,
        )),
    )(s)
}

fn parse_operation(s: &str) -> Res<Value> {
    fn parse_op<'a>(operation: Operator) -> impl Fn(&'a str) -> Res<Value> {
        let sep: &str = match &operation {
            Operator::Add => "+",
            Operator::Subtr => "-",
            Operator::Div => "/",
            Operator::Mult => "*",
            Operator::Pow => "^",
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
        };
        move |s| map(tag_no_space(sep), |_| Value::Operation(operation))(s)
    }
    alt((
        parse_op(Operator::Pow),
        parse_op(Operator::Mult),
        parse_op(Operator::Mod),
        parse_op(Operator::Div),
        parse_op(Operator::Add),
        parse_op(Operator::Subtr),
        parse_op(Operator::LessOrEqual),
        parse_op(Operator::GreaterOrEqual),
        parse_op(Operator::Less),
        parse_op(Operator::Greater),
        parse_op(Operator::Equal),
        parse_op(Operator::NotEqual),
        parse_op(Operator::Not),
        parse_op(Operator::And),
        parse_op(Operator::Or),
    ))(s)
}

fn parse_expression(s: &str) -> Res<Value> {
    map(many1(parse_value), Value::Expression)(s)
}

pub(super) fn load_file_path(s: &str) -> anyhow::Result<String> {
    let (rest, file_path) = preceded(
        tag_no_space_no_case("k_load"),
        delimited(
            tag_no_space("("),
            delimited(
                tag_no_space(r#"""#),
                take_until1(r#"""#),
                tag_no_space(r#"""#),
            ),
            tag_no_space(")"),
        ),
    )(s)
    .map_err(|e| anyhow::Error::msg(format!("{e}")))?;
    anyhow::ensure!(rest.trim().is_empty(), "Invalid operation!");

    let file = read_to_string(file_path)?;
    Ok(file)
}

fn parse_simple_instruction(s: &str) -> Res<Value> {
    alt((
        map(
            separated_pair(parse_variable, tag_no_space("="), parse_expression),
            |(name, expr)| Value::VariableExpr {
                name: Box::new(name),
                expr: Box::new(expr),
            },
        ),
        parse_expression,
    ))(s)
}

fn parse_if_statement(s: &str) -> Res<Value> {
    map(
        preceded(tag_no_space("if"), pair(parse_paren, parse_block)),
        |(cond, exprs)| Value::IfExpr { cond: Box::new(cond), exprs },
    )(s)
}

fn parse_block(s: &str) -> Res<Vec<Value>> {
    preceded(
        tag_no_space("{"),
        terminated(
            alt((map(parse_if_statement, |v| vec![v]), parse_instructions)),
            tag_no_space("}"),
        ),
    )(s)

    //Ok(("", vec![]))
}

pub(super) fn parse_instructions(instructions: &str) -> Res<Vec<Value>> {
    many1(alt((
        parse_if_statement,
        map_parser(
            preceded(
                opt(comments),
                terminated(
                    alt((
                        take_until(";"),
                        take_until("}"),
                        take_until("\n"),
                        eof,
                        rest,
                    )),
                    terminated(opt(tag_no_space(";")), opt(comments)),
                ),
            ),
            terminated(parse_simple_instruction, opt(comments)),
        ),
    )))(instructions)
}
