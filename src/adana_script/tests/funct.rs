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
