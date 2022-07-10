use slab_tree::{NodeRef, Tree};

use crate::prelude::{AssertUnwindSafe, BTreeMap};

use super::{
    ast::to_ast,
    number::{Number, Pow},
    parser::parse_var_expr,
    Operator, TreeNodeValue,
};

fn compute_recur(
    node: Option<NodeRef<TreeNodeValue>>,
    ctx: &mut BTreeMap<String, Number>,
) -> Number {
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
            TreeNodeValue::Ops(Operator::Mod) => {
                if node.children().count() == 1 {
                    return compute_recur(node.first_child(), ctx);
                }
                compute_recur(node.first_child(), ctx)
                    % compute_recur(node.last_child(), ctx)
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
                    .pow(compute_recur(node.last_child(), ctx))
            }
            TreeNodeValue::Ops(Operator::Div) => {
                if node.children().count() == 1 {
                    return compute_recur(node.first_child(), ctx);
                }
                compute_recur(node.first_child(), ctx)
                    / compute_recur(node.last_child(), ctx)
            }
            TreeNodeValue::Primitive(p) => *p,
            TreeNodeValue::VariableAssign(name) => {
                let v = compute_recur(node.first_child(), ctx);
                ctx.insert(name.to_owned(), v);
                v
            }
        }
    } else {
        Number::Int(0)
    }
}
// endregion: calculate

// region: exposed api
pub fn compute(
    s: &str,
    ctx: &mut BTreeMap<String, Number>,
) -> anyhow::Result<Number> {
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
    std::panic::catch_unwind(AssertUnwindSafe(|| compute_recur(root, ctx)))
        .map_err(|_| {
            anyhow::Error::msg("could not safely compute the whole thing")
        })
}
