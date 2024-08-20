use std::collections::BTreeMap;

use crate::compute;
use adana_script_core::primitive::Primitive;
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
    let res = compute(expr, &mut ctx, "N/A").unwrap();

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
    let res = compute(expr, &mut ctx, "N/A").unwrap();

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
    let res = compute(expr, &mut ctx, "N/A").unwrap();

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
    let res = compute(expr, &mut ctx, "N/A").unwrap();

    assert_eq!(
        ctx["x"].read().unwrap().clone(),
        Primitive::String("hello world".into())
    );
    assert_eq!(res, Primitive::String("hello world".into()));
}

#[test]
fn complex_struct_struct_struct_other() {
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
        x = x.a() + x.y.sp + x.y.z.m()

        "#;
    let mut ctx = BTreeMap::new();
    let res = compute(expr, &mut ctx, "N/A").unwrap();

    assert_eq!(
        ctx["x"].read().unwrap().clone(),
        Primitive::String("hello world".into())
    );
    assert_eq!(res, Primitive::String("hello world".into()));
}

#[test]
fn simple_array_two_depth() {
    let expr = r#"
             z = [0, 2, "hello", [3," ", "world"]]
             x = z[2] + z[3][1] + z[3][2] 
        "#;
    let mut ctx = BTreeMap::new();
    let res = compute(expr, &mut ctx, "N/A").unwrap();

    assert_eq!(
        ctx["x"].read().unwrap().clone(),
        Primitive::String("hello world".into())
    );
    assert_eq!(res, Primitive::String("hello world".into()));
}
