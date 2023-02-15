use std::collections::BTreeMap;

use crate::adana_script::{compute, Primitive};

#[test]
fn complex_struct_array() {
    let expr = r#"
         x = struct {
             m: 12,
             y: ["hello", 3, "world"]
         }
        x = x.y[0]  + " " + x.y[2]

        "#;
    let mut ctx = BTreeMap::new();
    let res = compute(expr, &mut ctx).unwrap();

    assert_eq!(
        ctx["x"].read().unwrap().clone(),
        Primitive::String("hello world".into())
    );
    assert_eq!(res, Primitive::String("hello world".into()));
}
#[test]
fn complex_struct_array_struct() {
    let expr = r#"
         x = struct {
             m: 12,
             y: ["hello", 3, struct {n: "world"}]
         }
        x = x.y[0]  + " " + x.y[2]["n"]

        "#;
    let mut ctx = BTreeMap::new();
    let res = compute(expr, &mut ctx).unwrap();

    assert_eq!(
        ctx["x"].read().unwrap().clone(),
        Primitive::String("hello world".into())
    );
    assert_eq!(res, Primitive::String("hello world".into()));
}

#[test]
fn complex_struct_struct_struct() {
    let expr = r#"
         x = struct {
             m: 12,
             a: "hello",
             y: struct {
                 sp: " ",
                 z: struct {
                     m: "world"
                 }
             } 
         }
        x = x.a + x.y.sp + x.y.z.m 

        "#;
    let mut ctx = BTreeMap::new();
    let res = compute(expr, &mut ctx).unwrap();

    assert_eq!(
        ctx["x"].read().unwrap().clone(),
        Primitive::String("hello world".into())
    );
    assert_eq!(res, Primitive::String("hello world".into()));
}

#[test]
fn complex_struct_struct_struct_fn() {
    let expr = r#"
         x = struct {
             m: 12,
             a: "hello",
             y: struct {
                 sp: " ",
                 z: struct {
                     m: () => {"world"}
                 }
             } 
         }
        x = x.a + x.y.sp + x.y.z.m() 

        "#;
    let mut ctx = BTreeMap::new();
    let res = compute(expr, &mut ctx).unwrap();

    assert_eq!(
        ctx["x"].read().unwrap().clone(),
        Primitive::String("hello world".into())
    );
    assert_eq!(res, Primitive::String("hello world".into()));
}

#[test]
fn complex_struct_struct_struct_fn2() {
    let expr = r#"
         x = struct {
             m: 12,
             a:()=> {"hello"},
             y: struct {
                 sp: " ",
                 z: struct {
                     m: () => {"world"}
                 }
             } 
         }
        x = multiline {x.a() + x.y.sp + x.y.z.m()}

        "#;
    let mut ctx = BTreeMap::new();
    let res = compute(expr, &mut ctx).unwrap();

    assert_eq!(
        ctx["x"].read().unwrap().clone(),
        Primitive::String("hello world".into())
    );
    assert_eq!(res, Primitive::String("hello world".into()));
}
