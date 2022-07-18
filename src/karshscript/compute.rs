use std::{
    borrow::{Borrow, Cow},
    ops::{Neg, Not},
};

use anyhow::Error;
use slab_tree::{NodeRef, Tree};

use crate::{
    karshscript::parser::{load_file_path, parse_instructions},
    prelude::BTreeMap,
};

use super::{
    ast::to_ast,
    primitive::{Abs, And, Cos, Logarithm, Or, Pow, Primitive, Sin, Sqrt, Tan},
    Operator, TreeNodeValue, Value,
};

fn compute_recur(
    node: Option<NodeRef<TreeNodeValue>>,
    ctx: &mut BTreeMap<String, Primitive>,
) -> anyhow::Result<Primitive> {
    if let Some(node) = node {
        match node.data() {
            TreeNodeValue::Ops(Operator::Not) => {
                if node.children().count() != 1 {
                    return Err(Error::msg(
                        "only one value allowed, no '!' possible",
                    ));
                }
                let left = compute_recur(node.first_child(), ctx)?;
                (!left).ok()
            }
            TreeNodeValue::Ops(Operator::Add) => {
                if node.children().count() == 1 {
                    return compute_recur(node.first_child(), ctx);
                }
                let left = compute_recur(node.first_child(), ctx)?;
                let right = compute_recur(node.last_child(), ctx)?;
                (left + right).ok()
            }
            TreeNodeValue::Ops(Operator::Mult) => {
                if node.children().count() == 1 {
                    return compute_recur(node.first_child(), ctx);
                }
                let left = compute_recur(node.first_child(), ctx)?;
                let right = compute_recur(node.last_child(), ctx)?;
                (left * right).ok()
            }
            TreeNodeValue::Ops(Operator::Mod) => {
                if node.children().count() == 1 {
                    return compute_recur(node.first_child(), ctx);
                }
                let left = compute_recur(node.first_child(), ctx)?;
                let right = compute_recur(node.last_child(), ctx)?;
                (left % right).ok()
            }
            TreeNodeValue::Ops(Operator::Subtr) => {
                if node.children().count() == 1 {
                    return compute_recur(node.first_child(), ctx)?.neg().ok();
                }
                let left = compute_recur(node.first_child(), ctx)?;
                let right = compute_recur(node.last_child(), ctx)?;
                (left - right).ok()
            }
            TreeNodeValue::Ops(Operator::Pow) => {
                if node.children().count() == 1 {
                    return compute_recur(node.first_child(), ctx);
                }
                let left = compute_recur(node.first_child(), ctx)?;
                let right = compute_recur(node.last_child(), ctx)?;
                left.pow(right).ok()
            }
            TreeNodeValue::Ops(Operator::Div) => {
                if node.children().count() == 1 {
                    return compute_recur(node.first_child(), ctx);
                }
                let left = compute_recur(node.first_child(), ctx)?;
                let right = compute_recur(node.last_child(), ctx)?;
                (left / right).ok()
            }
            TreeNodeValue::Ops(Operator::Equal) => {
                if node.children().count() == 1 {
                    return Err(Error::msg(
                        "only one value, no '==' comparison possible",
                    ));
                }
                let left = compute_recur(node.first_child(), ctx)?;
                let right = compute_recur(node.last_child(), ctx)?;
                (left.is_equal(&right)).ok()
            }
            TreeNodeValue::Ops(Operator::And) => {
                if node.children().count() == 1 {
                    return Err(Error::msg(
                        "only one value, no '&&' comparison possible",
                    ));
                }
                let left = compute_recur(node.first_child(), ctx)?;
                let right = compute_recur(node.last_child(), ctx)?;
                (left.and(right)).ok()
            }
            TreeNodeValue::Ops(Operator::Or) => {
                if node.children().count() == 1 {
                    return Err(Error::msg(
                        "only one value, no '||' comparison possible",
                    ));
                }
                let left = compute_recur(node.first_child(), ctx)?;
                let right = compute_recur(node.last_child(), ctx)?;
                (left.or(right)).ok()
            }
            TreeNodeValue::Ops(Operator::NotEqual) => {
                if node.children().count() == 1 {
                    return Err(Error::msg(
                        "only one value, no '!=' comparison possible",
                    ));
                }
                let left = compute_recur(node.first_child(), ctx)?;
                let right = compute_recur(node.last_child(), ctx)?;
                (left.is_equal(&right).not()).ok()
            }
            TreeNodeValue::Ops(Operator::Less) => {
                if node.children().count() == 1 {
                    return Err(Error::msg(
                        "only one value, no '<' comparison possible",
                    ));
                }
                let left = compute_recur(node.first_child(), ctx)?;
                let right = compute_recur(node.last_child(), ctx)?;
                (left.is_less_than(&right)).ok()
            }
            TreeNodeValue::Ops(Operator::Greater) => {
                if node.children().count() == 1 {
                    return Err(Error::msg(
                        "only one value, no '>' comparison possible",
                    ));
                }
                let left = compute_recur(node.first_child(), ctx)?;
                let right = compute_recur(node.last_child(), ctx)?;
                (left.is_greater_than(&right)).ok()
            }
            TreeNodeValue::Ops(Operator::GreaterOrEqual) => {
                if node.children().count() == 1 {
                    return Err(Error::msg(
                        "only one value, no '>=' comparison possible",
                    ));
                }
                let left = compute_recur(node.first_child(), ctx)?;
                let right = compute_recur(node.last_child(), ctx)?;
                (left.is_greater_or_equal(&right)).ok()
            }
            TreeNodeValue::Ops(Operator::LessOrEqual) => {
                if node.children().count() == 1 {
                    return Err(Error::msg(
                        "only one value, no '<=' comparison possible",
                    ));
                }
                let left = compute_recur(node.first_child(), ctx)?;
                let right = compute_recur(node.last_child(), ctx)?;
                (left.is_less_or_equal(&right)).ok()
            }
            TreeNodeValue::Primitive(Primitive::Bool(b)) => {
                Ok(Primitive::Bool(*b))
            }
            TreeNodeValue::Primitive(Primitive::Error(err)) => {
                Err(Error::msg(*err))
            }
            TreeNodeValue::Primitive(p) => p.clone().ok(),
            TreeNodeValue::VariableAssign(name) => {
                let v = compute_recur(node.first_child(), ctx)?.ok()?;
                ctx.insert(name.to_owned(), v.clone());
                Ok(v)
            }
            TreeNodeValue::BuiltInFunction(fn_type) => {
                let v = compute_recur(node.first_child(), ctx)?;
                match fn_type {
                    super::BuiltInFunctionType::Sqrt => v.sqrt().ok(),
                    super::BuiltInFunctionType::Abs => v.abs().ok(),
                    super::BuiltInFunctionType::Log => v.log().ok(),
                    super::BuiltInFunctionType::Ln => v.ln().ok(),
                    super::BuiltInFunctionType::Sin => v.sin().ok(),
                    super::BuiltInFunctionType::Cos => v.cos().ok(),
                    super::BuiltInFunctionType::Tan => v.tan().ok(),
                }
            }
        }
    } else {
        Primitive::Int(0).ok()
    }
}

fn value_to_tree(
    value: Value,
    ctx: &mut BTreeMap<String, Primitive>,
) -> anyhow::Result<Tree<TreeNodeValue>> {
    let mut tree: Tree<TreeNodeValue> = Tree::new();
    to_ast(ctx, value, &mut tree, &None)?;

    anyhow::ensure!(tree.root_id().is_some(), "Invalid expression!");

    if cfg!(test) {
        let mut tree_fmt = String::new();
        tree.write_formatted(&mut tree_fmt)?;
        println!("===================DEBUG TREE==================");
        print!("{tree_fmt}");
        println!("===================DEBUG TREE==================");
    }
    Ok(tree)
}

// region: exposed api
pub fn compute(
    s: &str,
    ctx: &mut BTreeMap<String, Primitive>,
) -> anyhow::Result<Primitive> {
    let mut instruction_str: Cow<str> = Cow::Borrowed(s);
    let (rest, instructions) =
        match load_file_path(s).map_err(|e| Error::msg(e.to_string())) {
            Ok(file) => {
                instruction_str = Cow::Owned(file);
                parse_instructions(instruction_str.borrow())
            }
            Err(_) => parse_instructions(instruction_str.borrow()),
        }
        .map_err(|e| Error::msg(e.to_string()))?;

    let mut result = Primitive::Int(0);
    if cfg!(test) {
        dbg!(rest);
        dbg!(&instructions);
    }
    anyhow::ensure!(rest.trim().is_empty(), "Invalid operation!");

    fn compute(
        instruction: Value,
        ctx: &mut BTreeMap<String, Primitive>,
    ) -> anyhow::Result<Primitive> {
        let tree = value_to_tree(instruction, ctx)?;

        let root = tree.root();

        compute_recur(root, ctx)
    }

    for instruction in instructions {
        match instruction {
            Value::IfExpr { cond, exprs } => {
                let cond = compute(*cond, ctx)?;
                if matches!(cond, Primitive::Bool(true)) {
                    for instruction in exprs {
                        result = compute(instruction, ctx)?;
                    }
                }
            }
            _ => {
                result = compute(instruction, ctx)?;
            }
        }
    }

    Ok(result)
}
