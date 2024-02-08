mod utils;
use adana_script_core::primitive::{Primitive, RefPrimitive};
use std::collections::BTreeMap;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn compute(script: &str, mem: &mut [u8]) -> Result<Primitive, JsError> {
    utils::set_panic_hook();
    let mut ctx: BTreeMap<String, RefPrimitive> = if !mem.is_empty() {
        bincode::deserialize(mem)?
    } else {
        BTreeMap::new()
    };

    let result = adana_script::compute(script, &mut ctx, "N/A")
        .map_err(|e| e.to_string())
        .map_err(|e| JsError::new(&e))?;
    bincode::serialize_into(mem, &ctx)?;

    Ok(result)
}

#[wasm_bindgen]
pub fn compute_as_js_value(
    script: &str,
    mem: &mut [u8],
) -> Result<JsValue, JsError> {
    let result = compute(script, mem)?;
    Ok(serde_wasm_bindgen::to_value(&result)?)
}

#[wasm_bindgen]
pub fn compute_as_string(
    script: &str,
    mem: &mut [u8],
) -> Result<String, JsError> {
    let result = compute(script, mem)?;
    Ok(result.to_string())
}
