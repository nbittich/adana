use nom::{
    character::complete::i64 as I64,
    combinator::{all_consuming, map_parser, value},
    multi::many1,
    number::complete::{double, recognize_float},
};
use serde::{Deserialize, Serialize};
use slab_tree::{NodeId, NodeRef, Tree};

use crate::prelude::*;

// region: structs
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
enum Value {
    Operations(Vec<Value>),
    Operation(Operator),
    Decimal(f64),
    Integer(i64),
    BlockParen(Vec<Value>),
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Serialize, Deserialize)]
enum Operator {
    Add,
    Subtr,
    Mult,
    Div,
    Exp,
}

#[derive(Debug, PartialEq, Copy, Clone, Serialize, Deserialize)]
enum TreeNodeValue {
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
    move |s| value(Value::Operation(operation), tag_no_space(sep))(s)
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

fn parse_operations(s: &str) -> Res<Value> {
    preceded(
        multispace0,
        terminated(map(many1(parse_value), Value::Operations), multispace0),
    )(s)
}

// endregion: parsers

// region: reducers
fn to_tree(
    value: &mut Value,
    tree: &mut Tree<TreeNodeValue>,
    curr_node_id: Option<NodeId>,
) -> Option<NodeId> {
    match value {
        Value::Operations(operations) | Value::BlockParen(operations) => {
            if operations.is_empty() {
                return None;
            }
            if operations.len() == 1 {
                return to_tree(&mut operations[0], tree, curr_node_id);
            }

            fn filter_op(op: Operator) -> impl Fn(&Value) -> bool {
                move |c| matches!(c, Value::Operation(operator) if operator == &op)
            }

            let op_pos = None
                .or_else(|| operations.iter().rposition(filter_op(Operator::Add)))
                .or_else(|| operations.iter().rposition(filter_op(Operator::Subtr)))
                .or_else(|| operations.iter().rposition(filter_op(Operator::Mult)))
                .or_else(|| operations.iter().rposition(filter_op(Operator::Div)))
                .or_else(|| operations.iter().rposition(filter_op(Operator::Exp)));

            if let Some(op_pos) = op_pos {
                let (left, right) = operations.split_at_mut(op_pos);

                let mut children_left = if left.len() == 1 {
                    left[0].clone()
                } else {
                    Value::BlockParen(left.into())
                };
                let mut children_right = if right[1..].len() == 1 {
                    right[1].clone()
                } else {
                    Value::BlockParen(right[1..].into())
                };

                if cfg!(test) {
                    println!("Left => {children_left:?}");
                    println!("Right => {children_right:?}");
                    println!();
                }

                let curr_node_id = to_tree(&mut operations[op_pos], tree, curr_node_id);
                to_tree(&mut children_left, tree, curr_node_id);
                to_tree(&mut children_right, tree, curr_node_id);

                curr_node_id
            } else {
                None
            }
        }

        Value::Operation(operator) => {
            let ops = TreeNodeValue::Ops(*operator);
            if let Some(node_id) = curr_node_id {
                let mut node = tree.get_mut(node_id).expect("node id does not exist!");

                let node = node.append(ops);
                Some(node.node_id())
            } else if let Some(mut root_node) = tree.root_mut() {
                let node = root_node.append(ops);
                Some(node.node_id())
            } else {
                let _ = tree.set_root(ops);
                tree.root_id()
            }
        }

        Value::Decimal(num) => {
            let double_node = TreeNodeValue::Double(*num);
            if let Some(node_id) = curr_node_id {
                let mut node = tree.get_mut(node_id).expect("node id does not exist!");
                node.append(double_node);
                Some(node.node_id())
            } else if let Some(mut root_node) = tree.root_mut() {
                root_node.append(double_node);
                tree.root_id()
            } else {
                Some(tree.set_root(double_node))
            }
        }
        Value::Integer(num) => {
            let double_node = TreeNodeValue::Int(*num);
            let node_id = if let Some(node_id) = curr_node_id {
                let mut node = tree.get_mut(node_id).expect("node id does not exist!");
                node.append(double_node);
                Some(node.node_id())
            } else if let Some(mut root_node) = tree.root_mut() {
                root_node.append(double_node);
                tree.root_id()
            } else {
                Some(tree.set_root(double_node))
            };
            node_id
        }
    }
}

// endregion: reducers

// region: calculate
fn calculate(node: Option<NodeRef<TreeNodeValue>>) -> f64 {
    if let Some(node) = node {
        match node.data() {
            TreeNodeValue::Ops(Operator::Add) => {
                if node.children().count() == 1 {
                    return calculate(node.first_child());
                }
                calculate(node.first_child()) + calculate(node.last_child())
            }
            TreeNodeValue::Ops(Operator::Mult) => {
                calculate(node.first_child()) * calculate(node.last_child())
            }
            TreeNodeValue::Ops(Operator::Subtr) => {
                if node.children().count() == 1 {
                    return -calculate(node.first_child());
                }
                calculate(node.first_child()) - calculate(node.last_child())
            }
            TreeNodeValue::Ops(Operator::Exp) => {
                calculate(node.first_child()).powf(calculate(node.last_child()))
            }
            TreeNodeValue::Ops(Operator::Div) => {
                calculate(node.first_child()) / calculate(node.last_child())
            }
            TreeNodeValue::Int(v) => *v as f64,
            TreeNodeValue::Double(v) => *v,
        }
    } else {
        0.
    }
}
// endregion: calculate

// region: exposed api
pub fn compute(s: &str) -> anyhow::Result<f64> {
    let (rest, mut value) = parse_operations(s).map_err(|e| anyhow::Error::msg(e.to_string()))?;

    anyhow::ensure!(rest.trim().is_empty(), "Invalid operation!");

    let mut tree: Tree<TreeNodeValue> = Tree::new();
    to_tree(&mut value, &mut tree, None);

    let root = tree.root();

    // i don't care if it panics, i catch it later
    std::panic::catch_unwind(|| calculate(root)).map_err(|_| anyhow::Error::msg("oops panic!"))
}
// endregion: exposed api

#[cfg(test)]
mod test {
    use slab_tree::Tree;

    use crate::programs::calc::{calculate, parse_operations, to_tree, TreeNodeValue};

    #[test]
    fn test_calculate_1() {
        let (_, mut b) = parse_operations("2 -1 / 5").unwrap();
        let mut tree: Tree<TreeNodeValue> = Tree::new();
        to_tree(&mut b, &mut tree, None);

        let root = tree.root();

        let res = calculate(root);
        assert_eq!(1.8, res);
    }

    #[test]
    fn test_calculate_2() {
        let (_, mut b) = parse_operations("2* (9*(5-(1/2))) ^2 -1 / 5").unwrap(); // expects 3280.3

        let mut tree: Tree<TreeNodeValue> = Tree::new();
        to_tree(&mut b, &mut tree, None);

        let root = tree.root();

        let res = calculate(root);
        assert_eq!(3280.3, res);
    }

    #[test]
    fn test_calculate_3() {
        //2* (9*(5-(1/2))) ^2 -1 / 5 * 8 -4
        let (_, mut b) = parse_operations("2* (9*(5-(1/2))) ^2 -1 / 5 * 8 - 4").unwrap();

        let mut tree: Tree<TreeNodeValue> = Tree::new();
        to_tree(&mut b, &mut tree, None);

        let mut tree_fmt = String::new();

        tree.write_formatted(&mut tree_fmt).unwrap();

        println!("{tree_fmt}");

        let root = tree.root();

        let res = calculate(root);
        assert_eq!(3274.9, res);
    }
    #[test]
    fn test_calculate_4() {
        let (_, mut b) = parse_operations("78/5-4.5*(9+7^2.5)-12*4+1-8/3*4-5").unwrap();

        let mut tree: Tree<TreeNodeValue> = Tree::new();
        to_tree(&mut b, &mut tree, None);

        let mut tree_fmt = String::new();

        tree.write_formatted(&mut tree_fmt).unwrap();

        println!("{tree_fmt}");

        let root = tree.root();

        let res = calculate(root);
        assert_eq!(-670.9548307564088, res);
    }
    #[test]
    fn test_calculate_5() {
        let (_, mut b) = parse_operations("0").unwrap();

        let mut tree: Tree<TreeNodeValue> = Tree::new();
        to_tree(&mut b, &mut tree, None);

        let mut tree_fmt = String::new();

        tree.write_formatted(&mut tree_fmt).unwrap();

        println!("{tree_fmt}");

        let root = tree.root();

        let res = calculate(root);
        assert_eq!(0.0, res);
    }
    #[test]
    fn test_calculate_6() {
        let (_, mut b) = parse_operations("9").unwrap();

        let mut tree: Tree<TreeNodeValue> = Tree::new();
        to_tree(&mut b, &mut tree, None);

        let mut tree_fmt = String::new();

        tree.write_formatted(&mut tree_fmt).unwrap();

        println!("{tree_fmt}");

        let root = tree.root();

        let res = calculate(root);
        assert_eq!(9.0, res);
    }
    #[test]
    fn test_calculate_7() {
        let (_, mut b) = parse_operations("-9").unwrap();

        let mut tree: Tree<TreeNodeValue> = Tree::new();
        to_tree(&mut b, &mut tree, None);

        let mut tree_fmt = String::new();

        tree.write_formatted(&mut tree_fmt).unwrap();

        println!("{tree_fmt}");

        let root = tree.root();

        let res = calculate(root);
        assert_eq!(-9.0, res);
    }
}
