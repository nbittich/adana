use std::collections::BTreeMap;

use crate::compute;
use adana_script_core::primitive::Primitive;
#[test]
fn test_builtin_to_int() {
    let mut ctx = BTreeMap::new();
    let res = compute("to_int(2)", &mut ctx, "N/A").unwrap();
    assert_eq!(res, Primitive::U8(2));

    let res = compute("to_int(256)", &mut ctx, "N/A").unwrap();
    assert_eq!(res, Primitive::Int(256));

    let res = compute(r#"to_int("2")"#, &mut ctx, "N/A").unwrap();
    assert_eq!(res, Primitive::Int(2));

    ctx.insert(
        "a".to_string(),
        Primitive::String("123".to_string()).ref_prim(),
    );
    let res = compute("to_int(a)", &mut ctx, "N/A").unwrap();
    assert_eq!(res, Primitive::Int(123));
}

#[test]
fn test_builtin_to_double() {
    let mut ctx = BTreeMap::new();
    let res = compute("to_double(2)", &mut ctx, "N/A").unwrap();
    assert_eq!(res, Primitive::Double(2.0));
    let res = compute(r#"to_double("2.1")"#, &mut ctx, "N/A").unwrap();
    assert_eq!(res, Primitive::Double(2.1));
}

#[test]
fn test_builtin_to_bool() {
    let mut ctx = BTreeMap::new();
    let res = compute("to_bool(2)", &mut ctx, "N/A").unwrap();
    assert_eq!(res, Primitive::Bool(true));
    let res = compute(r#"to_bool("false")"#, &mut ctx, "N/A").unwrap();
    assert_eq!(res, Primitive::Bool(false));
    ctx.insert("a".to_string(), Primitive::Double(0.0).ref_prim());
    let res = compute("to_bool(a)", &mut ctx, "N/A").unwrap();
    assert_eq!(res, Primitive::Bool(false));
}

#[test]
fn test_eval() {
    let mut ctx = BTreeMap::new();
    let _ = compute(r#"eval("z = sqrt(9)")"#, &mut ctx, "N/A").unwrap();
    assert_eq!(*ctx["z"].read().unwrap(), Primitive::Double(3.0));
}

#[test]
fn test_to_hex() {
    let mut ctx = BTreeMap::new();
    let r = compute(r#"to_hex(255)"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::String("0xff".into()));
    let r = compute(r#"to_hex(1)"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::String("0x1".into()));
    let r = compute(r#"to_hex(1024)"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::String("0x400".into()));
}

#[test]
fn test_to_binary() {
    let mut ctx = BTreeMap::new();
    let r = compute(r#"to_binary(255)"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::String("0b11111111".into()));
    let r = compute(r#"to_binary(-127)"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::String("0b10000001".into()));
    let r = compute(r#"to_binary(127)"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::String("0b1111111".into()));
}

#[test]
fn test_is_match() {
    let mut ctx = BTreeMap::new();
    let r = compute(
        r#"
      pattern = """(?i)a+(?-i)b+"""
      text = "AaAaAbbBBBb"
      is_match(text, pattern)
 
    "#,
        &mut ctx,
        "N/A",
    )
    .unwrap();
    assert_eq!(Primitive::Bool(true), r);
    let mut ctx = BTreeMap::new();
    let r = compute(
        r#"
      pattern = """(\w+): \$(\d+)"""
      text = "Item1: $100, Item2: $200, Item3: $300"
      is_match(text, pattern)
 
    "#,
        &mut ctx,
        "N/A",
    )
    .unwrap();
    assert_eq!(Primitive::Bool(true), r);
}
#[test]
fn test_match() {
    let mut ctx = BTreeMap::new();
    let r = compute(
        r#"
      pattern = """(?i)a+(?-i)b+"""
      text = "AaAaAbbBBBb"
      match(text, pattern)
 
    "#,
        &mut ctx,
        "N/A",
    )
    .unwrap();
    assert_eq!(
        Primitive::Array(vec![Primitive::String("AaAaAbb".to_string())]),
        r
    );
    let mut ctx = BTreeMap::new();
    let r = compute(
        r#"
      pattern = """(\w+): \$(\d+)"""
      text = "Item1: $100, Item2: $200, Item3: $300"
      match(text, pattern)
 
    "#,
        &mut ctx,
        "N/A",
    )
    .unwrap();
    assert_eq!(
        Primitive::Array(vec![
            Primitive::Array(vec![
                Primitive::String("Item1: $100".to_string()),
                Primitive::String("Item1".to_string()),
                Primitive::String("100".to_string())
            ]),
            Primitive::Array(vec![
                Primitive::String("Item2: $200".to_string()),
                Primitive::String("Item2".to_string()),
                Primitive::String("200".to_string())
            ]),
            Primitive::Array(vec![
                Primitive::String("Item3: $300".to_string()),
                Primitive::String("Item3".to_string()),
                Primitive::String("300".to_string())
            ])
        ]),
        r
    );
}

#[test]
fn test_type_of() {
    let mut ctx = BTreeMap::new();
    ctx.insert("x".to_string(), Primitive::Int(3).ref_prim());

    ctx.insert("y".to_string(), Primitive::Double(3.).ref_prim());
    ctx.insert(
        "z".to_string(),
        Primitive::Function { parameters: vec![], exprs: vec![] }.ref_prim(),
    );
    ctx.insert("a".to_string(), Primitive::Error("err".to_string()).ref_prim());
    ctx.insert("b".to_string(), Primitive::Array(vec![]).ref_prim());
    ctx.insert("c".to_string(), Primitive::Bool(true).ref_prim());
    ctx.insert("d".to_string(), Primitive::String("a".to_string()).ref_prim());
    ctx.insert("e".to_string(), Primitive::Unit.ref_prim());
    ctx.insert("f".to_string(), Primitive::NoReturn.ref_prim());
    ctx.insert(
        "g".to_string(),
        Primitive::EarlyReturn(Box::new(Primitive::Int(1))).ref_prim(),
    );
    assert_eq!(
        Primitive::String("int".to_string()),
        compute(r#"type_of(x)"#, &mut ctx, "N/A",).unwrap()
    );
    assert_eq!(
        Primitive::String("double".to_string()),
        compute(r#"type_of(y)"#, &mut ctx, "N/A",).unwrap()
    );
    assert_eq!(
        Primitive::String("function".to_string()),
        compute(r#"type_of(z)"#, &mut ctx, "N/A",).unwrap()
    );
    assert_eq!(
        Primitive::String("error".to_string()),
        compute(r#"type_of(a)"#, &mut ctx, "N/A",).unwrap()
    );
    assert_eq!(
        Primitive::String("array".to_string()),
        compute(r#"type_of(b)"#, &mut ctx, "N/A",).unwrap()
    );
    assert_eq!(
        Primitive::String("bool".to_string()),
        compute(r#"type_of(c)"#, &mut ctx, "N/A",).unwrap()
    );
    assert_eq!(
        Primitive::String("string".to_string()),
        compute(r#"type_of(d)"#, &mut ctx, "N/A",).unwrap()
    );
    assert_eq!(
        Primitive::String("unit".to_string()),
        compute(r#"type_of(e)"#, &mut ctx, "N/A",).unwrap()
    );
    assert_eq!(
        Primitive::String("!".to_string()),
        compute(r#"type_of(f)"#, &mut ctx, "N/A",).unwrap()
    );
    assert_eq!(
        Primitive::String("int".to_string()),
        compute(r#"type_of(g)"#, &mut ctx, "N/A",).unwrap()
    );
}
