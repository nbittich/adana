use std::collections::BTreeMap;

use crate::adana_script::{compute, Operator, Primitive, Value};

#[test]
fn test_anon_func_call() {
    let mut ctx = BTreeMap::new();
    let s = r#"
        (a,b) => {
            a + b
        }(2, 3)
    "#;

    let res = compute(s, &mut ctx).unwrap();
    assert_eq!(Primitive::Int(5), res);
    let s = r#"
        z = (a,b) => {
            c= 4
            a + b * c
        }
        z(2,3)
    "#;

    let res = compute(s, &mut ctx).unwrap();
    assert_eq!(Primitive::Int(14), res);

    assert_eq!(
        ctx,
        BTreeMap::from([(
            "z".to_string(),
            Primitive::Function {
                parameters: vec!["a".to_string(), "b".to_string(),],
                exprs: vec![
                    Value::VariableExpr {
                        name: Box::new(Value::Variable("c".to_string(),)),
                        expr: Box::new(Value::Expression(vec![
                            Value::Integer(4,),
                        ],)),
                    },
                    Value::Expression(vec![
                        Value::Variable("a".to_string(),),
                        Value::Operation(Operator::Add,),
                        Value::Variable("b".to_string(),),
                        Value::Operation(Operator::Mult,),
                        Value::Variable("c".to_string(),),
                    ],),
                ],
            },
        )])
    );

    let s = r#"
        a = 2
        b = 3
        (a,b) => {
            a + b
        }(a, b)
    "#;

    let res = compute(s, &mut ctx).unwrap();
    assert_eq!(Primitive::Int(5), res);

    let s = r#"
        a = 2
        b = 3
        c = 5
        (a,b, c) => {
            d = a + b
            while(c != 0) {
                d = d * c
                c = c-1
            }
            d
        }(a, b, c)
    "#;

    let res = compute(s, &mut ctx).unwrap();
    assert_eq!(Primitive::Int(600), res);
}

#[test]
#[serial_test::serial]
fn test_basic_map() {
    let script = r#"
        include("file_tests/test_fn.adana")
        m = map()
        m = push_v("nordine", 34, m)
        get_v("nordine", m)
    "#;
    let mut ctx = BTreeMap::new();

    let res = compute(script, &mut ctx).unwrap();
    assert_eq!(Primitive::Int(34), res);
    let script = r#"
        include("file_tests/test_fn.adana")
        m = map()
        m = push_v("nordine", 34, m)
        get_v("nordines", m)
    "#;
    let mut ctx = BTreeMap::new();

    let res = compute(script, &mut ctx).unwrap();
    assert_eq!(Primitive::Array(vec![]), res); // todo change that with null or smth
}
#[test]
#[serial_test::serial]
fn test_override_map() {
    let script = r#"
        include("file_tests/test_fn.adana")
        m = map()
        m = push_v("nordine", 34, m)
        m = push_v("nordine", 35, m)
        get_v("nordine", m)
    "#;
    let mut ctx = BTreeMap::new();

    let res = compute(script, &mut ctx).unwrap();
    assert_eq!(Primitive::Int(35), res);

    assert_eq!(
        ctx.get("m"),
        Some(&Primitive::Array(vec![Primitive::Array(vec![
            Primitive::String("nordine".to_string(),),
            Primitive::Int(35,),
        ],),],),)
    );
}
