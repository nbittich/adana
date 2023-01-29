use std::collections::BTreeMap;

use crate::adana_script::{compute, Primitive};

#[test]
fn test_builtin_to_int() {
    let mut ctx = BTreeMap::new();
    let res = compute("to_int(2)", &mut ctx).unwrap();
    assert_eq!(res, Primitive::Int(2));
    let res = compute(r#"to_int("2")"#, &mut ctx).unwrap();
    assert_eq!(res, Primitive::Int(2));
    ctx.insert("a".to_string(), Primitive::String("123".to_string()));
    let res = compute("to_int(a)", &mut ctx).unwrap();
    assert_eq!(res, Primitive::Int(123));
}

#[test]
fn test_builtin_to_double() {
    let mut ctx = BTreeMap::new();
    let res = compute("to_double(2)", &mut ctx).unwrap();
    assert_eq!(res, Primitive::Double(2.0));
    let res = compute(r#"to_double("2.1")"#, &mut ctx).unwrap();
    assert_eq!(res, Primitive::Double(2.1));
}

#[test]
fn test_builtin_to_bool() {
    let mut ctx = BTreeMap::new();
    let res = compute("to_bool(2)", &mut ctx).unwrap();
    assert_eq!(res, Primitive::Bool(true));
    let res = compute(r#"to_bool("false")"#, &mut ctx).unwrap();
    assert_eq!(res, Primitive::Bool(false));
    ctx.insert("a".to_string(), Primitive::Double(0.0));
    let res = compute("to_bool(a)", &mut ctx).unwrap();
    assert_eq!(res, Primitive::Bool(false));
}

#[test]
fn test_eval() {
    let mut ctx = BTreeMap::new();
    let _ = compute(r#"eval("z = sqrt(9)")"#, &mut ctx).unwrap();
    assert_eq!(ctx.get("z"), Some(&Primitive::Double(3.0)));
}
