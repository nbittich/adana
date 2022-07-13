use slab_tree::{NodeId, Tree};

use crate::prelude::{BTreeMap, Context};

use super::{Number, Operator, TreeNodeValue, Value, EULER_NUMBER, PI};

fn variable_from_ctx<'a>(
    name: &'a str,
    negate: bool,
    ctx: &mut BTreeMap<String, Number>,
) -> anyhow::Result<Value<'a>> {
    let value =
        ctx.get(name).context(format!("variable {name} not found in ctx"))?;

    if cfg!(test) {
        dbg!(value);
    }

    let value = match value {
        Number::Int(i) if negate => Value::Integer(-i),
        Number::Int(i) => Value::Integer(*i),
        Number::Double(d) if negate => Value::Decimal(-d),
        Number::Double(d) => Value::Decimal(*d),
    };
    Ok(value)
}

fn filter_op<'a>(
    op: Operator,
    operations: &'a [Value<'a>],
) -> impl FnOnce() -> Option<usize> + 'a {
    move || {
        operations.iter().rposition(
            |c| matches!(c, Value::Operation(operator) if operator == &op),
        )
    }
}

pub(super) fn to_ast(
    ctx: &mut BTreeMap<String, Number>,
    value: Value,
    tree: &mut Tree<TreeNodeValue>,
    curr_node_id: &Option<NodeId>,
) -> anyhow::Result<Option<NodeId>> {
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
                .or_else(filter_op(Operator::Add, &operations))
                .or_else(filter_op(Operator::Subtr, &operations))
                .or_else(filter_op(Operator::Mult, &operations))
                .or_else(filter_op(Operator::Mod, &operations))
                .or_else(filter_op(Operator::Div, &operations))
                .or_else(filter_op(Operator::Pow, &operations));

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
                            Some(Value::VariableNegate(d))
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

        Value::Decimal(num) => {
            let double_node = TreeNodeValue::Primitive(Number::Double(num));
            if let Some(node_id) = curr_node_id {
                let mut node = tree
                    .get_mut(*node_id)
                    .context("node id does not exist!")?;
                node.append(double_node);
                Ok(Some(node.node_id()))
            } else if let Some(mut root_node) = tree.root_mut() {
                root_node.append(double_node);
                Ok(tree.root_id())
            } else {
                Ok(Some(tree.set_root(double_node)))
            }
        }
        Value::Integer(num) => {
            let double_node = TreeNodeValue::Primitive(Number::Int(num));
            let node_id = if let Some(node_id) = curr_node_id {
                let mut node = tree
                    .get_mut(*node_id)
                    .context("node id does not exist!")?;
                node.append(double_node);
                Some(node.node_id())
            } else if let Some(mut root_node) = tree.root_mut() {
                root_node.append(double_node);
                tree.root_id()
            } else {
                Some(tree.set_root(double_node))
            };
            Ok(node_id)
        }
        Value::Variable(name) => {
            let value = variable_from_ctx(name, false, ctx)?;
            to_ast(ctx, value, tree, curr_node_id)
        }
        Value::VariableNegate(name) => {
            let value = variable_from_ctx(name, true, ctx)?;
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
            c if c == PI => to_ast(
                ctx,
                Value::Decimal(std::f64::consts::PI),
                tree,
                curr_node_id,
            ),
            c if c == EULER_NUMBER => to_ast(
                ctx,
                Value::Decimal(std::f64::consts::E),
                tree,
                curr_node_id,
            ),
            _ => unreachable!("should never happen or it's a bug"),
        },
        Value::Function { fn_type, expr } => {
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
    }
}
