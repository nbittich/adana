#![cfg(target_arch = "wasm32")]

mod utils;
use adana_script_core::constants::WASM_OUT;
use adana_script_core::primitive::{Primitive, RefPrimitive};
use std::collections::BTreeMap;
use wasm_bindgen::prelude::*;

#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

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
#[wasm_bindgen]
pub fn compute_as_js_value(
    script: &str,
    mem: &mut [u8],
) -> Result<JsValue, JsError> {
    let (ctx, result) = compute(script, mem)?;
    bincode::serialize_into(mem, &ctx)?;

    Ok(serde_wasm_bindgen::to_value(&result)?)
}

#[wasm_bindgen]
pub fn compute_as_string(
    script: &str,
    mem: &mut [u8],
) -> Result<String, JsError> {
    let (ctx, result) = compute(script, mem)?;

    let result = {
        if let Some(out) = ctx.get(WASM_OUT) {
            let mut out_rl = out.write()?;

            match &mut *out_rl {
                Primitive::Array(ref mut a) if !a.is_empty() => {
                    let mut s = String::new();
                    for p in a.drain(0..) {
                        s.push_str(&p.to_string());
                    }
                    if !matches!(result, Primitive::Unit) {
                        s.push_str(&result.to_string());
                    }

                    Ok(s)
                }
                _ => Ok(result.to_string()),
            }
        } else {
            Ok(result.to_string())
        }
    };
    bincode::serialize_into(mem, &ctx)?;

    result
}
