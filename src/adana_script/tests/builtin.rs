use std::collections::BTreeMap;

use crate::adana_script::{compute, Primitive};

#[test]
fn test_builtin_to_int() {
    let mut ctx = BTreeMap::new();
    let res = compute("to_int(2)", &mut ctx).unwrap();
    assert_eq!(res, Primitive::Int(2));
    let res = compute(r#"to_int("2")"#, &mut ctx).unwrap();
    assert_eq!(res, Primitive::Int(2));
    ctx.insert(
        "a".to_string(),
        Primitive::String("123".to_string()).mut_prim(),
    );
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
    ctx.insert("a".to_string(), Primitive::Double(0.0).mut_prim());
    let res = compute("to_bool(a)", &mut ctx).unwrap();
    assert_eq!(res, Primitive::Bool(false));
}

#[test]
fn test_eval() {
    let mut ctx = BTreeMap::new();
    let _ = compute(r#"eval("z = sqrt(9)")"#, &mut ctx).unwrap();
    assert_eq!(*ctx["z"].lock().unwrap(), Primitive::Double(3.0));
}

#[test]
fn test_type_of() {
    let mut ctx = BTreeMap::new();
    ctx.insert("x".to_string(), Primitive::Int(3).mut_prim());

    ctx.insert("y".to_string(), Primitive::Double(3.).mut_prim());
    ctx.insert(
        "z".to_string(),
        Primitive::Function { parameters: vec![], exprs: vec![] }.mut_prim(),
    );
    ctx.insert("a".to_string(), Primitive::Error("err".to_string()).mut_prim());
    ctx.insert("b".to_string(), Primitive::Array(vec![]).mut_prim());
    ctx.insert("c".to_string(), Primitive::Bool(true).mut_prim());
    ctx.insert("d".to_string(), Primitive::String("a".to_string()).mut_prim());
    ctx.insert("e".to_string(), Primitive::Unit.mut_prim());
    ctx.insert("f".to_string(), Primitive::NoReturn.mut_prim());
    ctx.insert(
        "g".to_string(),
        Primitive::EarlyReturn(Box::new(Primitive::Int(1))).mut_prim(),
    );
    assert_eq!(
        Primitive::String("int".to_string()),
        compute(r#"type_of(x)"#, &mut ctx).unwrap()
    );
    assert_eq!(
        Primitive::String("double".to_string()),
        compute(r#"type_of(y)"#, &mut ctx).unwrap()
    );
    assert_eq!(
        Primitive::String("function".to_string()),
        compute(r#"type_of(z)"#, &mut ctx).unwrap()
    );
    assert_eq!(
        Primitive::String("error".to_string()),
        compute(r#"type_of(a)"#, &mut ctx).unwrap()
    );
    assert_eq!(
        Primitive::String("array".to_string()),
        compute(r#"type_of(b)"#, &mut ctx).unwrap()
    );
    assert_eq!(
        Primitive::String("bool".to_string()),
        compute(r#"type_of(c)"#, &mut ctx).unwrap()
    );
    assert_eq!(
        Primitive::String("string".to_string()),
        compute(r#"type_of(d)"#, &mut ctx).unwrap()
    );
    assert_eq!(
        Primitive::String("unit".to_string()),
        compute(r#"type_of(e)"#, &mut ctx).unwrap()
    );
    assert_eq!(
        Primitive::String("!".to_string()),
        compute(r#"type_of(f)"#, &mut ctx).unwrap()
    );
    assert_eq!(
        Primitive::String("int".to_string()),
        compute(r#"type_of(g)"#, &mut ctx).unwrap()
    );
}
