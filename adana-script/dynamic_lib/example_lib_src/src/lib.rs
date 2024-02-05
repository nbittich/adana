use std::collections::BTreeMap;

use adana_script_core::{
    primitive::{Compiler, NativeFunctionCallResult, Primitive},
    Value,
};
use anyhow::Context;

#[no_mangle]
pub fn hello(
    params: Vec<Primitive>,
    _compiler: Box<Compiler>,
) -> NativeFunctionCallResult {
    let mut s = String::from("Hello");
    for p in params.iter() {
        s.push(' ');
        s.push_str(&p.to_string());
    }
    Ok(Primitive::String(s))
}
#[no_mangle]
pub fn callback(
    mut params: Vec<Primitive>,
    mut compiler: Box<Compiler>,
) -> NativeFunctionCallResult {
    let s = String::from("Hello");

    let res = Primitive::String(s);
    let prim = params.get_mut(0).context("missing function parameters")?;
    let prim_callback = prim.clone().to_value()?;
    let fn_call = Value::FunctionCall {
        parameters: Box::new(Value::BlockParen(vec![res.to_value()?])),
        function: Box::new(prim_callback),
    };

    let ctx = BTreeMap::new();
    let r = compiler(fn_call, ctx)?;
    Ok(r)
}
