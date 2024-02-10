//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use adana_script_core::primitive::{Primitive, RefPrimitive};
use adana_script_wasm::compute_as_js_value;
use std::assert_eq;
use std::collections::BTreeMap;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn compute_as_js_value_test() {
    let mut memory = vec![0; 64];
    let res = compute_as_js_value("x = 1+1", &mut memory)
        .map_err(JsValue::from)
        .unwrap();
    let res: Primitive = serde_wasm_bindgen::from_value(res).unwrap();
    assert_eq!(Primitive::Int(2), res);
    let ctx: BTreeMap<String, RefPrimitive> =
        bincode::deserialize(&memory).unwrap();
    assert_eq!(1, ctx.len());
    assert_eq!(Primitive::Int(2), ctx["x"].read().unwrap().clone());
}
