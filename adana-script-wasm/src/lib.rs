use adana_script_core::primitive::{Primitive, RefPrimitive};
use serde::Serialize;
use std::collections::BTreeMap;
use wasm_bindgen::prelude::*;
#[derive(Serialize)]
struct Out {
    ctx: BTreeMap<String, RefPrimitive>,
    result: Primitive,
}

#[wasm_bindgen]
pub fn compute(script: &str, ctx: JsValue) -> Result<JsValue, JsError> {
    let mut ctx: BTreeMap<String, RefPrimitive> =
        serde_wasm_bindgen::from_value(ctx)?;
    let result = adana_script::compute(script, &mut ctx, "N/A")
        .map_err(|e| e.to_string())
        .map_err(|e| JsError::new(&e))?;

    Ok(serde_wasm_bindgen::to_value(&Out { ctx, result })?)
}
