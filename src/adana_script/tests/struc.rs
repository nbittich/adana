use std::collections::{BTreeMap, HashMap};

use serial_test::serial;

use crate::adana_script::{compute, Operator, Primitive, Value};

#[test]
#[serial]
fn test_simple_struc() {
    let mut ctx = BTreeMap::new();
    let expr = "x = struct {x: 8;}";
    let _ = compute(expr, &mut ctx).unwrap();
    assert_eq!(ctx.len(), 1);
    assert_eq!(
        ctx.get("x"),
        Some(&Primitive::Struct(HashMap::from([(
            "x".to_string(),
            Primitive::Int(8)
        )])))
    );
}

#[test]
#[serial]
fn test_simple_struc_with_more_stuff_in_it() {
    let mut ctx = BTreeMap::new();
    let expr = r#"x = struct {
                x: 8;
                y: "hello;";
                z: ()=> {println("hello")};
           }"#;
    let _ = compute(expr, &mut ctx).unwrap();
    assert_eq!(ctx.len(), 1);
    assert_eq!(
        ctx.get("x"),
        Some(&Primitive::Struct(HashMap::from([
            ("x".to_string(), Primitive::Int(8)),
            ("y".to_string(), Primitive::String("hello;".to_string())),
            (
                "z".to_string(),
                Primitive::Function {
                    parameters: vec![],
                    exprs: vec![Value::BlockParen(
                        vec![Value::BuiltInFunction {
                        fn_type:
                            crate::adana_script::BuiltInFunctionType::Println,
                        expr: Box::new(Value::BlockParen(vec![Value::String(
                            "hello".to_string()
                        )]))
                    }]
                    )]
                }
            )
        ])))
    );
}

#[test]
#[serial]
fn test_struct_eq() {
    let mut ctx = BTreeMap::new();
    let expr = r#"
        x = struct {
                x: 8;
                y: "hello;";
                z: ()=> {println("hello")};
            }
        y = struct {
          z: () => {println("hello")};
          x: 8;
          y: "hello;";
        }
        x == y
    "#;
    let res = compute(expr, &mut ctx).unwrap();
    assert_eq!(Primitive::Bool(true), res);

    let expr = r#"
        x = struct {
                x: 8;
                y: "hello;";
                z: ()=> {println("hello")};
            }
        y = struct {
          z: () => {println("hello")};
          x: 8;
        }
        x == y
    "#;
    let res = compute(expr, &mut ctx).unwrap();
    assert_eq!(Primitive::Bool(false), res);
}
