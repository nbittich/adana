use std::ops::Neg;

use anyhow::Context;
use slab_tree::{NodeRef, Tree};

use crate::prelude::{AssertUnwindSafe, BTreeMap};

use super::{
    ast::to_ast,
    parser::parse_var_expr,
    primitive::{Abs, Cos, Logarithm, Pow, Primitive, Sin, Sqrt, Tan},
    Operator, TreeNodeValue,
};

fn compute_recur(
    node: Option<NodeRef<TreeNodeValue>>,
    ctx: &mut BTreeMap<String, Primitive>,
) -> anyhow::Result<Primitive> {
    if let Some(node) = node {
        match node.data() {
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

            TreeNodeValue::Primitive(Primitive::Bool(b)) => {
                Ok(Primitive::Bool(*b))
            }
            TreeNodeValue::Primitive(Primitive::Error(err)) => {
                Err(anyhow::Error::msg(err.clone()))
            }
            TreeNodeValue::Primitive(p) => p.ok(),
            TreeNodeValue::VariableAssign(name) => {
                let v = compute_recur(node.first_child(), ctx)?.ok()?;
                ctx.insert(name.to_owned(), v);
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
// endregion: calculate

// region: exposed api
pub fn compute(
    s: &str,
    ctx: &mut BTreeMap<String, Primitive>,
) -> anyhow::Result<Primitive> {
    let (rest, value) =
        parse_var_expr(s).map_err(|e| anyhow::Error::msg(e.to_string()))?;

    if cfg!(test) {
        dbg!(rest);
        dbg!(&value);
    }
    anyhow::ensure!(rest.trim().is_empty(), "Invalid operation!");

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

    let root = tree.root();

    // i don't care if it panics, i catch it later
    std::panic::catch_unwind(AssertUnwindSafe(|| {
        compute_recur(root, ctx).unwrap()
    }))
    .map_err(|_| anyhow::Error::msg("could not safely compute the whole thing"))
}
