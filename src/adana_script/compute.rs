use std::{
    borrow::Borrow,
    fs::{read_to_string, File},
    io::{BufRead, BufReader},
    ops::{Neg, Not},
    path::{Path, PathBuf},
};

use anyhow::{Context, Error};
use slab_tree::{NodeRef, Tree};

use crate::{adana_script::parser::parse_instructions, prelude::BTreeMap};

use super::{
    ast::to_ast,
    primitive::{
        Abs, And, Array, Cos, Logarithm, MutPrimitive, Or, Pow, Primitive, Sin,
        Sqrt, Tan, ToBool, ToNumber, TypeOf,
    },
    Operator, TreeNodeValue, Value,
};

fn compute_recur(
    node: Option<NodeRef<TreeNodeValue>>,
    ctx: &mut BTreeMap<String, MutPrimitive>,
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
            TreeNodeValue::Primitive(p) => Ok(p.clone()),
            TreeNodeValue::VariableAssign(name) => {
                let v = compute_recur(node.first_child(), ctx)?;
                if !matches!(v, Primitive::Error(_)) {
                    let mut old = ctx
                        .entry(name.clone())
                        .or_insert(Primitive::Unit.mut_prim())
                        .write()
                        .map_err(|e| {
                            anyhow::format_err!("could not acquire lock {e}")
                        })?;
                    *old = v.clone();
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
                    super::BuiltInFunctionType::Eval => {
                        if let Primitive::String(script) = v {
                            compute(&script, ctx)
                        } else {
                            Ok(Primitive::Error(format!("invalid script {v}")))
                        }
                    }
                    super::BuiltInFunctionType::Tan => Ok(v.tan()),
                    super::BuiltInFunctionType::ToInt => Ok(v.to_int()),
                    super::BuiltInFunctionType::ToDouble => Ok(v.to_double()),
                    super::BuiltInFunctionType::ToBool => Ok(v.to_bool()),
                    super::BuiltInFunctionType::ToString => {
                        Ok(Primitive::String(v.to_string()))
                    }
                    super::BuiltInFunctionType::Length => Ok(v.len()),
                    super::BuiltInFunctionType::Println => {
                        println!("{v}");
                        Ok(Primitive::Unit)
                    }
                    super::BuiltInFunctionType::Print => {
                        print!("{v}");
                        Ok(Primitive::Unit)
                    }
                    super::BuiltInFunctionType::Include => match v {
                        Primitive::String(file_path) => {
                            let curr_path = std::env::current_dir()
                                .context("no current dir! wasn't expected")?;
                            let temp_path = Path::new(&file_path);
                            if temp_path.is_absolute() || temp_path.exists() {
                                let parent = temp_path
                                    .parent()
                                    .context("parent doesn't exist")?;

                                std::env::set_current_dir(PathBuf::from(
                                    &parent,
                                ))?;
                            }

                            let res = temp_path
                                .file_name()
                                .context("file name not found")
                                .and_then(|p| {
                                    read_to_string(p)
                                        .map_err(anyhow::Error::new)
                                })
                                .and_then(move |file| compute(&file, ctx));
                            std::env::set_current_dir(curr_path)?; // todo this might be quiet fragile
                            res
                        }
                        _ => Ok(Primitive::Error(
                            "wrong include call".to_string(),
                        )),
                    },
                    super::BuiltInFunctionType::ReadLines => match v {
                        Primitive::String(file_path) => {
                            if !PathBuf::from(file_path.as_str()).exists() {
                                Ok(Primitive::Error(format!(
                                    "file {file_path} not found"
                                )))
                            } else {
                                let file = File::open(file_path)?;
                                let reader = BufReader::new(file);
                                Ok(Primitive::Array(
                                    reader
                                        .lines()
                                        .map(|s| s.map(Primitive::String))
                                        .collect::<Result<Vec<_>, _>>()?,
                                ))
                            }
                        }
                        _ => Ok(Primitive::Error(
                            "wrong read lines call".to_string(),
                        )),
                    },
                    super::BuiltInFunctionType::TypeOf => Ok(v.type_of()),
                }
            }
            TreeNodeValue::IfExpr(v) => {
                let mut scoped_ctx = ctx.clone();
                compute_instructions(vec![v.clone()], &mut scoped_ctx)
            }
            TreeNodeValue::WhileExpr(v) => {
                let mut scoped_ctx = ctx.clone();
                compute_instructions(vec![v.clone()], &mut scoped_ctx)
            }
            TreeNodeValue::Foreach(v) => {
                let mut scoped_ctx = ctx.clone();
                compute_instructions(vec![v.clone()], &mut scoped_ctx)
            }
            TreeNodeValue::Array(arr) => {
                let mut primitives = vec![];
                for v in arr {
                    let primitive = compute_instructions(vec![v.clone()], ctx)?;
                    match primitive {
                        v @ Primitive::Error(_) => return Ok(v),
                        Primitive::Unit => {
                            return Ok(Primitive::Error(
                                "cannot push unit () to array".to_string(),
                            ))
                        }
                        _ => primitives.push(primitive),
                    }
                }
                Ok(Primitive::Array(primitives))
            }
            TreeNodeValue::ArrayAccess { index, array } => {
                let error_message = || {
                    format!("illegal index {index} for array access {array:?}")
                };
                match (array, index) {
                    (Value::Variable(v), index) => {
                        let array = ctx
                            .get(v)
                            .context("array not found in context")?
                            .read()
                            .map_err(|e| {
                                anyhow::Error::msg(format!(
                                    "could not acquire lock {e}"
                                ))
                            })?;
                        Ok(array.index_at(index))
                    }
                    (Value::String(v), index) => {
                        let v = Primitive::String(v.clone());
                        Ok(v.index_at(index))
                    }
                    (Value::Array(array), Primitive::Int(index)) => {
                        let index = *index as usize;
                        let value = array.get(index).context(error_message())?;
                        if index < array.len() {
                            let primitive =
                                compute_instructions(vec![value.clone()], ctx)?;
                            return Ok(primitive);
                        }
                        Err(anyhow::Error::msg(error_message()))
                    }
                    _ => Err(anyhow::Error::msg(error_message())),
                }
            }
            TreeNodeValue::Struct(struc) => {
                let mut primitives = BTreeMap::new();
                for (k, v) in struc {
                    let primitive = compute_instructions(vec![v.clone()], ctx)?;
                    match primitive {
                        v @ Primitive::Error(_) => return Ok(v),
                        Primitive::Unit => {
                            return Ok(Primitive::Error(
                                "cannot push unit () to struct".to_string(),
                            ))
                        }
                        _ => {
                            primitives.insert(k.to_string(), primitive);
                        }
                    }
                }
                Ok(Primitive::Struct(primitives))
            }
            TreeNodeValue::StructAccess { struc, key } => match (struc, key) {
                (Value::Variable(v), key @ Primitive::String(_)) => {
                    let struc = ctx
                        .get(v)
                        .context("struct not found in context")?
                        .read()
                        .map_err(|e| {
                            anyhow::format_err!("could not acquire lock {e}")
                        })?;
                    Ok(struc.index_at(key))
                }
                _ => Ok(Primitive::Error(format!(
                    "Error struct access: struct {struc:?}, key {key} "
                ))),
            },
            TreeNodeValue::VariableArrayAssign { name, index } => {
                let mut v = compute_recur(node.first_child(), ctx)?;
                let mut array = ctx
                    .get_mut(name)
                    .context("array not found in context")?
                    .write()
                    .map_err(|e| {
                        anyhow::format_err!("could not acquire lock {e}")
                    })?;
                Ok(array.swap_mem(&mut v, index))
            }
            TreeNodeValue::Function(Value::Function { parameters, exprs }) => {
                if let Value::BlockParen(parameters) = parameters.borrow() {
                    let mut params = Vec::with_capacity(parameters.len());
                    for parameter in parameters {
                        if let Value::Variable(parameter) = parameter {
                            params.push(parameter.clone());
                        } else {
                            return Ok(Primitive::Error(format!(
                                "not a valid parameter: {parameter:?}"
                            )));
                        }
                    }
                    Ok(Primitive::Function {
                        parameters: params,
                        exprs: exprs.to_owned(),
                    })
                } else {
                    Ok(Primitive::Error(format!(
                        "not a valid function: {parameters:?}, {exprs:?}"
                    )))
                }
            }
            TreeNodeValue::FunctionCall(Value::FunctionCall {
                parameters,
                function,
            }) => {
                if let Value::BlockParen(param_values) = parameters.borrow() {
                    let function =
                        compute_instructions(vec![*function.clone()], ctx)?;
                    if let Primitive::Function { parameters, exprs } = function
                    {
                        let mut scope_ctx = BTreeMap::new();

                        // copy also the function definition to the scoped ctx
                        for (k, p) in ctx.iter() {
                            let maybe_fn = p.read().map_err(|e| {
                                anyhow::format_err!(
                                    "could not acquire lock {e}"
                                )
                            })?;
                            if matches!(
                                *maybe_fn,
                                Primitive::Function { parameters: _, exprs: _ }
                            ) {
                                scope_ctx.insert(k.to_string(), p.clone());
                            }
                        }

                        for (i, param) in parameters.iter().enumerate() {
                            if let Some(value) = param_values.get(i) {
                                let value = compute_instructions(
                                    vec![value.clone()],
                                    ctx,
                                )?;
                                scope_ctx
                                    .insert(param.clone(), value.mut_prim());
                            } else {
                                return Ok(Primitive::Error(format!(
                                    "missing parameter {param}"
                                )));
                            }
                        }

                        // TODO remove this and replace Arc<Mutex<T>> by Arc<T>
                        // call function in a specific os thread with its own stack
                        // This was relative to a small stack allocated by musl
                        // But now it doesn't seem needed anymore
                        // let res = spawn(move || {}).join().map_err(|e| {
                        //     anyhow::Error::msg(format!(
                        //         "something wrong: {e:?}"
                        //     ))
                        // })??;
                        let res = compute_instructions(exprs, &mut scope_ctx)?;

                        if let Primitive::EarlyReturn(v) = res {
                            return Ok(*v);
                        }
                        Ok(res)
                    } else {
                        Ok(Primitive::Error(format!(
                            " not a function: {function}"
                        )))
                    }
                } else {
                    Ok(Primitive::Error(format!(
                        "invalid function call: {parameters:?} => {function:?}"
                    )))
                }
            }
            TreeNodeValue::FunctionCall(v) => Ok(Primitive::Error(format!(
                "unexpected function call declaration: {v:?}"
            ))),
            TreeNodeValue::Function(v) => Ok(Primitive::Error(format!(
                "unexpected function declaration: {v:?}"
            ))),
            TreeNodeValue::Break => Ok(Primitive::NoReturn),
            TreeNodeValue::Null => Ok(Primitive::Null),
            TreeNodeValue::Drop(variables) => {
                if variables.iter().all(|k| ctx.contains_key(k)) {
                    for var in variables {
                        ctx.remove(var).unwrap();
                    }
                } else {
                    return Ok(Primitive::Error(format!("ctx doesn't contain all variables that must be dropped {variables:?}")));
                }

                Ok(Primitive::Unit)
            }
            TreeNodeValue::EarlyReturn(v) => {
                if let Some(v) = v {
                    let p = compute_instructions(vec![v.clone()], ctx)?;
                    Ok(Primitive::EarlyReturn(Box::new(p)))
                } else {
                    Ok(Primitive::EarlyReturn(Box::new(Primitive::Null)))
                }
            }
        }
    } else {
        Ok(Primitive::Unit)
    }
}

fn value_to_tree(
    value: Value,
    ctx: &mut BTreeMap<String, MutPrimitive>,
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

fn compute_lazy(
    instruction: Value,
    ctx: &mut BTreeMap<String, MutPrimitive>,
) -> anyhow::Result<Primitive> {
    let tree = value_to_tree(instruction, ctx)?;

    let root = tree.root();

    compute_recur(root, ctx)
}
fn compute_instructions(
    instructions: Vec<Value>,
    ctx: &mut BTreeMap<String, MutPrimitive>,
) -> anyhow::Result<Primitive> {
    let mut result = Primitive::Unit;

    for instruction in instructions {
        match instruction {
            v @ Value::EarlyReturn(_) => {
                let res = compute_lazy(v, ctx)?;
                if let Primitive::EarlyReturn(r) = res {
                    return Ok(*r);
                } else {
                    return Err(anyhow::Error::msg("bug! fixme"));
                }
            }
            Value::IfExpr { cond, exprs, else_expr } => {
                let cond = compute_lazy(*cond, ctx)?;
                if matches!(cond, Primitive::Error(_)) {
                    return Ok(cond);
                }
                if matches!(cond, Primitive::Bool(true)) {
                    let mut scoped_ctx = ctx.clone();

                    for instruction in exprs {
                        match compute_lazy(
                            instruction.clone(),
                            &mut scoped_ctx,
                        )? {
                            v @ Primitive::EarlyReturn(_)
                            | v @ Primitive::Error(_) => return Ok(v),
                            p => result = p,
                        }
                    }
                } else if let Some(else_expr) = else_expr {
                    let mut scoped_ctx = ctx.clone();

                    for instruction in else_expr {
                        match compute_lazy(
                            instruction.clone(),
                            &mut scoped_ctx,
                        )? {
                            v @ Primitive::EarlyReturn(_)
                            | v @ Primitive::Error(_) => return Ok(v),
                            p => result = p,
                        }
                    }
                }
            }
            Value::WhileExpr { cond, exprs } => {
                let mut scoped_ctx = ctx.clone();

                'while_loop: while matches!(
                    compute_lazy(*cond.clone(), &mut scoped_ctx)?,
                    Primitive::Bool(true)
                ) {
                    for instruction in &exprs {
                        match compute_lazy(
                            instruction.clone(),
                            &mut scoped_ctx,
                        )? {
                            Primitive::NoReturn => break 'while_loop,
                            v @ Primitive::EarlyReturn(_)
                            | v @ Primitive::Error(_) => return Ok(v),
                            p => result = p,
                        }
                    }
                }
            }
            Value::ForeachExpr { var, index_var, iterator, exprs } => {
                let iterator = compute_lazy(*iterator, ctx)?;

                let mut scoped_ctx = ctx.clone();
                let arr = match iterator {
                    Primitive::Array(arr) => arr,
                    Primitive::Struct(s) => s
                        .iter()
                        .map(|(k, v)| {
                            Primitive::Struct(BTreeMap::from([
                                ("key".into(), Primitive::String(k.clone())),
                                ("value".into(), v.clone()),
                            ]))
                        })
                        .collect(),
                    Primitive::String(s) => s
                        .chars()
                        .map(|c| Primitive::String(c.to_string()))
                        .collect(),
                    _ => {
                        return Ok(Primitive::Error(format!(
                            "not an iterable {iterator:?}"
                        )));
                    }
                };
                'foreach_loop: for (i, it) in arr.into_iter().enumerate() {
                    scoped_ctx.insert(var.clone(), it.mut_prim());
                    if let Some(index_var) = &index_var {
                        scoped_ctx.insert(
                            index_var.clone(),
                            Primitive::Int(i as i128).mut_prim(),
                        );
                    }
                    for instruction in &exprs {
                        match compute_lazy(
                            instruction.clone(),
                            &mut scoped_ctx,
                        )? {
                            Primitive::NoReturn => break 'foreach_loop,
                            v @ Primitive::EarlyReturn(_)
                            | v @ Primitive::Error(_) => return Ok(v),
                            p => result = p,
                        }
                    }
                }
            }
            _ => {
                result = compute_lazy(instruction, ctx)?;
            }
        }
        if let Primitive::EarlyReturn(p) = result {
            return Ok(*p);
        }
        if matches!(result, Primitive::Error(_)) {
            return Ok(result);
        }
    }

    Ok(result)
}
// region: exposed api
pub fn compute(
    s: &str,
    ctx: &mut BTreeMap<String, MutPrimitive>,
) -> anyhow::Result<Primitive> {
    let (rest, instructions) = parse_instructions(s).map_err(|e| {
        anyhow::Error::msg(format!("could not parse instructions. {e}"))
    })?;

    if cfg!(test) {
        dbg!(rest);
        dbg!(&instructions);
    }
    anyhow::ensure!(
        rest.trim().is_empty(),
        format!("Invalid operation! {instructions:?} => {rest}")
    );

    compute_instructions(instructions, ctx)
}
