use std::collections::BTreeMap;

use crate::compute;
use adana_script_core::primitive::Primitive;
#[test]
fn simple_foreach_range() {
    let expr = r#"
         arr = [1,2,3,4]
         total = 0
         for a in 0..4 {
             total = total + arr[a]
         }
       "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(Primitive::Int(10), ctx["total"].read().unwrap().clone());
    assert!(!ctx.contains_key("a"));
}
#[test]
fn simple_range() {
    let expr = r#"
            arr = 1..5
         "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(
        Primitive::Array(vec![
            Primitive::Int(1),
            Primitive::Int(2),
            Primitive::Int(3),
            Primitive::Int(4),
        ]),
        ctx["arr"].read().unwrap().clone()
    );
    assert!(!ctx.contains_key("a"));
}
#[test]
fn simple_range_in_array() {
    let expr = r#"
            arr = [ 9, 1, 3, true, 1..5 ]
            arr2 = [ 9, 1, 3, true, 1..=5 ]
         "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(
        Primitive::Array(vec![
            Primitive::U8(9),
            Primitive::U8(1),
            Primitive::U8(3),
            Primitive::Bool(true),
            Primitive::Array(vec![
                Primitive::Int(1),
                Primitive::Int(2),
                Primitive::Int(3),
                Primitive::Int(4),
            ])
        ]),
        ctx["arr"].read().unwrap().clone()
    );
    assert_eq!(
        Primitive::Array(vec![
            Primitive::U8(9),
            Primitive::U8(1),
            Primitive::U8(3),
            Primitive::Bool(true),
            Primitive::Array(vec![
                Primitive::Int(1),
                Primitive::Int(2),
                Primitive::Int(3),
                Primitive::Int(4),
                Primitive::Int(5),
            ])
        ]),
        ctx["arr2"].read().unwrap().clone()
    );
}
#[test]
fn simple_range_in_function() {
    let expr = r#"
            x = () => {1..5}
            y = () => {1..=5}
            arr = x()
            arr2 = y()
         "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(
        Primitive::Array(vec![
            Primitive::Int(1),
            Primitive::Int(2),
            Primitive::Int(3),
            Primitive::Int(4),
        ]),
        ctx["arr"].read().unwrap().clone()
    );
    assert_eq!(
        Primitive::Array(vec![
            Primitive::Int(1),
            Primitive::Int(2),
            Primitive::Int(3),
            Primitive::Int(4),
            Primitive::Int(5),
        ]),
        ctx["arr2"].read().unwrap().clone()
    );
}
#[test]
fn simple_range_struct() {
    let expr = r#"
            s = struct {
                x: 2, #end
                a: 1..=4,#end
                b: "s",
                c: 4,
                d: 1..4
            }#end
         "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(
        Primitive::Struct(BTreeMap::from([
            (
                "a".to_string(),
                Primitive::Array(vec![
                    Primitive::Int(1),
                    Primitive::Int(2),
                    Primitive::Int(3),
                    Primitive::Int(4),
                ])
            ),
            ("b".into(), Primitive::String("s".into())),
            ("c".to_string(), Primitive::U8(4)),
            (
                "d".to_string(),
                Primitive::Array(vec![
                    Primitive::Int(1),
                    Primitive::Int(2),
                    Primitive::Int(3),
                ])
            ),
            ("x".to_string(), Primitive::U8(2)),
        ])),
        ctx["s"].read().unwrap().clone()
    );
}

#[test]
fn simple_foreach_range_both_end() {
    let expr = r#"
        arr = [1,2,3,4, 5]
         total = 0
         for a in 0..=5 {
             total = total + arr[a]
         }
       "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(Primitive::Int(15), ctx["total"].read().unwrap().clone());
    assert!(!ctx.contains_key("a"));
}
