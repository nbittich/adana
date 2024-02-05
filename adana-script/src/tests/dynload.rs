use std::collections::BTreeMap;

use adana_script_core::primitive::Primitive;
use serial_test::serial;

use crate::compute;

#[test]
#[serial]
fn load_dynamic_lib_test() {
    let file_path = r#"
     lib = require("libplugin_example.so")
     text = lib.hello("Nordine", "la", "forme?")
    "#;
    let mut ctx = BTreeMap::new();
    let res = compute(file_path, &mut ctx, "dynamic_lib").unwrap();

    assert_eq!(
        Primitive::String("Hello Nordine la forme?".to_string()),
        ctx["text"].read().unwrap().clone()
    );

    dbg!(ctx);
    println!("{res:?}");
}
#[test]
#[serial]
fn callback_dynamic_lib_test() {
    let file_path = r#"
     lib = require("libplugin_example.so")
     callback = (input) => {input + " Nordine! ca va?"}
     text = lib.callback(callback)
    "#;
    let mut ctx = BTreeMap::new();
    let res = compute(file_path, &mut ctx, "dynamic_lib").unwrap();

    assert_eq!(
        Primitive::String("Hello Nordine! ca va?".to_string()),
        ctx["text"].read().unwrap().clone()
    );

    dbg!(ctx);
    println!("{res:?}");
}

#[test]
#[serial]
fn complex_callback_dynamic_lib_test() {
    let file_path = r#"
     lib = require("libplugin_example.so")
     callback = (input) => {lib.hello(input,"Nordine!","ca", "va?")}
     text = lib.callback(callback)
    "#;
    let mut ctx = BTreeMap::new();
    let res = compute(file_path, &mut ctx, "dynamic_lib").unwrap();

    assert_eq!(
        Primitive::String("Hello Hello Nordine! ca va?".to_string()),
        ctx["text"].read().unwrap().clone()
    );

    dbg!(ctx);
    println!("{res:?}");
}

#[test]
#[serial]
fn build_from_adana_dynamic_lib_test() {
    let file_path = r#"
     lib = require("example_lib_src")
     callback = (input) => {lib.hello(input,"Nordine!","ca", "va?")}
     text = lib.callback(callback)
    "#;
    let mut ctx = BTreeMap::new();
    let res = compute(file_path, &mut ctx, "dynamic_lib").unwrap();

    assert_eq!(
        Primitive::String("Hello Hello Nordine! ca va?".to_string()),
        ctx["text"].read().unwrap().clone()
    );

    dbg!(ctx);
    println!("{res:?}");
}
#[test]
#[serial]
fn require_direct_call_test() {
    let mut ctx = BTreeMap::new();

    let q = r#"
    text = require("example_lib_src").hello("Nordine", "la", "forme?")
    "#;
    let res = compute(q, &mut ctx, "dynamic_lib").unwrap();

    assert_eq!(
        Primitive::String("Hello Nordine la forme?".to_string()),
        ctx["text"].read().unwrap().clone()
    );

    dbg!(ctx);
    println!("{res:?}");
}
