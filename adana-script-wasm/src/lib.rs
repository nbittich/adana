#![cfg(target_arch = "wasm32")]

mod utils;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

mod internal {
    use crate::utils;
    use adana_script_core::primitive::{Primitive, RefPrimitive};
    use std::collections::BTreeMap;
    use wasm_bindgen::prelude::{JsError, JsValue};
    fn compute(
        script: &str,
        mem: &mut [u8],
    ) -> Result<(BTreeMap<String, RefPrimitive>, Primitive), JsError> {
        utils::set_panic_hook();
        let mut ctx: BTreeMap<String, RefPrimitive> = if !mem.is_empty() {
            bincode::deserialize(mem)?
        } else {
            BTreeMap::new()
        };

        let result = adana_script::compute(script, &mut ctx, "N/A")
            .map_err(|e| e.to_string())
            .map_err(|e| JsError::new(&e))?;

        Ok((ctx, result))
    }
    pub(super) fn compute_as_js_value(
        script: &str,
        mem: &mut [u8],
    ) -> Result<(BTreeMap<String, RefPrimitive>, JsValue), JsError> {
        let (ctx, result) = compute(script, mem)?;
        let value = serde_wasm_bindgen::to_value(&result)?;

        Ok((ctx, value))
    }
    pub(super) fn compute_as_string(
        script: &str,
        mem: &mut [u8],
    ) -> Result<(BTreeMap<String, RefPrimitive>, String), JsError> {
        let (ctx, result) = compute(script, mem)?;
        Ok((ctx, result.to_string()))
    }
}

#[wasm_bindgen]
pub fn compute_as_js_value(
    script: &str,
    mem: &mut [u8],
) -> Result<JsValue, JsError> {
    let (ctx, res) = internal::compute_as_js_value(script, mem)?;
    bincode::serialize_into(mem, &ctx)?;
    Ok(res)
}

#[wasm_bindgen]
pub fn compute_as_string(
    script: &str,
    mem: &mut [u8],
) -> Result<String, JsError> {
    let (ctx, res) = internal::compute_as_string(script, mem)?;
    bincode::serialize_into(mem, &ctx)?;
    Ok(res)
}

#[wasm_bindgen]
pub fn make_ctx_and_compute_as_string(
    script: &str,
    heap_size_in_mb: Option<usize>,
) -> Result<String, JsError> {
    let heap_size =
        if let Some(heap_size) = heap_size_in_mb.filter(|h| h <= &32) {
            heap_size
        } else {
            1
        };
    let mut mem = Vec::with_capacity(heap_size * 1024 * 1024); // 1mb by default
    let (_, res) = internal::compute_as_string(script, &mut mem)?;
    Ok(res)
}
