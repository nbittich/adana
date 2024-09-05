use nom::{
    combinator::recognize,
    error::{Error, ErrorKind},
};

use crate::prelude::{
    all_consuming, alt, delimited, double, many0, many1, map, map_parser,
    multispace0, one_of, opt, pair, peek, preceded, recognize_float, rest,
    separated_list0, separated_list1, separated_pair, tag, take_until,
    take_while1, terminated, tuple, verify, Res, I128, I8, U8,
};

use super::string_parser::parse_escaped_string;
use adana_script_core::{
    constants::{
        BREAK, DROP, ELSE, FOR, IF, IN, MULTILINE, NULL, RETURN, STRUCT, WHILE,
    },
    primitive::Primitive,
    KeyAccess, FORBIDDEN_VARIABLE_NAME,
};
use adana_script_core::{BuiltInFunctionType, MathConstants, Operator, Value};

pub(super) fn comments(s: &str) -> Res<Vec<&str>> {
    terminated(
        many0(preceded(tag_no_space("#"), alt((take_until("\n"), rest)))),
        multispace0,
    )(s)
}

fn tag_no_space<'a>(t: &'a str) -> impl Fn(&'a str) -> Res<&'a str> {
    move |s: &str| delimited(multispace0, tag(t), multispace0)(s)
}

fn tag_no_space_opt<'a>(
    t: &'a str,
) -> impl Fn(&'a str) -> Res<Option<&'a str>> {
    move |s: &str| opt(tag_no_space(t))(s)
}

fn recognize_hexadecimal(input: &str) -> Res<&str> {
    // <'a, E: ParseError<&'a str>>
    preceded(
        alt((tag("0x"), tag("0X"))),
        recognize(many1(one_of("0123456789abcdefABCDEF"))),
    )(input)
}
fn recognize_binary(input: &str) -> Res<&str> {
    preceded(alt((tag("0b"), tag("0B"))), recognize(many1(one_of("01"))))(input)
}

fn parse_u8_binary(s: &str) -> Res<Value> {
    let r = u8::from_str_radix(s, 2).map_err(|_e| {
        nom::Err::Error(nom::error::make_error::<&str, Error<&str>>(
            "could not parse radix",
            ErrorKind::IsNot,
        ))
    })?;
    Ok(("", Value::U8(r)))
}

fn parse_i128_binary(s: &str) -> Res<Value> {
    let r = i128::from_str_radix(s, 2).map_err(|_e| {
        nom::Err::Error(nom::error::make_error::<&str, Error<&str>>(
            "could not parse radix",
            ErrorKind::IsNot,
        ))
    })?;
    Ok(("", Value::Integer(r)))
}

fn parse_u8_hex(s: &str) -> Res<Value> {
    let r = u8::from_str_radix(s, 16).map_err(|_e| {
        nom::Err::Error(nom::error::make_error::<&str, Error<&str>>(
            "could not parse radix",
            ErrorKind::IsNot,
        ))
    })?;
    Ok(("", Value::U8(r)))
}

fn parse_i128_hex(s: &str) -> Res<Value> {
    let r = i128::from_str_radix(s, 16).map_err(|_e| {
        nom::Err::Error(nom::error::make_error::<&str, Error<&str>>(
            "could not parse radix",
            ErrorKind::IsNot,
        ))
    })?;
    Ok(("", Value::Integer(r)))
}
fn parse_number(s: &str) -> Res<Value> {
    alt((
        map_parser(recognize_binary, parse_u8_binary),
        map_parser(recognize_binary, parse_i128_binary),
        map_parser(recognize_hexadecimal, parse_u8_hex),
        map_parser(recognize_hexadecimal, parse_i128_hex),
        map_parser(
            recognize_float,
            alt((
                map(all_consuming(U8), Value::U8),
                map(all_consuming(I8), Value::I8),
                map(all_consuming(I128), Value::Integer),
                map(all_consuming(double), Value::Decimal),
            )),
        ),
    ))(s)
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
    let (rest, content) = preceded(
        delimiter,
        terminated(
            map(take_until(DELIMITER_F_STRING), |s: &str| {
                s.replace("\\n", "\n")
                    .replace("\\t", "\t")
                    .replace("\\r", "\r")
                    .replace("\\\\", "\\")
            }),
            delimiter,
        ),
    )(s)?;
    let mut parameters = vec![];

    let mut param_rest = content.as_ref();

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
        match parse_complex_expression(param_str) {
            Ok((_, param_value)) => {
                parameters.push((format!("${{{param_str}}}"), param_value));
            }
            Err(e) => {
                eprintln!("{e}");
                let err = nom::error::make_error::<&str, Error<&str>>(
                    "could not parse f string.",
                    ErrorKind::IsNot,
                );
                return Err(nom::Err::Error(err));
            }
        }
    }

    Ok((rest, Value::FString(content, parameters)))
}
fn parse_string(s: &str) -> Res<Value> {
    map(delimited(tag("\""), parse_escaped_string, tag("\"")), |s| {
        Value::String(s.into_owned())
    })(s)
}

fn parse_key_struct(s: &str) -> Res<&str> {
    take_while1(|c: char| c.is_alphanumeric() || c == '_' || c == '-')(s)
}
fn parse_variable_str(s: &str) -> Res<&str> {
    let allowed_values = |s| {
        take_while1(|s: char| {
            (s.is_alphanumeric() && s != '²' && s != '³')
                || s == '_'
                || s == '&'
        })(s)
    };
    map_parser(
        verify(allowed_values, |s: &str| {
            s.chars()
                .next()
                .filter(|c| c.is_alphabetic() || c == &'_' || c == &'&')
                .is_some()
                && s.chars().filter(|c| c == &'&').count() <= 1
        }),
        verify(all_consuming(allowed_values), |s: &str| {
            !default_check_reserved_keywords(&[s])
        }),
    )(s)
}

fn parse_variable(s: &str) -> Res<Value> {
    map(parse_variable_str, |s: &str| {
        if s.starts_with('_') {
            Value::VariableUnused
        } else if s.starts_with('&') {
            Value::VariableRef(s.replace('&', ""))
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
    parse_paren(parser, false)(s)
}
fn parse_block_paren_opt(s: &str) -> Res<Value> {
    let parser = |p| many1(parse_value)(p);
    parse_paren(parser, true)(s)
}

fn parse_paren<'a, F>(
    parser: F,
    paren_opt: bool,
) -> impl Fn(&'a str) -> Res<Value>
where
    F: Fn(&'a str) -> Res<Vec<Value>>,
{
    move |s| {
        let parser = map(&parser, Value::BlockParen);
        if paren_opt {
            preceded(
                tag_no_space_opt("("),
                terminated(parser, tag_no_space_opt(")")),
            )(s)
        } else {
            preceded(tag_no_space("("), terminated(parser, tag_no_space(")")))(
                s,
            )
        }
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
    let parse_single_instr = |p| {
        map(
            map_parser(
                preceded(
                    tag_no_space("{"),
                    terminated(take_until("}"), tag_no_space("}")),
                ),
                parse_simple_instruction,
            ),
            |v| vec![v],
        )(p)
    };
    map(
        separated_pair(
            map_parser(take_until("=>"), parse_paren(parser, false)),
            tag("=>"),
            alt((
                parse_expr,
                parse_block(parse_instructions),
                parse_single_instr,
                map_parser(take_until("\n"), parse_expr),
            )),
        ),
        |(parameters, exprs)| Value::Function {
            parameters: Box::new(parameters),
            exprs,
        },
    )(s)
}

fn parse_fn_args(s: &str) -> Res<Vec<Value>> {
    preceded(
        tag_no_space("("),
        terminated(
            separated_list0(
                tag_no_space(","),
                alt((
                    parse_fn,
                    map(many1(preceded(multispace0, parse_value)), |mut v| {
                        if v.len() == 1 {
                            v.remove(0)
                        } else {
                            Value::Expression(v)
                        }
                    }),
                )),
            ),
            tag_no_space(")"),
        ),
    )(s)
}
fn parse_fn_call(s: &str) -> Res<Value> {
    map(
        pair(
            alt((parse_fn, parse_variable)),
            map(parse_fn_args, Value::BlockParen),
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
                parse_multidepth_access,
                parse_fn_call,
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
                preceded(
                    opt(tag_no_space("(")),
                    terminated(parse_for_expr_header, opt(tag_no_space(")"))),
                ),
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
            alt((parse_multidepth_access, parse_variable)),
        )(p)
    };

    map(preceded(tag_no_space(DROP), parse_paren(parser, false)), |variables| {
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
    fn parse_builtin_many_args<'a>(
        fn_type: BuiltInFunctionType,
    ) -> impl Fn(&'a str) -> Res<Value> {
        move |s: &str| {
            map(
                preceded(tag_no_space(fn_type.as_str()), parse_fn_args),
                |expr| Value::BuiltInFunction {
                    fn_type,
                    expr: Box::new(Value::Array(expr)),
                },
            )(s)
        }
    }
    alt((
        parse_builtin(BuiltInFunctionType::Eval),
        parse_builtin(BuiltInFunctionType::Println),
        parse_builtin(BuiltInFunctionType::Print),
        parse_builtin(BuiltInFunctionType::Length),
        parse_builtin(BuiltInFunctionType::Include),
        parse_builtin(BuiltInFunctionType::Require),
        parse_builtin(BuiltInFunctionType::MakeError),
        alt((
            parse_builtin(BuiltInFunctionType::IsU8),
            parse_builtin(BuiltInFunctionType::IsI8),
            parse_builtin(BuiltInFunctionType::IsInt),
            parse_builtin(BuiltInFunctionType::IsDouble),
            parse_builtin(BuiltInFunctionType::IsError),
            parse_builtin(BuiltInFunctionType::IsStruct),
            parse_builtin(BuiltInFunctionType::IsFunction),
            parse_builtin(BuiltInFunctionType::IsBool),
            parse_builtin(BuiltInFunctionType::IsArray),
        )),
        alt((
            parse_builtin(BuiltInFunctionType::ToHex),
            parse_builtin(BuiltInFunctionType::ToBinary),
            parse_builtin(BuiltInFunctionType::TypeOf),
            parse_builtin(BuiltInFunctionType::ToInt),
            parse_builtin(BuiltInFunctionType::ToDouble),
            parse_builtin(BuiltInFunctionType::ToBool),
            parse_builtin(BuiltInFunctionType::ToString),
        )),
        alt((
            parse_builtin(BuiltInFunctionType::Sqrt),
            parse_builtin(BuiltInFunctionType::Abs),
            parse_builtin(BuiltInFunctionType::Tan),
            parse_builtin(BuiltInFunctionType::Ln),
            parse_builtin(BuiltInFunctionType::Log),
            parse_builtin(BuiltInFunctionType::Sin),
            parse_builtin(BuiltInFunctionType::Cos),
            parse_builtin(BuiltInFunctionType::ToUpper),
            parse_builtin(BuiltInFunctionType::ToLower),
            parse_builtin(BuiltInFunctionType::Capitalize),
            parse_builtin(BuiltInFunctionType::Ceil),
            parse_builtin(BuiltInFunctionType::Floor),
        )),
        alt((
            parse_builtin(BuiltInFunctionType::Jsonify),
            parse_builtin(BuiltInFunctionType::ParseJson),
        )),
        alt((
            parse_builtin_many_args(BuiltInFunctionType::IsMatch),
            parse_builtin_many_args(BuiltInFunctionType::Match),
            parse_builtin_many_args(BuiltInFunctionType::Replace),
            parse_builtin_many_args(BuiltInFunctionType::Round),
            parse_builtin_many_args(BuiltInFunctionType::ReplaceAll),
        )), // parse_builtin(BuiltInFunctionType::ReadLines),
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
    use nom::character::complete::char as charParser;
    let parse_key = |p| {
        preceded(
            multispace0,
            terminated(
                map(
                    preceded(
                        opt(charParser('"')),
                        terminated(parse_key_struct, opt(charParser('"'))),
                    ),
                    String::from,
                ),
                multispace0,
            ),
        )(p)
    };
    let pair_key_value = move|p| {
        separated_pair(
            parse_key,
            tag_no_space(":"),
            preceded(opt(comments),parse_struct_expr))
    }(p);
    preceded(
        tag_no_space(STRUCT),
        map(
            delimited(
                tag_no_space("{"),
                many0(preceded(
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
                    alt((parse_fn, parse_range, parse_number, parse_value)),
                ),
                preceded(multispace0, tag_no_space("]")),
            ),
        ),
        Value::Array,
    )(s)
}

fn parse_key_brackets(s: &str) -> Res<KeyAccess> {
    map(
        pair(
            preceded(
                tag("["),
                terminated(
                    map(
                        delimited(tag("\""), parse_key_struct, tag("\"")),
                        |k| KeyAccess::Key(Primitive::String(k.to_string())),
                    ),
                    tag("]"),
                ),
            ),
            opt(parse_fn_args),
        ),
        |(k, params)| {
            if let Some(params) = params {
                KeyAccess::FunctionCall {
                    parameters: Value::BlockParen(params),
                    key: Box::new(k),
                }
            } else {
                k
            }
        },
    )(s)
}

fn parse_variable_brackets(s: &str) -> Res<KeyAccess> {
    map(
        pair(
            preceded(
                tag("["),
                terminated(
                    map(
                        verify(parse_variable, |v| {
                            matches!(
                                v,
                                Value::VariableRef(_) | Value::Variable(_)
                            )
                        }),
                        KeyAccess::Variable,
                    ),
                    tag("]"),
                ),
            ),
            opt(parse_fn_args),
        ),
        |(k, params)| {
            if let Some(params) = params {
                KeyAccess::FunctionCall {
                    parameters: Value::BlockParen(params),
                    key: Box::new(k),
                }
            } else {
                k
            }
        },
    )(s)
}
fn parse_index_brackets(s: &str) -> Res<KeyAccess> {
    map(
        pair(
            preceded(
                tag_no_space("["),
                terminated(
                    alt((
                        map_parser(
                            take_until("]"),
                            map(many1(parse_value), |mut v| {
                                if v.len() == 1 {
                                    v.remove(0)
                                } else {
                                    Value::BlockParen(v)
                                }
                            }),
                        ),
                        parse_variable,
                        parse_number,
                    )),
                    tag_no_space("]"),
                ),
            ),
            opt(parse_fn_args),
        ),
        |(k, params)| {
            if let Some(params) = params {
                KeyAccess::FunctionCall {
                    parameters: Value::BlockParen(params),
                    key: Box::new(KeyAccess::Variable(k)),
                }
            } else {
                KeyAccess::Variable(k)
            }
        },
    )(s)
}
fn parse_key_dots(s: &str) -> Res<KeyAccess> {
    map(
        pair(
            map(preceded(tag("."), parse_variable_str), |k| {
                KeyAccess::Key(Primitive::String(k.to_string()))
            }),
            opt(parse_fn_args),
        ),
        |(k, params)| {
            if let Some(params) = params {
                KeyAccess::FunctionCall {
                    parameters: Value::BlockParen(params),
                    key: Box::new(k),
                }
            } else {
                k
            }
        },
    )(s)
}

fn parse_multidepth_access(s: &str) -> Res<Value> {
    let (res, (root, mut next_keys)) = map(
        alt((
            pair(
                parse_builtin_fn,
                alt((
                    parse_key_brackets,
                    parse_index_brackets,
                    parse_variable_brackets,
                    parse_key_dots,
                )),
            ),
            pair(
                parse_fn_call,
                alt((
                    parse_key_brackets,
                    parse_index_brackets,
                    parse_variable_brackets,
                    parse_key_dots,
                )),
            ),
            pair(
                parse_variable,
                alt((
                    parse_key_brackets,
                    parse_index_brackets,
                    parse_variable_brackets,
                    parse_key_dots,
                )),
            ),
            pair(
                parse_struct,
                alt((
                    parse_key_brackets,
                    parse_index_brackets,
                    parse_variable_brackets,
                    parse_key_dots,
                )),
            ),
            pair(
                alt((parse_array, parse_fstring, parse_string)),
                alt((parse_variable_brackets, parse_index_brackets)),
            ),
        )),
        |(s, key)| (Box::new(s), vec![key]),
    )(s)?;

    let mut new_rest = res;

    while let Ok((rest, key)) = alt((
        parse_key_brackets,
        parse_index_brackets,
        parse_variable_brackets,
        parse_key_dots,
    ))(new_rest)
    {
        next_keys.push(key);
        new_rest = rest;
    }

    Ok((new_rest, Value::MultiDepthAccess { root, next_keys }))
}

fn parse_implicit_multiply(s: &str) -> Res<Value> {
    map(pair(parse_number, peek(parse_variable_str)), |(multiplier, _)| {
        Value::ImplicitMultiply(Box::new(multiplier))
    })(s)
}

fn parse_value(s: &str) -> Res<Value> {
    preceded(
        opt(comments),
        terminated(
            alt((
                parse_block_paren,
                parse_operation,
                parse_implicit_multiply,
                parse_multidepth_access,
                parse_struct,
                parse_builtin_fn,
                parse_fn_call,
                parse_array,
                parse_range,
                parse_number,
                parse_fn,
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
        parse_op(Operator::Pow2),
        parse_op(Operator::Pow3),
        parse_op(Operator::Mult),
        parse_op(Operator::Mod),
        parse_op(Operator::Div),
        parse_op(Operator::Add),
        parse_op(Operator::Subtr),
        parse_op(Operator::LessOrEqual),
        parse_op(Operator::GreaterOrEqual),
        parse_op(Operator::BitwiseLShift),
        parse_op(Operator::BitwiseRShift),
        parse_op(Operator::Less),
        parse_op(Operator::Greater),
        parse_op(Operator::Equal),
        parse_op(Operator::NotEqual),
        parse_op(Operator::Not),
        parse_op(Operator::And),
        parse_op(Operator::Or),
        parse_op(Operator::Or),
        alt((
            parse_op(Operator::BitwiseNot),
            parse_op(Operator::BitwiseAnd),
            parse_op(Operator::BitwiseOr),
            parse_op(Operator::BitwiseXor),
        )),
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
                pair(
                    alt((parse_multidepth_access, parse_variable)),
                    opt(parse_operation),
                ),
                tag_no_space("="),
                parse_complex_expression,
            ),
            |((variable, mut operator), expr)| {
                if let Some(operator) = operator.take() {
                    Value::VariableExpr {
                        name: Box::new(variable.clone()),
                        expr: Box::new(Value::Expression(vec![
                            variable, operator, expr,
                        ])),
                    }
                } else {
                    Value::VariableExpr {
                        name: Box::new(variable),
                        expr: Box::new(expr),
                    }
                }
            },
        ),
        parse_complex_expression,
    ))(s)
}

fn parse_if_statement(s: &str) -> Res<Value> {
    map(
        preceded(
            tag_no_space(IF),
            tuple((
                parse_block_paren_opt,
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
            preceded(
                tag_no_space("{"),
                terminated(take_until("}"), tag_no_space("}")),
            ),
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
            pair(parse_block_paren_opt, parse_block(parse_instructions)),
        ),
        |(cond, exprs)| Value::WhileExpr { cond: Box::new(cond), exprs },
    )(s)
}

fn parse_block<F>(parser: F) -> impl Fn(&str) -> Res<Vec<Value>>
where
    F: Fn(&str) -> Res<Vec<Value>>,
{
    move |s| {
        map(
            preceded(
                tag_no_space("{"),
                terminated(opt(&parser), tag_no_space("}")),
            ),
            |exprs| exprs.unwrap_or_default(),
        )(s)
    }
}

fn parse_break(s: &str) -> Res<Value> {
    map(tag_no_space(BREAK), |_| Value::Break)(s)
}
// CONTEXT: this should maybe replace parse_expression
fn parse_complex_expression(s: &str) -> Res<Value> {
    alt((
        all_consuming(parse_multidepth_access),
        all_consuming(parse_builtin_fn),
        all_consuming(parse_fn_call),
        // TODO maybe with tuple() this giant mess can be simplified e.g
        // tuple(alt(parser1, parser2,...))
        map(
            tuple((
                alt((parse_multidepth_access, parse_fn_call)),
                parse_operation,
                parse_expression,
            )),
            |(k, o, v)| Value::Expression(vec![k, o, v]),
        ),
        parse_fn,
        parse_struct,
        parse_fstring,
        parse_array,
        parse_expression,
    ))(s)
}

fn parse_early_return(s: &str) -> Res<Value> {
    map(preceded(tag_no_space(RETURN), opt(parse_complex_expression)), |v| {
        Value::EarlyReturn(Box::new(v))
    })(s)
}

pub fn parse_instructions(instructions: &str) -> Res<Vec<Value>> {
    let (instructions, _) = opt(comments)(instructions)?;

    let instructions = instructions.trim();
    if instructions.is_empty() {
        return Ok((instructions, vec![Value::NoOp]));
    }

    terminated(
        many1(preceded(
            opt(comments),
            alt((
                all_consuming(parse_value),
                parse_foreach,
                parse_while_statement,
                parse_if_statement,
                parse_simple_instruction,
                parse_drop,
                parse_early_return,
                parse_break,
            )),
        )),
        opt(comments),
    )(instructions)
}
fn default_check_reserved_keywords(aliases: &[&str]) -> bool {
    FORBIDDEN_VARIABLE_NAME
        .iter()
        .any(|c| aliases.iter().any(|al| al.eq_ignore_ascii_case(c)))
}
