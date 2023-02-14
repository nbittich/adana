use crate::{
    prelude::{
        all_consuming, alt, delimited, double, many0, many1, map, map_parser,
        multispace0, one_of, opt, pair, preceded, recognize_float, rest,
        separated_list0, separated_list1, separated_pair, tag, tag_no_case,
        take_until, take_while1, terminated, tuple, verify, Res, I128,
    },
    reserved_keywords::check_reserved_keyword,
};

use super::{
    constants::{
        BREAK, DROP, ELSE, FOR, IF, IN, MULTILINE, NULL, RETURN, STRUCT, WHILE,
    },
    BuiltInFunctionType, MathConstants, Operator, Value,
};

pub(super) fn comments(s: &str) -> Res<Vec<&str>> {
    terminated(
        many0(preceded(tag_no_space("#"), alt((take_until("\n"), rest)))),
        multispace0,
    )(s)
}

fn tag_no_space<'a>(t: &'a str) -> impl Fn(&'a str) -> Res<&'a str> {
    move |s: &str| delimited(multispace0, tag(t), multispace0)(s)
}

#[allow(dead_code)]
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

fn parse_range(s: &str) -> Res<Value> {
    map(
        pair(
            preceded(
                multispace0,
                terminated(
                    map_parser(
                        take_until(".."),
                        all_consuming(alt((parse_variable, parse_number))),
                    ),
                    tag(".."),
                ),
            ),
            pair(opt(tag("=")), alt((parse_variable, parse_number))),
        ),
        |(start, (incl_both_end, end))| Value::Range {
            start: Box::new(start),
            end: Box::new(end),
            incl_both_end: incl_both_end.is_some(),
        },
    )(s)
}

fn parse_bool(s: &str) -> Res<Value> {
    alt((
        map(tag_no_space("true"), |_| Value::Bool(true)),
        map(tag_no_space("false"), |_| Value::Bool(false)),
    ))(s)
}

fn parse_fstring(s: &str) -> Res<Value> {
    const DELIMITER_F_STRING: &str = r#"""""#;
    let delimiter = |x| tag(DELIMITER_F_STRING)(x);
    let take_unless = |x| take_until(DELIMITER_F_STRING)(x);
    let (rest, content) =
        preceded(delimiter, terminated(take_unless, delimiter))(s)?;
    let mut parameters = vec![];

    let mut param_rest = content;

    while let Ok((pr, param_str)) = preceded(
        take_until("${"),
        delimited(
            tag_no_space("${"),
            delimited(multispace0, take_until("}"), multispace0),
            tag_no_space("}"),
        ),
    )(param_rest)
    {
        param_rest = pr;
        let (_, param_value) = parse_expression(param_str)?;
        parameters.push((format!("${{{param_str}}}"), param_value));
    }

    Ok((
        rest,
        Value::FString(
            content
                .replace("\\n", "\n")
                .replace("\\t", "\t")
                .replace("\\r", "\r")
                .replace("\\\\", "\\"),
            parameters,
        ),
    ))
}
fn parse_string(s: &str) -> Res<Value> {
    map(delimited(tag("\""), take_until("\""), tag("\"")), |s: &str| {
        Value::String(
            s.replace("\\n", "\n")
                .replace("\\t", "\t")
                .replace("\\r", "\r")
                .replace("\\\\", "\\"),
        )
    })(s)
}

fn parse_variable_str(s: &str) -> Res<&str> {
    let allowed_values =
        |s| take_while1(|s: char| s.is_alphanumeric() || s == '_')(s);
    map_parser(
        verify(allowed_values, |s: &str| {
            s.chars()
                .next()
                .filter(|c| c.is_alphabetic() || c == &'_')
                .is_some()
        }),
        verify(all_consuming(allowed_values), |s: &str| {
            !check_reserved_keyword(&[s])
        }),
    )(s)
}

fn parse_variable(s: &str) -> Res<Value> {
    map(parse_variable_str, |s: &str| {
        if s.starts_with('_') {
            Value::VariableUnused
        } else {
            Value::Variable(s.to_string())
        }
    })(s)
}
fn parse_constant(s: &str) -> Res<Value> {
    map(one_of(MathConstants::get_symbols()), Value::Const)(s)
}

fn parse_block_paren(s: &str) -> Res<Value> {
    let parser = |p| many1(parse_value)(p);
    parse_paren(parser)(s)
}

fn parse_paren<'a, F>(parser: F) -> impl Fn(&'a str) -> Res<Value>
where
    F: Fn(&'a str) -> Res<Vec<Value>>,
{
    move |s| {
        preceded(
            tag_no_space("("),
            terminated(map(&parser, Value::BlockParen), tag_no_space(")")),
        )(s)
    }
}

fn parse_fn(s: &str) -> Res<Value> {
    let parser = |p| separated_list0(tag_no_space(","), parse_value)(p);
    let parse_expr = |p| {
        preceded(
            tag_no_space("{"),
            terminated(
                map(map(many1(parse_value), Value::BlockParen), |v| vec![v]),
                tag_no_space("}"),
            ),
        )(p)
    };
    map(
        separated_pair(
            map_parser(take_until("=>"), parse_paren(parser)),
            tag("=>"),
            alt((
                parse_expr,
                parse_block(parse_instructions),
                map_parser(take_until("\n"), parse_expr),
            )),
        ),
        |(parameters, exprs)| Value::Function {
            parameters: Box::new(parameters),
            exprs,
        },
    )(s)
}

fn parse_fn_call(s: &str) -> Res<Value> {
    let parser = |p| {
        preceded(
            tag_no_space("("),
            terminated(
                separated_list0(
                    tag_no_space(","),
                    alt((
                        parse_fn,
                        map(
                            many1(preceded(multispace0, parse_value)),
                            Value::Expression,
                        ),
                    )),
                ),
                tag_no_space(")"),
            ),
        )(p)
    };

    map(
        pair(
            alt((
                parse_fn,
                parse_struct_access,
                parse_array_access,
                parse_variable,
            )),
            map(parser, Value::BlockParen),
        ),
        |(function, parameters)| Value::FunctionCall {
            parameters: Box::new(parameters),
            function: Box::new(function),
        },
    )(s)
}

fn parse_foreach(s: &str) -> Res<Value> {
    let parse_for_expr_header = |s| {
        separated_pair(
            pair(
                opt(terminated(parse_variable_str, tag_no_space(","))),
                parse_variable_str,
            ),
            tag_no_space(IN),
            alt((
                parse_range,
                parse_fn_call,
                parse_struct_access,
                parse_array_access,
                parse_array,
                parse_fstring,
                parse_string,
                parse_variable,
            )),
        )(s)
    };
    map(
        preceded(
            tag_no_space(FOR),
            pair(
                alt((
                    preceded(
                        tag_no_space("("),
                        terminated(parse_for_expr_header, tag_no_space(")")),
                    ),
                    parse_for_expr_header,
                )),
                parse_block(parse_instructions),
            ),
        ),
        |(((idx, v), arr), exprs)| Value::ForeachExpr {
            var: v.to_string(),
            index_var: idx.map(String::from),
            iterator: Box::new(arr),
            exprs,
        },
    )(s)
}

fn parse_drop(s: &str) -> Res<Value> {
    let parser = |p| {
        separated_list1(
            tag_no_space(","),
            alt((parse_struct_access, parse_array_access, parse_variable)),
        )(p)
    };

    map(preceded(tag_no_space(DROP), parse_paren(parser)), |variables| {
        Value::Drop(Box::new(variables))
    })(s)
}

fn parse_builtin_fn(s: &str) -> Res<Value> {
    fn parse_builtin<'a>(
        fn_type: BuiltInFunctionType,
    ) -> impl Fn(&'a str) -> Res<Value> {
        move |s: &str| {
            map(
                preceded(tag_no_space(fn_type.as_str()), parse_block_paren),
                |expr| Value::BuiltInFunction { fn_type, expr: Box::new(expr) },
            )(s)
        }
    }
    alt((
        parse_builtin(BuiltInFunctionType::Sqrt),
        parse_builtin(BuiltInFunctionType::Abs),
        parse_builtin(BuiltInFunctionType::Ln),
        parse_builtin(BuiltInFunctionType::Log),
        parse_builtin(BuiltInFunctionType::Sin),
        parse_builtin(BuiltInFunctionType::Cos),
        parse_builtin(BuiltInFunctionType::ToInt),
        parse_builtin(BuiltInFunctionType::ToDouble),
        parse_builtin(BuiltInFunctionType::TypeOf),
        parse_builtin(BuiltInFunctionType::ToBool),
        parse_builtin(BuiltInFunctionType::ToString),
        parse_builtin(BuiltInFunctionType::Eval),
        parse_builtin(BuiltInFunctionType::Tan),
        parse_builtin(BuiltInFunctionType::Println),
        parse_builtin(BuiltInFunctionType::Print),
        parse_builtin(BuiltInFunctionType::Length),
        parse_builtin(BuiltInFunctionType::Include),
        parse_builtin(BuiltInFunctionType::ReadLines),
    ))(s)
}

fn parse_struct_expr(s: &str) -> Res<Value> {
    // FIXME because this super dumb
    alt((
        parse_fn_call,
        parse_fn,
        map(many1(parse_value), |mut expr| {
            if expr.len() == 1 {
                expr.remove(0)
            } else {
                Value::Expression(expr)
            }
        }),
        parse_value,
    ))(s)
}
pub(super) fn parse_struct(s: &str) -> Res<Value> {
    let pair_key_value = |p| {
        separated_pair(
            preceded(multispace0, terminated(map(parse_variable_str, String::from), multispace0)),
            tag_no_space(":"),
            preceded(opt(comments),parse_struct_expr))
    }(p);
    preceded(
        tag_no_space(STRUCT),
        map(
            delimited(
                tag_no_space("{"),
                many1(preceded(
                    opt(comments),
                    terminated(pair_key_value, opt(tag_no_space(","))),
                )),
                preceded(opt(comments), tag_no_space("}")),
            ),
            |list| Value::Struct(list.into_iter().collect()),
        ),
    )(s)
}
fn parse_array(s: &str) -> Res<Value> {
    map(
        preceded(
            tag_no_space("["),
            terminated(
                separated_list0(
                    tag_no_space(","),
                    alt((parse_fn, parse_value)),
                ),
                preceded(multispace0, tag_no_space("]")),
            ),
        ),
        Value::Array,
    )(s)
}
fn parse_array_access(s: &str) -> Res<Value> {
    map(
        pair(
            alt((parse_array, parse_variable, parse_fstring, parse_string)),
            preceded(
                tag_no_space("["),
                terminated(
                    alt((parse_variable, parse_number)),
                    tag_no_space("]"),
                ),
            ),
        ),
        |(arr, idx)| Value::ArrayAccess {
            arr: Box::new(arr),
            index: Box::new(idx),
        },
    )(s)
}

fn parse_struct_access(s: &str) -> Res<Value> {
    map(
        alt((
            pair(
                alt((parse_struct, parse_variable)),
                preceded(
                    tag("["),
                    terminated(
                        delimited(tag("\""), parse_variable_str, tag("\"")),
                        tag("]"),
                    ),
                ),
            ),
            separated_pair(
                alt((parse_struct, parse_variable)),
                tag_no_space("."),
                parse_variable_str,
            ),
        )),
        |(s, key)| Value::StructAccess {
            struc: Box::new(s),
            key: String::from(key),
        },
    )(s)
}

fn parse_value(s: &str) -> Res<Value> {
    preceded(
        opt(comments),
        terminated(
            alt((
                parse_fn_call,
                parse_struct_access,
                parse_array_access,
                parse_array,
                parse_struct,
                parse_range,
                parse_fn,
                parse_block_paren,
                parse_builtin_fn,
                parse_operation,
                parse_number,
                parse_bool,
                parse_fstring,
                parse_string,
                parse_variable,
                parse_constant,
                parse_null,
            )),
            opt(comments),
        ),
    )(s)
}

fn parse_operation(s: &str) -> Res<Value> {
    fn parse_op<'a>(operation: Operator) -> impl Fn(&'a str) -> Res<Value> {
        move |s| {
            map(tag_no_space(operation.as_str()), |_| {
                Value::Operation(operation)
            })(s)
        }
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
        parse_multiline, // todo this is probably the source of all issues
        map(many1(preceded(opt(comments), parse_value)), |mut v| {
            if v.len() == 1 {
                v.remove(0)
            } else {
                Value::Expression(v)
            }
        }),
    )(s)
}

fn parse_simple_instruction(s: &str) -> Res<Value> {
    alt((
        map(
            separated_pair(
                alt((parse_struct_access, parse_array_access, parse_variable)),
                tag_no_space("="),
                alt((
                    parse_struct,
                    parse_fstring,
                    parse_fn_call,
                    parse_fn,
                    // parse_struct_access, /* FIXME seems not necessary or
                    //                     and also buggy
                    //                     missing test */
                    parse_array_access,
                    parse_array,
                    parse_expression,
                )),
            ),
            |(name, expr)| Value::VariableExpr {
                name: Box::new(name),
                expr: Box::new(expr),
            },
        ),
        alt((
            parse_fn_call,
            parse_fn,
            parse_struct_access,
            parse_array_access,
            parse_struct,
            parse_fstring,
            parse_array,
            parse_expression,
        )),
    ))(s)
}

fn parse_if_statement(s: &str) -> Res<Value> {
    map(
        preceded(
            tag_no_space(IF),
            tuple((
                parse_block_paren,
                parse_block(parse_instructions),
                opt(preceded(
                    tag_no_space(ELSE),
                    alt((
                        map(parse_if_statement, |v| vec![v]),
                        parse_block(parse_instructions),
                    )),
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

fn parse_null(s: &str) -> Res<Value> {
    map(tag_no_space(NULL), |_| Value::Null)(s)
}

fn parse_while_statement(s: &str) -> Res<Value> {
    map(
        preceded(
            tag_no_space(WHILE),
            pair(parse_block_paren, parse_block(parse_instructions)),
        ),
        |(cond, exprs)| Value::WhileExpr { cond: Box::new(cond), exprs },
    )(s)
}

fn parse_block<F>(parser: F) -> impl Fn(&str) -> Res<Vec<Value>>
where
    F: Fn(&str) -> Res<Vec<Value>>,
{
    move |s| {
        preceded(tag_no_space("{"), terminated(&parser, tag_no_space("}")))(s)
    }
}

fn parse_break(s: &str) -> Res<Value> {
    map(tag_no_space(BREAK), |_| Value::Break)(s)
}
fn parse_early_return(s: &str) -> Res<Value> {
    map(preceded(tag_no_space(RETURN), opt(parse_value)), |v| {
        Value::EarlyReturn(Box::new(v))
    })(s)
}

pub(super) fn parse_instructions(instructions: &str) -> Res<Vec<Value>> {
    terminated(
        many1(preceded(
            opt(comments),
            alt((
                parse_foreach,
                parse_while_statement,
                parse_if_statement,
                parse_break,
                parse_early_return,
                parse_simple_instruction,
                parse_drop,
            )),
        )),
        opt(comments),
    )(instructions)
}
