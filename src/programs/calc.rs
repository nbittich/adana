use std::{collections::HashMap, panic::AssertUnwindSafe};

use anyhow::Context;
use nom::{
    character::complete::{alpha1, alphanumeric1, i64 as I64},
    combinator::{all_consuming, map_parser},
    multi::many1,
    number::complete::{double, recognize_float},
};
use serde::{Deserialize, Serialize};
use slab_tree::{NodeId, NodeRef, Tree};

use crate::prelude::*;

// region: structs
#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum Value<'a> {
    Expression(Vec<Value<'a>>),
    Operation(Operator),
    Decimal(f64),
    Integer(i64),
    BlockParen(Vec<Value<'a>>),
    Variable(&'a str),
    VariableExpr { name: Box<Value<'a>>, expr: Box<Value<'a>> },
}

#[derive(Debug, Eq, Copy, Clone, PartialEq, Serialize, Deserialize)]
enum Operator {
    Add,
    Subtr,
    Mult,
    Div,
    Exp,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
enum TreeNodeValue {
    VariableAssign(String),
    Ops(Operator),
    Int(i64),
    Double(f64),
}
// endregion: structs

// region: parsers
fn tag_no_space<'a>(t: &'a str) -> impl Fn(&'a str) -> Res<&'a str> {
    move |s: &str| delimited(multispace0, tag(t), multispace0)(s)
}

fn parse_number(s: &str) -> Res<Value> {
    map_parser(
        recognize_float,
        alt((
            map(all_consuming(I64), Value::Integer),
            map(all_consuming(double), Value::Decimal),
        )),
    )(s)
}

fn parse_variable(s: &str) -> Res<Value> {
    map_parser(alpha1, map(all_consuming(alphanumeric1), Value::Variable))(s)
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

fn parse_value(s: &str) -> Res<Value> {
    preceded(
        multispace0,
        terminated(
            alt((
                parse_paren,
                parse_exp,
                parse_mult,
                parse_div,
                parse_add,
                parse_subtr,
                parse_number,
                parse_variable,
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
        Operator::Exp => "^",
    };
    move |s| map(tag_no_space(sep), |_| Value::Operation(operation))(s)
}

fn parse_exp(s: &str) -> Res<Value> {
    parse_op(Operator::Exp)(s)
}

fn parse_mult(s: &str) -> Res<Value> {
    parse_op(Operator::Mult)(s)
}

fn parse_div(s: &str) -> Res<Value> {
    parse_op(Operator::Div)(s)
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

fn parse_str(s: &str) -> Res<Value> {
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

// region: reducers

fn to_tree(
    ctx: &mut HashMap<String, f64>,
    value: Value,
    tree: &mut Tree<TreeNodeValue>,
    curr_node_id: &Option<NodeId>,
) -> anyhow::Result<Option<NodeId>> {
    match value {
        Value::Expression(mut operations)
        | Value::BlockParen(mut operations) => {
            fn filter_op<'a>(
                op: Operator,
                operations: &'a [Value<'a>],
            ) -> impl FnOnce() -> Option<usize> + 'a {
                move || {
                    operations.iter().rposition(|c| matches!(c, Value::Operation(operator) if operator == &op))
                }
            }

            if operations.is_empty() {
                return Ok(None);
            }
            if operations.len() == 1 {
                return to_tree(ctx, operations.remove(0), tree, curr_node_id);
            }

            let op_pos = None
                .or_else(filter_op(Operator::Add, &operations))
                .or_else(filter_op(Operator::Subtr, &operations))
                .or_else(filter_op(Operator::Mult, &operations))
                .or_else(filter_op(Operator::Div, &operations))
                .or_else(filter_op(Operator::Exp, &operations));

            if let Some(op_pos) = op_pos {
                let mut left: Vec<Value> =
                    operations.drain(0..op_pos).collect();

                let operation = operations.remove(0);

                let children_left = if left.len() == 1 {
                    left.remove(0)
                } else {
                    Value::BlockParen(left)
                };

                let children_right = if operations.len() == 1 {
                    operations.remove(0)
                } else {
                    Value::BlockParen(operations)
                };

                if cfg!(test) {
                    println!("Left => {children_left:?}");
                    println!("Right => {children_right:?}");
                    println!();
                }

                let curr_node_id = to_tree(ctx, operation, tree, curr_node_id)?;

                to_tree(ctx, children_left, tree, &curr_node_id)?;
                to_tree(ctx, children_right, tree, &curr_node_id)?;

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
            let double_node = TreeNodeValue::Double(num);
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
            let double_node = TreeNodeValue::Int(num);
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
            let value = ctx
                .get(name)
                .copied()
                .context(format!("variable {name} not found in ctx"))?;

            if cfg!(test) {
                dbg!(value);
            }
            return to_tree(ctx, Value::Decimal(value), tree, curr_node_id);
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

                let _ = to_tree(ctx, value, tree, &node_id)?
                    .context(format!("invalid variable expr {n}"))?;

                Ok(node_id)
            } else {
                Err(anyhow::Error::msg("invalid variable expression"))
            }
        }
    }
}

// endregion: reducers

// region: calculate
fn compute_recur(
    node: Option<NodeRef<TreeNodeValue>>,
    ctx: &mut HashMap<String, f64>,
) -> f64 {
    if let Some(node) = node {
        match node.data() {
            TreeNodeValue::Ops(Operator::Add) => {
                if node.children().count() == 1 {
                    return compute_recur(node.first_child(), ctx);
                }
                compute_recur(node.first_child(), ctx)
                    + compute_recur(node.last_child(), ctx)
            }
            TreeNodeValue::Ops(Operator::Mult) => {
                if node.children().count() == 1 {
                    return compute_recur(node.first_child(), ctx);
                }
                compute_recur(node.first_child(), ctx)
                    * compute_recur(node.last_child(), ctx)
            }
            TreeNodeValue::Ops(Operator::Subtr) => {
                if node.children().count() == 1 {
                    return -compute_recur(node.first_child(), ctx);
                }
                compute_recur(node.first_child(), ctx)
                    - compute_recur(node.last_child(), ctx)
            }
            TreeNodeValue::Ops(Operator::Exp) => {
                if node.children().count() == 1 {
                    return compute_recur(node.first_child(), ctx);
                }
                compute_recur(node.first_child(), ctx)
                    .powf(compute_recur(node.last_child(), ctx))
            }
            TreeNodeValue::Ops(Operator::Div) => {
                if node.children().count() == 1 {
                    return compute_recur(node.first_child(), ctx);
                }
                compute_recur(node.first_child(), ctx)
                    / compute_recur(node.last_child(), ctx)
            }
            TreeNodeValue::Int(v) => *v as f64,
            TreeNodeValue::Double(v) => *v,
            TreeNodeValue::VariableAssign(name) => {
                let v = compute_recur(node.first_child(), ctx);
                ctx.insert(name.to_owned(), v);
                v
            }
        }
    } else {
        0.
    }
}
// endregion: calculate

// region: exposed api
pub fn compute(s: &str, ctx: &mut HashMap<String, f64>) -> anyhow::Result<f64> {
    let (rest, value) =
        parse_str(s).map_err(|e| anyhow::Error::msg(e.to_string()))?;

    if cfg!(test) {
        dbg!(rest);
        dbg!(&value);
    }
    anyhow::ensure!(rest.trim().is_empty(), "Invalid operation!");

    let mut tree: Tree<TreeNodeValue> = Tree::new();
    to_tree(ctx, value, &mut tree, &None)?;

    anyhow::ensure!(tree.root_id().is_some(), "Invalid expression!");

    if cfg!(test) {
        let mut tree_fmt = String::new();
        tree.write_formatted(&mut tree_fmt)?;
        println!("DEBUG: {tree_fmt}");
    }

    let root = tree.root();

    // i don't care if it panics, i catch it later
    std::panic::catch_unwind(AssertUnwindSafe(|| compute_recur(root, ctx)))
        .map_err(|_| {
            anyhow::Error::msg("could not safely compute the whole thing")
        })
}
// endregion: exposed api

#[cfg(test)]
mod test {

    use std::collections::HashMap;

    use crate::programs::calc::{compute, parse_str, Operator::*, Value};

    #[test]
    #[should_panic(expected = "invalid expression!")]
    fn test_expr_invalid() {
        let expr = "use example";
        let mut ctx = HashMap::from([("x".to_string(), 2.)]);
        compute(expr, &mut ctx).unwrap();
    }
    #[test]
    #[should_panic(expected = "invalid expression!")]
    fn test_expr_invalid_drc() {
        let expr = "drc logs -f triplestore";
        let mut ctx = HashMap::from([("x".to_string(), 2.)]);
        compute(expr, &mut ctx).unwrap();
    }

    #[test]
    #[should_panic(expected = "Invalid operation!")]
    fn test_op_invalid() {
        let expr = "use example = wesh";
        let mut ctx = HashMap::from([("x".to_string(), 2.)]);
        compute(expr, &mut ctx).unwrap();
    }

    #[test]
    fn test_compute_with_ctx() {
        let expr = "x * 5";
        let mut ctx = HashMap::from([("x".to_string(), 2.)]);

        let res = compute(expr, &mut ctx).unwrap();
        assert_eq!(10., res);
    }
    #[test]
    fn test_compute_assign_with_ctx() {
        let expr = "y = x * 5";
        let mut ctx = HashMap::from([("x".to_string(), 2.)]);

        let res = compute(expr, &mut ctx).unwrap();
        assert_eq!(10., res);

        assert_eq!(ctx.get("y"), Some(&10.));
    }

    #[test]
    fn test_variable() {
        let expr = "x*5+9*y/8";
        let (_, op) = parse_str(expr).unwrap();
        assert_eq!(
            op,
            Value::Expression(vec![
                Value::Variable("x",),
                Value::Operation(Mult,),
                Value::Integer(5,),
                Value::Operation(Add,),
                Value::Integer(9,),
                Value::Operation(Mult,),
                Value::Variable("y",),
                Value::Operation(Div,),
                Value::Integer(8,),
            ],),
        );
    }
    #[test]
    fn test_variable_expr() {
        let expr = "z = x*5+9*y/8";
        let (_, op) = parse_str(expr).unwrap();
        assert_eq!(
            op,
            Value::VariableExpr {
                name: Box::new(Value::Variable("z")),
                expr: Box::new(Value::Expression(vec![
                    Value::Variable("x",),
                    Value::Operation(Mult,),
                    Value::Integer(5,),
                    Value::Operation(Add,),
                    Value::Integer(9,),
                    Value::Operation(Mult,),
                    Value::Variable("y",),
                    Value::Operation(Div,),
                    Value::Integer(8,),
                ]))
            },
        );
    }

    #[test]
    fn test_compute() {
        let mut ctx = HashMap::new();
        assert_eq!(
            3280.3,
            compute("x=2* (9*(5-(1/2))) ^2 -1 / 5", &mut ctx).unwrap()
        );
        assert_eq!(
            3274.9,
            compute("y = 2* (9*(5-(1/2))) ^2 -1 / 5 * 8 - 4", &mut ctx)
                .unwrap()
        );
        assert_eq!(
            -670.9548307564088,
            compute("z = 78/5-4.5*(9+7^2.5)-12*4+1-8/3*4-5", &mut ctx).unwrap()
        );
        assert_eq!(
            37736.587719298244,
            compute("f = 1988*19-(((((((9*2))))+2*4)-3))/6-1^2*1000/(7-4*(3/9-(9+3/2-4)))", &mut ctx).unwrap()
        );
        assert_eq!(0., compute("0", &mut ctx).unwrap());
        assert_eq!(9., compute("9", &mut ctx).unwrap());
        assert_eq!(-9., compute("-9", &mut ctx).unwrap());
        assert_eq!(
            6. / 2. * (2. + 1.),
            compute("6/2*(2+1)", &mut ctx).unwrap()
        );
        assert_eq!(2. - 1. / 5., compute("2 -1 / 5", &mut ctx).unwrap());
        // todo maybe should panic in these cases
        assert_eq!(2. * 4., compute("2* * *4", &mut ctx).unwrap());
        assert_eq!(2. * 4., compute("2* ** *4", &mut ctx).unwrap());
        assert_eq!(4., compute("*4", &mut ctx).unwrap());

        // compute with variables
        assert_eq!(
            -4765.37866215695,
            compute("f = 555*19-(((((((9*2))))+2*f)-x))/6-1^2*y/(z-4*(3/9-(9+3/2-4))) - x", &mut ctx).unwrap()
        );

        assert_eq!(ctx.get("f"), Some(&-4765.37866215695));
        assert_eq!(ctx.get("z"), Some(&-670.9548307564088));
        assert_eq!(ctx.get("y"), Some(&3274.9));
        assert_eq!(ctx.get("x"), Some(&3280.3));
    }
}
