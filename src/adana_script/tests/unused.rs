use std::collections::BTreeMap;

use crate::adana_script::{compute, Primitive};

#[test]
fn test_simple_unused_array() {
    let expr = r#"
            _ = [1,2,3,4]
        "#;
    let mut ctx = BTreeMap::new();
    let r = compute(expr, &mut ctx).unwrap();

    assert!(ctx.is_empty());
    assert_eq!(
        r,
        Primitive::Array(vec![
            Primitive::Int(1),
            Primitive::Int(2),
            Primitive::Int(3),
            Primitive::Int(4),
        ])
    );
}
#[test]
fn test_simple_unused_range() {
    let expr = r#"
            _ = 1..=4
        "#;
    let mut ctx = BTreeMap::new();
    let r = compute(expr, &mut ctx).unwrap();

    assert!(ctx.is_empty());
    assert_eq!(
        r,
        Primitive::Array(vec![
            Primitive::Int(1),
            Primitive::Int(2),
            Primitive::Int(3),
            Primitive::Int(4),
        ])
    );
}

#[test]
fn test_simple_unused_fn_call_range() {
    let expr = r#"
            _ = (n) => { 1..=n }(4)
        "#;
    let mut ctx = BTreeMap::new();
    let r = compute(expr, &mut ctx).unwrap();

    assert!(ctx.is_empty());
    assert_eq!(
        r,
        Primitive::Array(vec![
            Primitive::Int(1),
            Primitive::Int(2),
            Primitive::Int(3),
            Primitive::Int(4),
        ])
    );
}

#[test]
fn test_simple_unused_struct() {
    let expr = r#"
            _ = struct {
                _ : "this will not be available within context struct!", # because I said so
                _special: "also you",
                arr: 1..=4,
                _and_you: 3 # com
            } # more comments
        "#;
    let mut ctx = BTreeMap::new();
    let r = compute(expr, &mut ctx).unwrap();

    assert!(ctx.is_empty());
    assert_eq!(
        r,
        Primitive::Struct(BTreeMap::from([(
            "arr".into(),
            Primitive::Array(vec![
                Primitive::Int(1),
                Primitive::Int(2),
                Primitive::Int(3),
                Primitive::Int(4),
            ])
        )]))
    );
}

#[test]
fn test_simple_unused_foreach() {
    let expr = r#"
            arr = []
            for _, n in 1..=4 {
                 arr = arr + n
            }

        "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(expr, &mut ctx).unwrap();

    assert_eq!(ctx.len(), 1);
    assert_eq!(
        ctx["arr"].read().unwrap().clone(),
        Primitive::Array(vec![
            Primitive::Int(1),
            Primitive::Int(2),
            Primitive::Int(3),
            Primitive::Int(4),
        ])
    );
}

#[test]
fn test_simple_unused_foreach_index() {
    let expr = r#"
            arr = []
            for i, _ in 7..=10 {
                 arr = arr + (i + 1)
            }

        "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(expr, &mut ctx).unwrap();

    assert_eq!(ctx.len(), 1);
    assert_eq!(
        ctx["arr"].read().unwrap().clone(),
        Primitive::Array(vec![
            Primitive::Int(1),
            Primitive::Int(2),
            Primitive::Int(3),
            Primitive::Int(4),
        ])
    );
}
