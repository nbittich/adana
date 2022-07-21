use std::{
    borrow::{Borrow, Cow},
    ops::{Neg, Not},
};

use anyhow::{Context, Error};
use slab_tree::{NodeRef, Tree};

use crate::{
    karshscript::parser::{load_file_path, parse_instructions},
    prelude::BTreeMap,
};

use super::{
    ast::to_ast,
    primitive::{
        Abs, And, Array, Cos, Logarithm, Or, Pow, Primitive, Sin, Sqrt, Tan,
    },
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
                Ok(!left)
            }
            TreeNodeValue::Ops(Operator::Add) => {
                if node.children().count() == 1 {
                    return compute_recur(node.first_child(), ctx);
                }
                let left = compute_recur(node.first_child(), ctx)?;
                let right = compute_recur(node.last_child(), ctx)?;
                Ok(left + right)
            }
            TreeNodeValue::Ops(Operator::Mult) => {
                if node.children().count() == 1 {
                    return compute_recur(node.first_child(), ctx);
                }
                let left = compute_recur(node.first_child(), ctx)?;
                let right = compute_recur(node.last_child(), ctx)?;
                Ok(left * right)
            }
            TreeNodeValue::Ops(Operator::Mod) => {
                if node.children().count() == 1 {
                    return compute_recur(node.first_child(), ctx);
                }
                let left = compute_recur(node.first_child(), ctx)?;
                let right = compute_recur(node.last_child(), ctx)?;
                Ok(left % right)
            }
            TreeNodeValue::Ops(Operator::Subtr) => {
                if node.children().count() == 1 {
                    return Ok(compute_recur(node.first_child(), ctx)?.neg());
                }
                let left = compute_recur(node.first_child(), ctx)?;
                let right = compute_recur(node.last_child(), ctx)?;
                Ok(left - right)
            }
            TreeNodeValue::Ops(Operator::Pow) => {
                if node.children().count() == 1 {
                    return compute_recur(node.first_child(), ctx);
                }
                let left = compute_recur(node.first_child(), ctx)?;
                let right = compute_recur(node.last_child(), ctx)?;
                Ok(left.pow(right))
            }
            TreeNodeValue::Ops(Operator::Div) => {
                if node.children().count() == 1 {
                    return compute_recur(node.first_child(), ctx);
                }
                let left = compute_recur(node.first_child(), ctx)?;
                let right = compute_recur(node.last_child(), ctx)?;
                Ok(left / right)
            }
            TreeNodeValue::Ops(Operator::Equal) => {
                if node.children().count() == 1 {
                    return Err(Error::msg(
                        "only one value, no '==' comparison possible",
                    ));
                }
                let left = compute_recur(node.first_child(), ctx)?;
                let right = compute_recur(node.last_child(), ctx)?;
                Ok(left.is_equal(&right))
            }
            TreeNodeValue::Ops(Operator::And) => {
                if node.children().count() == 1 {
                    return Err(Error::msg(
                        "only one value, no '&&' comparison possible",
                    ));
                }
                let left = compute_recur(node.first_child(), ctx)?;
                let right = compute_recur(node.last_child(), ctx)?;
                Ok(left.and(right))
            }
            TreeNodeValue::Ops(Operator::Or) => {
                if node.children().count() == 1 {
                    return Err(Error::msg(
                        "only one value, no '||' comparison possible",
                    ));
                }
                let left = compute_recur(node.first_child(), ctx)?;
                let right = compute_recur(node.last_child(), ctx)?;
                Ok(left.or(right))
            }
            TreeNodeValue::Ops(Operator::NotEqual) => {
                if node.children().count() == 1 {
                    return Err(Error::msg(
                        "only one value, no '!=' comparison possible",
                    ));
                }
                let left = compute_recur(node.first_child(), ctx)?;
                let right = compute_recur(node.last_child(), ctx)?;
                Ok(left.is_equal(&right).not())
            }
            TreeNodeValue::Ops(Operator::Less) => {
                if node.children().count() == 1 {
                    return Err(Error::msg(
                        "only one value, no '<' comparison possible",
                    ));
                }
                let left = compute_recur(node.first_child(), ctx)?;
                let right = compute_recur(node.last_child(), ctx)?;
                Ok(left.is_less_than(&right))
            }
            TreeNodeValue::Ops(Operator::Greater) => {
                if node.children().count() == 1 {
                    return Err(Error::msg(
                        "only one value, no '>' comparison possible",
                    ));
                }
                let left = compute_recur(node.first_child(), ctx)?;
                let right = compute_recur(node.last_child(), ctx)?;
                Ok(left.is_greater_than(&right))
            }
            TreeNodeValue::Ops(Operator::GreaterOrEqual) => {
                if node.children().count() == 1 {
                    return Err(Error::msg(
                        "only one value, no '>=' comparison possible",
                    ));
                }
                let left = compute_recur(node.first_child(), ctx)?;
                let right = compute_recur(node.last_child(), ctx)?;
                Ok(left.is_greater_or_equal(&right))
            }
            TreeNodeValue::Ops(Operator::LessOrEqual) => {
                if node.children().count() == 1 {
                    return Err(Error::msg(
                        "only one value, no '<=' comparison possible",
                    ));
                }
                let left = compute_recur(node.first_child(), ctx)?;
                let right = compute_recur(node.last_child(), ctx)?;
                Ok(left.is_less_or_equal(&right))
            }
            TreeNodeValue::Primitive(Primitive::Bool(b)) => {
                Ok(Primitive::Bool(*b))
            }
            TreeNodeValue::Primitive(Primitive::Error(err)) => {
                Err(Error::msg(*err))
            }
            TreeNodeValue::Primitive(p) => Ok(p.clone()),
            TreeNodeValue::VariableAssign(name) => {
                let v = compute_recur(node.first_child(), ctx)?;
                if !matches!(v, Primitive::Error(_)) {
                    ctx.insert(name.to_owned(), v.clone());
                }
                Ok(v)
            }
            TreeNodeValue::BuiltInFunction(fn_type) => {
                let v = compute_recur(node.first_child(), ctx)?;
                match fn_type {
                    super::BuiltInFunctionType::Sqrt => Ok(v.sqrt()),
                    super::BuiltInFunctionType::Abs => Ok(v.abs()),
                    super::BuiltInFunctionType::Log => Ok(v.log()),
                    super::BuiltInFunctionType::Ln => Ok(v.ln()),
                    super::BuiltInFunctionType::Sin => Ok(v.sin()),
                    super::BuiltInFunctionType::Cos => Ok(v.cos()),
                    super::BuiltInFunctionType::Tan => Ok(v.tan()),
                    super::BuiltInFunctionType::Length => Ok(v.len()),
                    super::BuiltInFunctionType::Println => {
                        println!("{v}");
                        Ok(Primitive::Unit)
                    }
                    super::BuiltInFunctionType::Print => {
                        print!("{v}");
                        Ok(Primitive::Unit)
                    }
                }
            }
            TreeNodeValue::IfExpr(v) => {
                compute_instructions(vec![v.clone()], ctx)
            }
            TreeNodeValue::WhileExpr(v) => {
                compute_instructions(vec![v.clone()], ctx)
            }
            TreeNodeValue::Array(arr) => {
                let mut primitives = vec![];
                for v in arr {
                    let primitive = compute_instructions(vec![v.clone()], ctx)?;
                    primitives.push(primitive);
                }
                Ok(Primitive::Array(primitives))
            }
            TreeNodeValue::ArrayAccess { index, array } => {
                let error_message = || {
                    format!("illegal index {index} for array access {array:?}")
                };
                match array {
                    Value::Variable(v) => {
                        let array =
                            ctx.get(v).context("array not found in context")?;
                        Ok(array.index_at(index.clone()))
                    }
                    Value::Array(array) => {
                        if let Primitive::Int(index) = index {
                            let index = *index as usize;
                            let value =
                                array.get(index).context(error_message())?;
                            if index < array.len() {
                                let primitive = compute_instructions(
                                    vec![value.clone()],
                                    ctx,
                                )?;
                                return Ok(primitive);
                            }
                        }
                        Err(anyhow::Error::msg(error_message()))
                    }
                    _ => Err(anyhow::Error::msg(error_message())),
                }
            }
            TreeNodeValue::VariableArrayAssign { name, index } => {
                let mut v = compute_recur(node.first_child(), ctx)?;
                let array =
                    ctx.get_mut(name).context("array not found in context")?;
                Ok(array.swap_mem(&mut v, index))
            }
        }
    } else {
        Ok(Primitive::Unit)
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

fn compute_instructions(
    instructions: Vec<Value>,
    ctx: &mut BTreeMap<String, Primitive>,
) -> anyhow::Result<Primitive> {
    let mut result = Primitive::Unit;

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
            Value::IfExpr { cond, exprs, else_expr } => {
                let cond = compute(*cond, ctx)?;
                if matches!(cond, Primitive::Bool(true)) {
                    for instruction in exprs {
                        result = compute(instruction, ctx)?;
                    }
                } else if let Some(else_expr) = else_expr {
                    for instruction in else_expr {
                        result = compute(instruction, ctx)?;
                    }
                }
            }
            Value::WhileExpr { cond, exprs } => {
                while matches!(
                    compute(*cond.clone(), ctx)?,
                    Primitive::Bool(true)
                ) {
                    for instruction in &exprs {
                        result = compute(instruction.clone(), ctx)?;
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

    if cfg!(test) {
        dbg!(rest);
        dbg!(&instructions);
    }
    anyhow::ensure!(rest.trim().is_empty(), "Invalid operation!");

    compute_instructions(instructions, ctx)
}
