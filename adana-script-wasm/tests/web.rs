//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use adana_script_core::primitive::Primitive;
use adana_script_wasm::{compute, Out};
use std::assert_eq;
use wasm_bindgen::JsValue;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
fn compute_test() {
    let res = compute("x = 1+1", JsValue::NULL).map_err(JsValue::from).unwrap();
    let res: Out = serde_wasm_bindgen::from_value(res).unwrap();
    assert_eq!(Primitive::Int(2), res.result);
    assert_eq!(1, res.ctx.len());
    assert_eq!(Primitive::Int(2), res.ctx["x"].read().unwrap().clone());
}
