use std::collections::BTreeMap;

use crate::adana_script::{compute, Primitive};

#[test]
fn test_simple() {
    let expr = r#"
           x = 99
           y = &x
        "#;
    let mut ctx = BTreeMap::new();

    let _ = compute(expr, &mut ctx).unwrap();
    assert_eq!(
        ctx["x"].read().unwrap().clone(),
        ctx["y"].read().unwrap().clone()
    );
    assert!(matches!(ctx["y"].read().unwrap().clone(), Primitive::Ref(_)));
}

#[test]
fn test_simple_modify() {
    let expr = r#"
           x = 99
           y = &x
           x = 100
        "#;
    let mut ctx = BTreeMap::new();

    let _ = compute(expr, &mut ctx).unwrap();
    assert_eq!(
        ctx["x"].read().unwrap().clone(),
        ctx["y"].read().unwrap().clone()
    );
    assert!(matches!(ctx["y"].read().unwrap().clone(), Primitive::Ref(_)));
    assert_eq!(ctx["y"].read().unwrap().clone(), Primitive::Int(100));
}
#[test]
fn test_simple_struct_ref() {
    let expr = r#"
           x = struct {
                 n: "hello",
                 f: "world"
           }
           y = &x
           p = y.f
        "#;
    let mut ctx = BTreeMap::new();

    let _ = compute(expr, &mut ctx).unwrap();
    assert_eq!(
        ctx["x"].read().unwrap().clone(),
        ctx["y"].read().unwrap().clone()
    );
    assert!(matches!(ctx["y"].read().unwrap().clone(), Primitive::Ref(_)));
    assert_eq!(
        ctx["p"].read().unwrap().clone(),
        Primitive::String("world".into())
    );
}
#[test]
fn test_simple_array_ref() {
    let expr = r#"
           x = ["hello", "world", 2]
           y = &x
           p = y[1]
        "#;
    let mut ctx = BTreeMap::new();

    let _ = compute(expr, &mut ctx).unwrap();
    assert_eq!(
        ctx["x"].read().unwrap().clone(),
        ctx["y"].read().unwrap().clone()
    );
    assert!(matches!(ctx["y"].read().unwrap().clone(), Primitive::Ref(_)));
    assert_eq!(
        ctx["p"].read().unwrap().clone(),
        Primitive::String("world".into())
    );
}
#[test]
fn test_forin_range_ref() {
    let expr = r#"
           x = 100 
           y = &x
           p = 0
           for _ in 0..&x {
               p = p+1
           }

        "#;
    let mut ctx = BTreeMap::new();

    let _ = compute(expr, &mut ctx).unwrap();
    assert_eq!(
        ctx["x"].read().unwrap().clone(),
        ctx["y"].read().unwrap().clone()
    );
    assert!(matches!(ctx["y"].read().unwrap().clone(), Primitive::Ref(_)));
    assert_eq!(ctx["p"].read().unwrap().clone(), Primitive::Int(100));
}

#[test]
fn test_forin_range_ref2() {
    let expr = r#"
           x = 100 
           y = &x
           p = 0
           for _ in 0..&y {
               p = p+1
           }

        "#;
    let mut ctx = BTreeMap::new();

    let _ = compute(expr, &mut ctx).unwrap();
    assert_eq!(
        ctx["x"].read().unwrap().clone(),
        ctx["y"].read().unwrap().clone()
    );
    assert!(matches!(ctx["y"].read().unwrap().clone(), Primitive::Ref(_)));
    assert_eq!(ctx["p"].read().unwrap().clone(), Primitive::Int(100));
}
