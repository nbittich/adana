use slab_tree::{NodeId, Tree};

use crate::prelude::{BTreeMap, Context};

use super::{MathConstants, Operator, Primitive, TreeNodeValue, Value};

fn arr_values_to_arr_primitive(arr: Vec<Value>) -> anyhow::Result<Primitive> {
    let mut primitives = vec![];
    for v in arr {
        match v {
            Value::Integer(i) => primitives.push(Primitive::Int(i)),
            Value::Bool(b) => primitives.push(Primitive::Bool(b)),
            Value::Decimal(d) => primitives.push(Primitive::Double(d)),
            Value::String(s) => primitives.push(Primitive::String(s)),
            Value::Array(arr) => {
                primitives.push(arr_values_to_arr_primitive(arr)?)
            }
            _ => {
                return Err(anyhow::Error::msg(
                    "invalid conversion for array! ",
                ))
            }
        }
    }
    Ok(Primitive::Array(primitives))
}
fn primitive_to_value(p: &Primitive, negate: bool) -> anyhow::Result<Value> {
    match p {
        Primitive::Int(i) if negate => Ok(Value::Integer(-i)),
        Primitive::Int(i) => Ok(Value::Integer(*i)),
        Primitive::Double(d) if negate => Ok(Value::Decimal(-d)),
        Primitive::Double(d) => Ok(Value::Decimal(*d)),
        Primitive::Bool(b) if !negate => Ok(Value::Bool(*b)),
        Primitive::String(s) if !negate => Ok(Value::String(s.to_string())),
        Primitive::Array(arr) if !negate => {
            let values = arr
                .iter()
                .map(|p| primitive_to_value(p, false))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Value::Array(values))
        }
        Primitive::Array(_) => {
            Err(anyhow::Error::msg("attempt to negate an array value from ctx"))
        }
        Primitive::Unit => {
            Err(anyhow::Error::msg("attempt to get an unit value from ctx"))
        }
        Primitive::Bool(_) | Primitive::String(_) => Err(anyhow::Error::msg(
            "attempt to negate a bool or string or unit value",
        )),
        Primitive::Error(msg) => Err(anyhow::Error::msg(*msg)),
    }
}

fn variable_from_ctx(
    name: &str,
    negate: bool,
    ctx: &mut BTreeMap<String, Primitive>,
) -> anyhow::Result<Value> {
    let value = ctx
        .get(name)
        .context(format!("variable {name} not found in ctx"))?
        .as_ref_ok()?;

    if cfg!(test) {
        dbg!(value);
    }

    primitive_to_value(value, negate)
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

pub(super) fn to_ast(
    ctx: &mut BTreeMap<String, Primitive>,
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

            let op_pos = None
                .or_else(filter_op(Operator::Or, &operations))
                .or_else(filter_op(Operator::And, &operations))
                .or_else(filter_op(Operator::GreaterOrEqual, &operations))
                .or_else(filter_op(Operator::LessOrEqual, &operations))
                .or_else(filter_op(Operator::Greater, &operations))
                .or_else(filter_op(Operator::Less, &operations))
                .or_else(filter_op(Operator::Equal, &operations))
                .or_else(filter_op(Operator::NotEqual, &operations))
                .or_else(filter_op(Operator::Add, &operations))
                .or_else(filter_op(Operator::Subtr, &operations))
                .or_else(filter_op(Operator::Mult, &operations))
                .or_else(filter_op(Operator::Mod, &operations))
                .or_else(filter_op(Operator::Div, &operations))
                .or_else(filter_op(Operator::Pow, &operations))
                .or_else(filter_op(Operator::Not, &operations));

            if let Some(op_pos) = op_pos {
                let mut left: Vec<Value> =
                    operations.drain(0..op_pos).collect();

                let operation = operations.remove(0);

                // handle negation
                if operation == Value::Operation(Operator::Subtr)
                    && matches!(left.last(), Some(Value::Operation(_)))
                {
                    let right_first = match operations.first() {
                        Some(Value::Decimal(d)) => Some(Value::Decimal(-d)),
                        Some(Value::Integer(d)) => Some(Value::Integer(-d)),
                        Some(Value::Variable(d)) => {
                            Some(Value::VariableNegate(d.to_string()))
                        }
                        _ => None,
                    };
                    if let Some(first) = right_first {
                        operations[0] = first;
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
                    println!("Right => {operation:?}");
                    println!("Op => {operation:?}");
                    println!();
                }

                let curr_node_id = to_ast(ctx, operation, tree, curr_node_id)?;

                to_ast(ctx, Value::BlockParen(left), tree, &curr_node_id)?;
                to_ast(
                    ctx,
                    Value::BlockParen(operations),
                    tree,
                    &curr_node_id,
                )?;

                Ok(curr_node_id)
            } else {
                Err(anyhow::Error::msg("invalid expression!"))
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
        Value::Variable(name) => {
            let value = variable_from_ctx(name.as_str(), false, ctx)?;
            to_ast(ctx, value, tree, curr_node_id)
        }
        Value::VariableNegate(name) => {
            let value = variable_from_ctx(name.as_str(), true, ctx)?;
            to_ast(ctx, value, tree, curr_node_id)
        }
        Value::VariableExpr { name, expr } => {
            anyhow::ensure!(
                tree.root().is_none(),
                "invalid variable assignment "
            );

            if let Value::Variable(n) = *name {
                let variable_assign_node =
                    TreeNodeValue::VariableAssign(n.to_string());

                let node_id = Some(tree.set_root(variable_assign_node));

                let value = *expr;

                let _ = to_ast(ctx, value, tree, &node_id)?
                    .context(format!("invalid variable expr {n}"))?;

                Ok(node_id)
            } else {
                Err(anyhow::Error::msg("invalid variable expression"))
            }
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
        Value::Array(arr) => append_to_current_and_return(
            TreeNodeValue::Primitive(arr_values_to_arr_primitive(arr)?),
            tree,
            curr_node_id,
        ),
    }
}
