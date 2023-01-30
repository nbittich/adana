use std::collections::HashMap;

use crate::adana_script::{
    parser::{parse_instructions, parse_struct},
    BuiltInFunctionType, Operator,
    Value::{
        self, BlockParen, Expression, Function, Integer, Operation, Variable,
        VariableExpr, WhileExpr,
    },
};

#[test]
fn test_parse_multiline() {
    let (rest, _result) = parse_instructions(
        r#"
    multiline 
    {
        2*(3/4.-12%5 +7^9) -6/12.*4 / 
        sqrt(2*(3/4.-12%5 +7^9) --6/12.*4) + 
        abs(-2*(3/4.-12%5 +7^9) -6/12.*4 / sqrt(5))
    }
    "#,
    )
    .unwrap();
    assert!(rest.is_empty());
}
#[test]
fn test_parse_fn() {
    let (rest, result) = parse_instructions(
        r#"
        z = (x) => {
            x = 0
         }
    "#,
    )
    .unwrap();
    assert_eq!(
        result,
        vec![Value::VariableExpr {
            name: Box::new(Value::Variable("z".to_string(),)),
            expr: Box::new(Value::Function {
                parameters: Box::new(Value::BlockParen(vec![Value::Variable(
                    "x".to_string(),
                ),],)),
                exprs: vec![Value::VariableExpr {
                    name: Box::new(Value::Variable("x".to_string(),)),
                    expr: Box::new(Value::Expression(
                        vec![Value::Integer(0,),],
                    )),
                },],
            }),
        },]
    );
    assert!(rest.trim().is_empty());
    let (rest, result) = parse_instructions(
        r#"
         (x, y) => {
            x = 0
         }
    "#,
    )
    .unwrap();
    assert_eq!(
        result,
        vec![Value::Function {
            parameters: Box::new(Value::BlockParen(vec![
                Value::Variable("x".to_string(),),
                Value::Variable("y".to_string(),),
            ],)),
            exprs: vec![Value::VariableExpr {
                name: Box::new(Value::Variable("x".to_string(),)),
                expr: Box::new(Value::Expression(vec![Value::Integer(0,),],)),
            },],
        },]
    );
    assert!(rest.trim().is_empty());
    let (rest, result) = parse_instructions(
        r#"
         (x, y) => {
            x = 0
            println("hello")
         }
    "#,
    )
    .unwrap();
    assert_eq!(
        result,
        vec![Value::Function {
            parameters: Box::new(Value::BlockParen(vec![
                Value::Variable("x".to_string(),),
                Value::Variable("y".to_string(),),
            ],)),
            exprs: vec![
                Value::VariableExpr {
                    name: Box::new(Value::Variable("x".to_string(),)),
                    expr: Box::new(Value::Expression(
                        vec![Value::Integer(0,),],
                    )),
                },
                Value::Expression(vec![Value::BuiltInFunction {
                    fn_type: BuiltInFunctionType::Println,
                    expr: Box::new(Value::BlockParen(vec![Value::String(
                        format!("hello")
                    )]))
                }])
            ],
        },]
    );
    assert!(rest.trim().is_empty());
}

#[test]
fn test_parse_break() {
    let s = r#"
    index_of_v = (key, m) => {
        count = 0
        res = -1
         while(count < length(m)) {
            k = m[count]
            if(k[0] == key) {
                res = count
                break
            } else {
                count = count +1
            }
        }
        res
    }
    "#;

    let (_, v) = parse_instructions(s).unwrap();

    assert_eq!(
        v,
        vec![VariableExpr {
            name: Box::new(Variable("index_of_v".to_string(),)),
            expr: Box::new(Function {
                parameters: Box::new(BlockParen(vec![
                    Variable("key".to_string(),),
                    Variable("m".to_string(),),
                ],)),
                exprs: vec![
                    VariableExpr {
                        name: Box::new(Variable("count".to_string(),)),
                        expr: Box::new(Expression(vec![Integer(0,),],),)
                    },
                    VariableExpr {
                        name: Box::new(Variable("res".to_string(),)),
                        expr: Box::new(Expression(vec![
                            Operation(Operator::Subtr,),
                            Integer(1,),
                        ],),)
                    },
                    WhileExpr {
                        cond: Box::new(BlockParen(vec![
                            Variable("count".to_string(),),
                            Operation(Operator::Less,),
                            Value::BuiltInFunction {
                                fn_type: BuiltInFunctionType::Length,
                                expr: Box::new(BlockParen(vec![Variable(
                                    "m".to_string(),
                                ),],)),
                            },
                        ],),),
                        exprs: vec![
                            VariableExpr {
                                name: Box::new(Variable("k".to_string(),)),
                                expr: Box::new(Expression(vec![
                                    Value::ArrayAccess {
                                        arr: Box::new(Variable(
                                            "m".to_string(),
                                        )),
                                        index: Box::new(Variable(
                                            "count".to_string(),
                                        )),
                                    },
                                ],)),
                            },
                            Value::IfExpr {
                                cond: Box::new(BlockParen(vec![
                                    Value::ArrayAccess {
                                        arr: Box::new(Variable(
                                            "k".to_string(),
                                        )),
                                        index: Box::new(Integer(0,)),
                                    },
                                    Operation(Operator::Equal,),
                                    Variable("key".to_string(),),
                                ],),),
                                exprs: vec![
                                    VariableExpr {
                                        name: Box::new(Variable(
                                            "res".to_string(),
                                        )),
                                        expr: Box::new(Expression(vec![
                                            Variable("count".to_string(),),
                                        ],)),
                                    },
                                    Value::Break,
                                ],
                                else_expr: Some(vec![VariableExpr {
                                    name: Box::new(Variable(
                                        "count".to_string(),
                                    )),
                                    expr: Box::new(Expression(vec![
                                        Variable("count".to_string(),),
                                        Operation(Operator::Add,),
                                        Integer(1,),
                                    ],)),
                                },],),
                            },
                        ],
                    },
                    Expression(vec![Variable("res".to_string(),),],),
                ],
            },)
        },],
    )
}

#[test]
fn test_paren_bug_2023() {
    let expr = "(4*3 * (2+2))";
    let (res, instructions) = parse_instructions(expr).unwrap();
    assert_eq!("", res);
    assert_eq!(
        instructions,
        vec![Value::Expression(vec![Value::BlockParen(vec![
            Value::Integer(4,),
            Value::Operation(Operator::Mult,),
            Value::Integer(3,),
            Value::Operation(Operator::Mult,),
            Value::BlockParen(vec![
                Value::Integer(2,),
                Value::Operation(Operator::Add,),
                Value::Integer(2,),
            ],),
        ],),],),]
    );
    dbg!(&res);
}

#[test]
fn test_struct() {
    let expr = "struct {x:= 99}";
    let (res, struc) = parse_instructions(expr).unwrap();
    assert_eq!("", res);
    assert_eq!(
        vec![Value::Struct(HashMap::from([("x".into(), Value::Integer(99))]))],
        struc
    );
    let expr = r#"
        # commentaire
      my = struct { # commentaire
          # ici un commentaire
            a := 7;
            b := "salut"; # i am a comment
            c := [1,2,3];
      # autre := ["commentaire"];
            d := 1.;
            x := true;
            g := null;
            aa := (n) => {
                print("hello" + n)
            }; # commentaire
            i := ()=> {
                1
            };
            j := 4*2+1 *sqrt(2.);
            r := () => "hello!";
            mm := (2 *2)
        }
        "#;
    let (res, struc) = parse_instructions(expr).unwrap();
    assert_eq!("", res);

    assert_eq!(
        struc,
        vec![Value::VariableExpr {
            name: Box::new(Value::Variable("my".to_string(),)),
            expr: Box::new(Value::Struct(HashMap::from([
                (
                    "aa".into(),
                    Value::Function {
                        parameters: Box::new(Value::BlockParen(vec![
                            Value::Variable("n".into(),),
                        ],)),
                        exprs: vec![Value::Expression(vec![
                            Value::BuiltInFunction {
                                fn_type: BuiltInFunctionType::Print,
                                expr: Box::new(BlockParen(vec![
                                    Value::String("hello".into(),),
                                    Value::Operation(Operator::Add,),
                                    Value::Variable("n".into(),),
                                ],)),
                            },
                        ],),],
                    }
                ),
                ("a".into(), Value::Integer(7,)),
                (
                    "r".into(),
                    Value::Function {
                        parameters: Box::new(BlockParen(vec![],)),
                        exprs: vec![Value::Expression(vec![Value::String(
                            "hello!".into(),
                        ),],),],
                    }
                ),
                (
                    "c".into(),
                    Value::Array(vec![
                        Value::Integer(1,),
                        Value::Integer(2,),
                        Value::Integer(3,),
                    ],)
                ),
                (
                    "i".into(),
                    Value::Function {
                        parameters: Box::new(Value::BlockParen(vec![],)),
                        exprs: vec![Value::Expression(vec![
                            Value::Integer(1,),
                        ],),],
                    }
                ),
                ("x".into(), Value::Bool(true,)),
                (
                    "mm".into(),
                    Value::BlockParen(vec![
                        Value::Integer(2,),
                        Value::Operation(Operator::Mult,),
                        Value::Integer(2,),
                    ],)
                ),
                (
                    "j".into(),
                    Value::Expression(vec![
                        Value::Integer(4,),
                        Value::Operation(Operator::Mult,),
                        Value::Integer(2,),
                        Value::Operation(Operator::Add,),
                        Value::Integer(1,),
                        Value::Operation(Operator::Mult,),
                        Value::BuiltInFunction {
                            fn_type: BuiltInFunctionType::Sqrt,
                            expr: Box::new(Value::BlockParen(vec![
                                Value::Decimal(2.0,),
                            ],)),
                        },
                    ]),
                ),
                ("b".into(), Value::String("salut".into(),)),
                ("d".into(), Value::Decimal(1.0,)),
                ("g".into(), Value::Null),
            ]),)),
        }]
    );
    dbg!(res, struc);
}
