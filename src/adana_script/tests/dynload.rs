use std::collections::BTreeMap;

use adana_script_core::primitive::Primitive;
use serial_test::serial;

use crate::adana_script::compute;

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
