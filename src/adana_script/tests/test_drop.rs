use std::collections::BTreeMap;

use crate::adana_script::{compute, BuiltInFunctionType, Primitive, Value};

#[test]
fn test_drop_arr_access() {
    let exp = r#"
    arr = [1,2,3,4]
    drop(arr[2])
    "#;
    let mut ctx = BTreeMap::new();

    let _ = compute(exp, &mut ctx).unwrap();
    assert_eq!(
        ctx["arr"].read().unwrap().clone(),
        Primitive::Array(vec![
            Primitive::Int(1),
            Primitive::Int(2),
            Primitive::Int(4),
        ])
    );
}
#[test]
fn test_drop_string() {
    let exp = r#"
    arr = "hello"
    drop(arr[0])
    "#;
    let mut ctx = BTreeMap::new();

    let _ = compute(exp, &mut ctx).unwrap();
    assert_eq!(
        ctx["arr"].read().unwrap().clone(),
        Primitive::String("ello".to_string())
    );
}
#[test]
fn test_drop_struct_access() {
    let exp = r#"
    s = struct {
        x: (_, n) => {println(n)},
        y: "hello",
        z: "world"
    }
    drop(s["z"])
    "#;
    let mut ctx = BTreeMap::new();

    let _ = compute(exp, &mut ctx).unwrap();
    assert_eq!(
        ctx["s"].read().unwrap().clone(),
        Primitive::Struct(BTreeMap::from([
            (
                "x".to_string(),
                Primitive::Function {
                    parameters: vec![
                        Value::VariableUnused,
                        Value::Variable("n".to_string(),),
                    ],
                    exprs: vec![Value::BlockParen(vec![
                        Value::BuiltInFunction {
                            fn_type: BuiltInFunctionType::Println,
                            expr: Box::new(Value::BlockParen(vec![
                                Value::Variable("n".to_string(),),
                            ],)),
                        },
                    ],),],
                }
            ),
            ("y".to_string(), Primitive::String("hello".to_string(),)),
        ]),)
    );
}
#[test]
fn test_drop_struct_access_alt() {
    let exp = r#"
    s = struct {
        x: (_, n) => {println(n)},
        y: "hello",
        z: "world"
    }
    drop(s.z)
    "#;
    let mut ctx = BTreeMap::new();

    let _ = compute(exp, &mut ctx).unwrap();
    assert_eq!(
        ctx["s"].read().unwrap().clone(),
        Primitive::Struct(BTreeMap::from([
            (
                "x".to_string(),
                Primitive::Function {
                    parameters: vec![
                        Value::VariableUnused,
                        Value::Variable("n".to_string(),),
                    ],
                    exprs: vec![Value::BlockParen(vec![
                        Value::BuiltInFunction {
                            fn_type: BuiltInFunctionType::Println,
                            expr: Box::new(Value::BlockParen(vec![
                                Value::Variable("n".to_string(),),
                            ],)),
                        },
                    ],),],
                }
            ),
            ("y".to_string(), Primitive::String("hello".to_string(),)),
        ]),)
    );
}
