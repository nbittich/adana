use std::collections::BTreeMap;

use crate::adana_script::{compute, Primitive};

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
    let _ = compute(expr, &mut ctx).unwrap();
    assert_eq!(Primitive::Int(10), ctx["total"].read().unwrap().clone());
    assert!(ctx.get("a").is_none());
}
#[test]
fn simple_range() {
    let expr = r#"
            arr = 1..5
         "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(expr, &mut ctx).unwrap();
    assert_eq!(
        Primitive::Array(vec![
            Primitive::Int(1),
            Primitive::Int(2),
            Primitive::Int(3),
            Primitive::Int(4),
        ]),
        ctx["arr"].read().unwrap().clone()
    );
    assert!(ctx.get("a").is_none());
}
#[test]
fn simple_range_struct() {
    let expr = r#"
            s = struct {
                x: 2; #end
                a: [1,2,3,4]; #end
            }#end
         "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(expr, &mut ctx).unwrap();
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
            ("x".to_string(), Primitive::Int(2)),
        ])),
        ctx["s"].read().unwrap().clone()
    );
}
