use std::borrow::Borrow;

use slab_tree::{NodeId, Tree};

use crate::prelude::{BTreeMap, Context};

use adana_script_core::{
    primitive::{Neg, Primitive, RefPrimitive, ToNumber},
    MathConstants, Operator, TreeNodeValue, Value,
};

fn variable_from_ctx(
    name: &str,
    negate: bool,
    ctx: &mut BTreeMap<String, RefPrimitive>,
) -> anyhow::Result<Primitive> {
    let value = ctx
        .get(name)
        .cloned()
        .or_else(|| {
            Some(
                Primitive::Error(format!("variable {name} not found in ctx"))
                    .ref_prim(),
            )
        })
        .context(format!("variable {name} not found in ctx"))?;

    if cfg!(test) {
        dbg!(&value);
    }
    let guard = value
        .read()
        .map_err(|e| anyhow::format_err!("could not acquire lock {e}"))?;
    let primitive = if negate { guard.neg() } else { guard.clone() };

    Ok(primitive)
}

fn filter_op(
    op: Operator,
    operations: &[Value],
) -> impl FnOnce() -> Option<usize> + '_ {
    move || {
        operations.iter().rposition(
            |c| matches!(c, Value::Operation(operator) if operator == &op),
        )
    }
}

pub fn to_ast(
    ctx: &mut BTreeMap<String, RefPrimitive>,
    value: Value,
    tree: &mut Tree<TreeNodeValue>,
    curr_node_id: &Option<NodeId>,
) -> anyhow::Result<Option<NodeId>> {
    fn append_to_current_and_return(
        v: TreeNodeValue,
        tree: &mut Tree<TreeNodeValue>,
        curr_node_id: &Option<NodeId>,
    ) -> anyhow::Result<Option<NodeId>> {
        if let Some(node_id) = curr_node_id {
            let mut node =
                tree.get_mut(*node_id).context("node id does not exist!")?;
            node.append(v);
            Ok(Some(node.node_id()))
        } else if let Some(mut root_node) = tree.root_mut() {
            root_node.append(v);
            Ok(tree.root_id())
        } else {
            Ok(Some(tree.set_root(v)))
        }
    }

    match value {
        Value::Expression(mut operations)
        | Value::BlockParen(mut operations) => {
            if cfg!(test) {
                dbg!(&operations);
            }

            if operations.is_empty() {
                return Ok(None);
            }
            if operations.len() == 1 {
                return to_ast(ctx, operations.remove(0), tree, curr_node_id);
            }

            // handle implicit multiply and pow2,pow3. e.g 2x²
            while let Some(pos) = operations.iter().position(|o| {
                matches!(
                    o,
                    Value::ImplicitMultiply(_)
                        | Value::Operation(Operator::Pow2)
                        | Value::Operation(Operator::Pow3)
                )
            }) {
                let operation = operations.remove(pos);
                match operation {
                    Value::ImplicitMultiply(v)
                        if matches!(
                            v.borrow(),
                            &Value::Integer(_) | &Value::Decimal(_) | &Value::U8(_) | &Value::I8(_)
                        ) =>
                    {
                        operations.insert(pos,*v);
                        operations.insert(pos+1, Value::Operation(Operator::Mult));
                    }
                    Value::Operation(Operator::Pow2) => {
                        operations.insert(pos,Value::Operation(Operator::Pow));
                        operations.insert(pos+1,Value::U8(2));
                    }
                    Value::Operation(Operator::Pow3) => {
                        operations.insert(pos,Value::Operation(Operator::Pow));
                        operations.insert(pos+1,Value::U8(3));
                    }
                    _ => unreachable!("AST ERROR: unreachable implicit parameter {operation:?}"),
                }
            }

            let get_next_op_pos = |operations: &Vec<Value>| {
                None.or_else(filter_op(Operator::Or, operations))
                    .or_else(filter_op(Operator::And, operations))
                    .or_else(filter_op(Operator::BitwiseOr, operations))
                    .or_else(filter_op(Operator::BitwiseXor, operations))
                    .or_else(filter_op(Operator::BitwiseAnd, operations))
                    .or_else(filter_op(Operator::GreaterOrEqual, operations))
                    .or_else(filter_op(Operator::LessOrEqual, operations))
                    .or_else(filter_op(Operator::Greater, operations))
                    .or_else(filter_op(Operator::Less, operations))
                    .or_else(filter_op(Operator::Equal, operations))
                    .or_else(filter_op(Operator::NotEqual, operations))
                    .or_else(filter_op(Operator::BitwiseLShift, operations))
                    .or_else(filter_op(Operator::BitwiseRShift, operations))
                    .or_else(filter_op(Operator::Add, operations))
                    .or_else(filter_op(Operator::Subtr, operations))
                    .or_else(filter_op(Operator::Mult, operations))
                    .or_else(filter_op(Operator::Mod, operations))
                    .or_else(filter_op(Operator::Div, operations))
                    .or_else(filter_op(Operator::Pow, operations))
                    .or_else(filter_op(Operator::Not, operations))
                    .or_else(filter_op(Operator::BitwiseNot, operations))
            };

            let op_pos = get_next_op_pos(&operations);

            if let Some(op_pos) = op_pos {
                let mut left: Vec<Value> =
                    operations.drain(0..op_pos).collect();

                let operation = operations.remove(0);

                // handle negation
                if operation == Value::Operation(Operator::Subtr)
                   // || operation == Value::Operation(Operator::BitwiseNot) maybe needed
                    // but maybe not
                        && matches!(
                            left.last(),
                            Some(
                                Value::Operation(Operator::Subtr)
                                | Value::Operation(Operator::Mult)
                                | Value::Operation(Operator::Pow)
                                | Value::Operation(Operator::Add) // FIXME too tired to think about
                                                                  // it. Is it needed?
                                | Value::Operation(Operator::Mod)
                                | Value::Operation(Operator::Div)
                            )
                        )
                {
                    let right_first = match (&operation, operations.first()) {
                        (
                            Value::Operation(Operator::Subtr),
                            Some(Value::Decimal(d)),
                        ) => Some(Value::Decimal(-d)),
                        (
                            Value::Operation(Operator::Subtr),
                            Some(Value::Integer(d)),
                        ) => Some(Value::Integer(-d)),
                        (
                            Value::Operation(Operator::Subtr),
                            Some(Value::U8(d)),
                        ) => Some(Value::I8(-(*d as i8))),
                        (
                            Value::Operation(Operator::Subtr),
                            Some(Value::I8(d)),
                        ) => Some(Value::I8(-d)),
                        (
                            Value::Operation(Operator::Subtr),
                            Some(Value::Variable(d)),
                        ) => Some(Value::VariableNegate(d.to_string())),
                        _ => None,
                    };
                    if let Some(first) = right_first {
                        operations[0] = first; // override one of the negate operator by the
                                               // negated value
                        left.append(&mut operations);
                        return to_ast(
                            ctx,
                            Value::BlockParen(left),
                            tree,
                            curr_node_id,
                        );
                    }
                }

                if cfg!(test) {
                    println!("Left => {left:?}");
                    println!("Op => {operation:?}");
                    println!("Right => {operations:?}");
                    println!();
                }

                let curr_node_id = to_ast(ctx, operation, tree, curr_node_id)?;

                to_ast(ctx, Value::BlockParen(left), tree, &curr_node_id)?;
                to_ast(
                    ctx,
                    if operations.len() == 1 {
                        operations.remove(0)
                    } else {
                        Value::BlockParen(operations)
                    },
                    tree,
                    &curr_node_id,
                )?;

                Ok(curr_node_id)
            } else {
                Err(anyhow::Error::msg(format!(
                    "{} invalid expression! {op_pos:?}",
                    nu_ansi_term::Color::Red.paint("AST ERROR:")
                )))
            }
        }

        Value::Operation(operator) => {
            let ops = TreeNodeValue::Ops(operator);
            if let Some(node_id) = curr_node_id {
                let mut node = tree
                    .get_mut(*node_id)
                    .context("node id does not exist!")?;

                let node = node.append(ops);
                Ok(Some(node.node_id()))
            } else if let Some(mut root_node) = tree.root_mut() {
                let node = root_node.append(ops);
                Ok(Some(node.node_id()))
            } else {
                let _ = tree.set_root(ops);
                Ok(tree.root_id())
            }
        }
        Value::NoOp => append_to_current_and_return(
            TreeNodeValue::Primitive(Primitive::Unit),
            tree,
            curr_node_id,
        ),
        Value::ImplicitMultiply(value) => Err(anyhow::Error::msg(format!(
            "{} invalid implicit multiplier, unreachable branch: {value:?}",
            nu_ansi_term::Color::Red.paint("AST BUG:"),
        ))),

        Value::Decimal(num) => append_to_current_and_return(
            TreeNodeValue::Primitive(Primitive::Double(num)),
            tree,
            curr_node_id,
        ),
        Value::Integer(num) => append_to_current_and_return(
            TreeNodeValue::Primitive(Primitive::Int(num)),
            tree,
            curr_node_id,
        ),
        Value::U8(num) => append_to_current_and_return(
            TreeNodeValue::Primitive(Primitive::U8(num)),
            tree,
            curr_node_id,
        ),
        Value::I8(num) => append_to_current_and_return(
            TreeNodeValue::Primitive(Primitive::I8(num)),
            tree,
            curr_node_id,
        ),
        Value::Bool(bool_v) => append_to_current_and_return(
            TreeNodeValue::Primitive(Primitive::Bool(bool_v)),
            tree,
            curr_node_id,
        ),
        Value::String(string_v) => append_to_current_and_return(
            TreeNodeValue::Primitive(Primitive::String(string_v)),
            tree,
            curr_node_id,
        ),
        Value::FString(string_v, para) => append_to_current_and_return(
            TreeNodeValue::FString(string_v, para),
            tree,
            curr_node_id,
        ),
        Value::Range { start, end, incl_both_end } => {
            let start = match *start {
                Value::Variable(name) | Value::VariableRef(name) => {
                    let primitive =
                        variable_from_ctx(name.as_str(), false, ctx)?;
                    match primitive.to_int() {
                        Primitive::Int(num) => Ok(num),
                        Primitive::U8(num) => Ok(num as i128),
                        Primitive::I8(num) => Ok(num as i128),
                        _ => Err(anyhow::format_err!(
                            "range error: {primitive:?} is not an integer"
                        )),
                    }
                }

                Value::Integer(num) => Ok(num),
                Value::U8(num) => Ok(num as i128),
                Value::I8(num) => Ok(num as i128),
                _ => {
                    return Err(anyhow::format_err!(
                        "range error: {start:?} is not an integer"
                    ))
                }
            }?;
            let end = match *end {
                Value::Variable(name) | Value::VariableRef(name) => {
                    let primitive =
                        variable_from_ctx(name.as_str(), false, ctx)?;
                    match primitive.to_int() {
                        Primitive::Int(num) => Ok(num),
                        Primitive::U8(num) => Ok(num as i128),
                        Primitive::I8(num) => Ok(num as i128),
                        _ => Err(anyhow::format_err!(
                            "range error: {primitive:?} is not an integer"
                        )),
                    }
                }
                Value::U8(num) => Ok(num as i128),
                Value::I8(num) => Ok(num as i128),
                Value::Integer(num) => Ok(num),
                _ => {
                    return Err(anyhow::format_err!(
                        "range error: {end:?} is not an integer"
                    ))
                }
            }?;
            let end = if incl_both_end { end + 1 } else { end };
            let range: Vec<Primitive> =
                (start..end).map(Primitive::Int).collect();
            append_to_current_and_return(
                TreeNodeValue::Primitive(Primitive::Array(range)),
                tree,
                curr_node_id,
            )
        }
        Value::Variable(name) => {
            let value = variable_from_ctx(name.as_str(), false, ctx)?;
            append_to_current_and_return(
                TreeNodeValue::Primitive(value),
                tree,
                curr_node_id,
            )
        }
        Value::VariableRef(name) => append_to_current_and_return(
            TreeNodeValue::VariableRef(name),
            tree,
            curr_node_id,
        ),
        Value::VariableUnused => append_to_current_and_return(
            TreeNodeValue::VariableUnused,
            tree,
            curr_node_id,
        ),
        Value::VariableNegate(name) => {
            let value = variable_from_ctx(name.as_str(), true, ctx)?;
            append_to_current_and_return(
                TreeNodeValue::Primitive(value),
                tree,
                curr_node_id,
            )
        }
        Value::VariableExpr { name, expr } => {
            anyhow::ensure!(
                tree.root().is_none(),
                "invalid variable assignment, tree root is not none"
            );
            let variable_assign_node = if let Value::Variable(n) = *name {
                Ok(TreeNodeValue::VariableAssign(Some(n)))
            } else if let Value::VariableUnused = *name {
                Ok(TreeNodeValue::VariableAssign(None))
            } else if let Value::ArrayAccess { arr, index } = *name {
                // let index = match *index {
                //     Value::Integer(n) => Ok(Primitive::Int(n)),
                //     Value::Variable(v) => variable_from_ctx(&v, false, ctx),
                //     v => {
                //         Err(anyhow::Error::msg(format!("invalid index {v:?}")))
                //     }
                // }?;

                if let Value::Variable(n) = *arr {
                    Ok(TreeNodeValue::VariableArrayAssign {
                        name: n,
                        index: *index,
                    })
                } else {
                    Err(anyhow::Error::msg(format!(
                        "invalid variable expression {arr:?} => {expr:?}"
                    )))
                }
            } else if let Value::StructAccess { struc, key } = *name {
                if let Value::Variable(n) = *struc {
                    Ok(TreeNodeValue::VariableArrayAssign {
                        name: n,
                        index: Value::String(key),
                    })
                } else {
                    Err(anyhow::Error::msg(format!(
                        "invalid variable expression {struc:?} => {expr:?}"
                    )))
                }
            } else {
                // FIXME for my future self. x.y.z or x[0][1] is not yet supported
                // for assignment
                // We need Primitive::Ref to make it happen
                Err(anyhow::Error::msg(format!(
                    "{} invalid variable expression {name:?} => {expr:?}",
                    nu_ansi_term::Color::Red.paint("AST ERROR:"),
                )))
            }?;

            let node_id = Some(tree.set_root(variable_assign_node));

            let value = *expr;

            let _ = to_ast(ctx, value, tree, &node_id)?
                .context(format!("invalid variable expr {node_id:?}"))?;
            Ok(node_id)
        }
        Value::Const(c) => match c {
            c if c == MathConstants::Pi.get_symbol() => to_ast(
                ctx,
                Value::Decimal(std::f64::consts::PI),
                tree,
                curr_node_id,
            ),
            c if c == MathConstants::EulerNumber.get_symbol() => to_ast(
                ctx,
                Value::Decimal(std::f64::consts::E),
                tree,
                curr_node_id,
            ),
            c if c == MathConstants::Tau.get_symbol() => to_ast(
                ctx,
                Value::Decimal(std::f64::consts::TAU),
                tree,
                curr_node_id,
            ),
            _ => unreachable!("should never happen or it's a bug"),
        },
        Value::BuiltInFunction { fn_type, expr } => {
            let fn_node = TreeNodeValue::BuiltInFunction(fn_type);
            let node_id = if let Some(node_id) = curr_node_id {
                let mut node = tree
                    .get_mut(*node_id)
                    .context("node id does not exist!")?;

                let node = node.append(fn_node);
                Some(node.node_id())
            } else if let Some(mut root_node) = tree.root_mut() {
                let node = root_node.append(fn_node);
                Some(node.node_id())
            } else {
                Some(tree.set_root(fn_node))
            };
            to_ast(ctx, *expr, tree, &node_id)?;
            Ok(node_id)
        }
        v @ Value::IfExpr { cond: _, exprs: _, else_expr: _ } => {
            let if_node = TreeNodeValue::IfExpr(v);
            append_to_current_and_return(if_node, tree, curr_node_id)
        }
        v @ Value::WhileExpr { cond: _, exprs: _ } => {
            let while_node = TreeNodeValue::WhileExpr(v);
            append_to_current_and_return(while_node, tree, curr_node_id)
        }
        v @ Value::ForeachExpr {
            var: _,
            index_var: _,
            iterator: _,
            exprs: _,
        } => {
            let foreach_node = TreeNodeValue::Foreach(v);
            append_to_current_and_return(foreach_node, tree, curr_node_id)
        }
        Value::Array(arr) => append_to_current_and_return(
            TreeNodeValue::Array(arr),
            tree,
            curr_node_id,
        ),
        Value::Struct(struc_map) => append_to_current_and_return(
            TreeNodeValue::Struct(struc_map),
            tree,
            curr_node_id,
        ),
        Value::ArrayAccess { arr, index } => match (*arr, *index) {
            (v, index @ Value::Integer(_)) => append_to_current_and_return(
                TreeNodeValue::ArrayAccess { index, array: v },
                tree,
                curr_node_id,
            ),
            (v, index @ Value::U8(_)) => append_to_current_and_return(
                TreeNodeValue::ArrayAccess { index, array: v },
                tree,
                curr_node_id,
            ),
            (v, index @ Value::I8(_)) => append_to_current_and_return(
                TreeNodeValue::ArrayAccess { index, array: v },
                tree,
                curr_node_id,
            ),
            (v, variable @ Value::Variable(_)) => append_to_current_and_return(
                TreeNodeValue::ArrayAccess { index: variable, array: v },
                tree,
                curr_node_id,
            ),
            (v, variable @ Value::String(_)) => append_to_current_and_return(
                TreeNodeValue::ArrayAccess { index: variable, array: v },
                tree,
                curr_node_id,
            ),
            (v, variable @ Value::BlockParen(_)) => {
                append_to_current_and_return(
                    TreeNodeValue::ArrayAccess { index: variable, array: v },
                    tree,
                    curr_node_id,
                )
            }

            (arr, index) => Err(anyhow::Error::msg(format!(
                "illegal array access! array => {arr:?}, index=> {index:?}"
            ))),
        },
        Value::StructAccess { struc, key } => append_to_current_and_return(
            TreeNodeValue::StructAccess {
                struc: *struc,
                key: Primitive::String(key),
            },
            tree,
            curr_node_id,
        ),
        f @ Value::Function { parameters: _, exprs: _ } => {
            append_to_current_and_return(
                TreeNodeValue::Function(f),
                tree,
                curr_node_id,
            )
        }
        fc @ Value::FunctionCall { parameters: _, function: _ } => {
            append_to_current_and_return(
                TreeNodeValue::FunctionCall(fc),
                tree,
                curr_node_id,
            )
        }
        Value::Break => append_to_current_and_return(
            TreeNodeValue::Break,
            tree,
            curr_node_id,
        ),
        Value::Null => append_to_current_and_return(
            TreeNodeValue::Null,
            tree,
            curr_node_id,
        ),
        Value::Drop(v) => {
            if let Value::BlockParen(variables) = *v {
                let mut vars = Vec::with_capacity(variables.len());
                for variable in variables {
                    match variable {
                        v @ Value::Variable(_)
                        | v @ Value::ArrayAccess { arr: _, index: _ }
                        | v @ Value::StructAccess { struc: _, key: _ } => {
                            vars.push(v)
                        }
                        _ => {
                            return Err(anyhow::Error::msg(format!(
                                "not a variable: {variable:?}"
                            )));
                        }
                    }
                }
                append_to_current_and_return(
                    TreeNodeValue::Drop(vars),
                    tree,
                    curr_node_id,
                )
            } else {
                Err(anyhow::Error::msg(format!("drop must be in paren: {v:?}")))
            }
        }
        Value::EarlyReturn(v) => append_to_current_and_return(
            TreeNodeValue::EarlyReturn(*v),
            tree,
            curr_node_id,
        ),
    }
}
