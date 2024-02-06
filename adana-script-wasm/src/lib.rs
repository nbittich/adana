mod utils;
use adana_script_core::primitive::RefPrimitive;
use std::collections::BTreeMap;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn compute(script: &str, mem: &mut [u8]) -> Result<JsValue, JsError> {
    utils::set_panic_hook();
    // wasm memory:
    // https://developer.mozilla.org/fr/docs/WebAssembly/JavaScript_interface/Memory
    let mut ctx: BTreeMap<String, RefPrimitive> = if !mem.is_empty() {
        bincode::deserialize(mem)?
    } else {
        BTreeMap::new()
    };

    let result = adana_script::compute(script, &mut ctx, "N/A")
        .map_err(|e| e.to_string())
        .map_err(|e| JsError::new(&e))?;

    bincode::serialize_into(mem, &ctx)?;
    Ok(serde_wasm_bindgen::to_value(&result)?)
}
