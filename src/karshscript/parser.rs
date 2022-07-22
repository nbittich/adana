use std::fs::read_to_string;

use nom::{
    bytes::complete::take_while1, combinator::rest, multi::separated_list0,
    sequence::pair,
};

use crate::{
    karshscript::constants::{
        ABS, COS, K_LOAD, LENGTH, LN, LOG, PRINT, PRINT_LN, SIN, SQRT, TAN,
    },
    prelude::{
        all_consuming, alt, cut, delimited, double, many0, many1, map,
        map_parser, multispace0, one_of, opt, preceded, recognize_float,
        separated_pair, tag, tag_no_case, take_until, take_until1, terminated,
        tuple, verify, Res, I128,
    },
    reserved_keywords::check_reserved_keyword,
};

use super::{
    constants::{ELSE, IF, MULTILINE, WHILE},
    BuiltInFunctionType, MathConstants, Operator, Value,
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
        delimited(
            preceded(multispace0, tag("\"")),
            take_until("\""),
            tag_no_space("\""),
        ),
        |s| Value::String(s.to_string()),
    )(s)
}

fn parse_variable(s: &str) -> Res<Value> {
    let allowed_values =
        |s| take_while1(|s: char| s.is_alphanumeric() || s == '_')(s);
    map_parser(
        verify(allowed_values, |s: &str| {
            s.chars().next().filter(|c| c.is_alphabetic()).is_some()
        }),
        map(
            verify(all_consuming(allowed_values), |s: &str| {
                !check_reserved_keyword(&[s])
            }),
            |s: &str| Value::Variable(s.to_string()),
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
            BuiltInFunctionType::Sqrt => SQRT,
            BuiltInFunctionType::Abs => ABS,
            BuiltInFunctionType::Log => LOG,
            BuiltInFunctionType::Ln => LN,
            BuiltInFunctionType::Length => LENGTH,
            BuiltInFunctionType::Sin => SIN,
            BuiltInFunctionType::Cos => COS,
            BuiltInFunctionType::Tan => TAN,
            BuiltInFunctionType::Println => PRINT_LN,
            BuiltInFunctionType::Print => PRINT,
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
        parse_fn(BuiltInFunctionType::Println),
        parse_fn(BuiltInFunctionType::Print),
        parse_fn(BuiltInFunctionType::Length),
    ))(s)
}

fn parse_array(s: &str) -> Res<Value> {
    map(
        preceded(
            tag_no_space("["),
            terminated(
                separated_list0(tag_no_space(","), parse_value),
                preceded(multispace0, tag("]")),
            ),
        ),
        Value::Array,
    )(s)
}
fn parse_array_access(s: &str) -> Res<Value> {
    map(
        pair(
            alt((parse_variable, parse_array)),
            preceded(
                terminated(tag("["), multispace0),
                terminated(parse_value, preceded(multispace0, tag("]"))),
            ),
        ),
        |(arr, idx)| Value::ArrayAccess {
            arr: Box::new(arr),
            index: Box::new(idx),
        },
    )(s)
}

fn parse_value(s: &str) -> Res<Value> {
    preceded(
        multispace0,
        terminated(
            alt((
                parse_array_access,
                parse_array,
                parse_string,
                parse_paren,
                parse_operation,
                parse_number,
                parse_bool,
                parse_builtin_fn,
                parse_variable,
                parse_constant,
            )),
            opt(comments),
        ),
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
    map_parser(
        parse_multiline,
        map(many1(preceded(opt(comments), parse_value)), Value::Expression),
    )(s)
}

pub(super) fn load_file_path(s: &str) -> anyhow::Result<String> {
    let (rest, file_path) = preceded(
        tag_no_space_no_case(K_LOAD),
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
            separated_pair(
                alt((parse_array_access, parse_variable)),
                tag_no_space("="),
                parse_expression,
            ),
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
        preceded(
            tag_no_space(IF),
            tuple((
                parse_paren,
                parse_block,
                opt(preceded(
                    tag_no_space(ELSE),
                    alt((map(parse_if_statement, |v| vec![v]), parse_block)),
                )),
            )),
        ),
        |(cond, exprs, else_expr)| Value::IfExpr {
            cond: Box::new(cond),
            exprs,
            else_expr,
        },
    )(s)
}
fn parse_multiline(s: &str) -> Res<&str> {
    alt((
        preceded(
            tag_no_space(MULTILINE),
            delimited(tag_no_space("{"), take_until("}"), tag_no_space("}")),
        ),
        alt((take_until("\n"), rest)),
    ))(s)
}
fn parse_while_statement(s: &str) -> Res<Value> {
    map(
        preceded(tag_no_space(WHILE), pair(parse_paren, parse_block)),
        |(cond, exprs)| Value::WhileExpr { cond: Box::new(cond), exprs },
    )(s)
}

fn parse_block(s: &str) -> Res<Vec<Value>> {
    preceded(
        tag_no_space("{"),
        terminated(parse_instructions, tag_no_space("}")),
    )(s)
}

pub(super) fn parse_instructions(instructions: &str) -> Res<Vec<Value>> {
    terminated(
        many1(preceded(
            opt(comments),
            alt((
                parse_while_statement,
                parse_if_statement,
                parse_simple_instruction,
            )),
        )),
        opt(comments),
    )(instructions)
}

#[cfg(test)]
mod test {
    use super::parse_multiline;

    #[test]
    fn test_parse_multiline() {
        let (rest, _result) = parse_multiline(
            r#"
        multiline 
        {
            2*(3/4.-12%5 +7^9) -6/12.*4 / 
            sqrt(2*(3/4.-12%5 +7^9) --6/12.*4) + 
            abs(-2*(3/4.-12%5 +7^9) -6/12.*4 / sqrt(5))
        }
        "#,
        )
        .unwrap();
        assert!(rest.is_empty());
    }
}
