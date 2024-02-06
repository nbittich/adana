mod utils;
use adana_script_core::primitive::{Primitive, RefPrimitive};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Serialize, Deserialize)]
pub struct Out {
    pub ctx: BTreeMap<String, RefPrimitive>,
    pub result: Primitive,
}

#[wasm_bindgen]
pub fn compute(script: &str, ctx: JsValue) -> Result<JsValue, JsError> {
    utils::set_panic_hook();
    // TODO try wasm memory:
    // https://developer.mozilla.org/fr/docs/WebAssembly/JavaScript_interface/Memory
    let mut ctx: BTreeMap<String, RefPrimitive> =
        if !ctx.is_undefined() && ctx.is_object() {
            serde_wasm_bindgen::from_value(ctx)?
        } else {
            BTreeMap::new()
        };
    let result = adana_script::compute(script, &mut ctx, "N/A")
        .map_err(|e| e.to_string())
        .map_err(|e| JsError::new(&e))?;

    Ok(serde_wasm_bindgen::to_value(&Out { ctx, result })?)
}
