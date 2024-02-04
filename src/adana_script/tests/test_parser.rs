use std::collections::BTreeMap;

use crate::adana_script::parser::parse_instructions;
use adana_script_core::{
    BuiltInFunctionType, Operator,
    Value::{
        self, BlockParen, Expression, Function, Operation, Variable,
        VariableExpr, WhileExpr, U8,
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
                    expr: Box::new(Value::U8(0,)),
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
                expr: Box::new(Value::U8(0,)),
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
                    expr: Box::new(Value::U8(0,),),
                },
                Value::BuiltInFunction {
                    fn_type: BuiltInFunctionType::Println,
                    expr: Box::new(Value::BlockParen(vec![Value::String(
                        "hello".to_string()
                    )]))
                }
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
                        expr: Box::new(Value::U8(0))
                    },
                    VariableExpr {
                        name: Box::new(Variable("res".to_string(),)),
                        expr: Box::new(Expression(vec![
                            Operation(Operator::Subtr,),
                            Value::U8(1,),
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
                                expr: Box::new(Value::ArrayAccess {
                                    arr: Box::new(Variable("m".to_string(),)),
                                    index: Box::new(Variable(
                                        "count".to_string(),
                                    )),
                                },)
                            },
                            Value::IfExpr {
                                cond: Box::new(BlockParen(vec![
                                    Value::ArrayAccess {
                                        arr: Box::new(Variable(
                                            "k".to_string(),
                                        )),
                                        index: Box::new(U8(0,)),
                                    },
                                    Operation(Operator::Equal,),
                                    Variable("key".to_string(),),
                                ],),),
                                exprs: vec![
                                    VariableExpr {
                                        name: Box::new(Variable(
                                            "res".to_string(),
                                        )),
                                        expr: Box::new(Variable(
                                            "count".to_string(),
                                        ),),
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
                                        Value::U8(1,),
                                    ],)),
                                },],),
                            },
                        ],
                    },
                    Variable("res".to_string(),),
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
        vec![Value::BlockParen(vec![
            Value::U8(4,),
            Value::Operation(Operator::Mult,),
            Value::U8(3,),
            Value::Operation(Operator::Mult,),
            Value::BlockParen(vec![
                Value::U8(2,),
                Value::Operation(Operator::Add,),
                Value::U8(2,),
            ],),
        ],),]
    );
    dbg!(&res);
}
#[test]
fn test_struct1() {
    let expr = " struct {x: 99,}";
    let (res, struc) = parse_instructions(expr).unwrap();
    assert_eq!("", res);
    assert_eq!(
        vec![Value::Struct(BTreeMap::from([("x".into(), Value::U8(99))]))],
        struc
    );
}
#[test]
fn test_struct_array() {
    let expr = r#"x = [1, 2, struct{x: (name)=> {println("hello" + name)},}, "hello"]"#;
    let (res, struc_arr) = parse_instructions(expr).unwrap();
    assert_eq!("", res);
    assert_eq!(
        struc_arr,
        vec![Value::VariableExpr {
            name: Box::new(Value::Variable("x".into(),)),
            expr: Box::new(Value::Array(vec![
                Value::U8(1,),
                Value::U8(2,),
                Value::Struct(BTreeMap::from([(
                    "x".to_string(),
                    Value::Function {
                        parameters: Box::new(Value::BlockParen(vec![
                            Value::Variable("name".to_string(),),
                        ],)),
                        exprs: vec![Value::BlockParen(vec![
                            Value::BuiltInFunction {
                                fn_type: BuiltInFunctionType::Println,
                                expr: Box::new(Value::BlockParen(vec![
                                    Value::String("hello".into(),),
                                    Value::Operation(Operator::Add,),
                                    Value::Variable("name".into(),),
                                ],)),
                            },
                        ],),],
                    },
                )]),),
                Value::String("hello".into(),),
            ]),),
        }]
    );
}
#[test]
fn test_struct_array2() {
    let expr = r#"x = [1, 2, struct{
                                x: (name)=> {println("hello" + name)},
                            },
               "hello"]"#;
    let (res, struc_arr) = parse_instructions(expr).unwrap();
    assert_eq!("", res);
    assert_eq!(
        struc_arr,
        vec![Value::VariableExpr {
            name: Box::new(Value::Variable("x".into(),)),
            expr: Box::new(Value::Array(vec![
                Value::U8(1,),
                Value::U8(2,),
                Value::Struct(BTreeMap::from([(
                    "x".to_string(),
                    Value::Function {
                        parameters: Box::new(Value::BlockParen(vec![
                            Value::Variable("name".to_string(),),
                        ],)),
                        exprs: vec![Value::BlockParen(vec![
                            Value::BuiltInFunction {
                                fn_type: BuiltInFunctionType::Println,
                                expr: Box::new(Value::BlockParen(vec![
                                    Value::String("hello".into(),),
                                    Value::Operation(Operator::Add,),
                                    Value::Variable("name".into(),),
                                ],)),
                            },
                        ],),],
                    },
                )]),),
                Value::String("hello".into(),),
            ]),),
        }]
    );
}
#[test]
fn test_struct2() {
    let expr = r#"
        # commentaire
      my = struct { # commentaire
          # ici un commentaire
            b : "salut", # i am a comment
            c : [1,2,3],
            a : 7,
      # autre : ["commentaire"],
            d : 1.,
            x : true,
            g : null,
            aa : (n) => {
                print("hello" + n)
            }, # commentaire
            i : ()=> {
                1
            },
            j : 4*2+1 *sqrt(2.),
            r : () => {"hello!"},
            mm : (2 *2),
        }
        #parse_number,
        "#;
    let (res, struc) = parse_instructions(expr).unwrap();
    dbg!(&struc);
    assert_eq!("", res);

    assert_eq!(
        struc,
        vec![Value::VariableExpr {
            name: Box::new(Value::Variable("my".to_string(),)),
            expr: Box::new(Value::Struct(BTreeMap::from([
                ("a".into(), Value::U8(7,)),
                (
                    "aa".into(),
                    Value::Function {
                        parameters: Box::new(Value::BlockParen(vec![
                            Value::Variable("n".into(),),
                        ],)),
                        exprs: vec![Value::BlockParen(vec![
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
                ("b".into(), Value::String("salut".into(),)),
                (
                    "c".into(),
                    Value::Array(vec![
                        Value::U8(1,),
                        Value::U8(2,),
                        Value::U8(3,),
                    ],)
                ),
                ("d".into(), Value::Decimal(1.0,)),
                ("g".into(), Value::Null),
                (
                    "i".into(),
                    Value::Function {
                        parameters: Box::new(Value::BlockParen(vec![],)),
                        exprs: vec![Value::BlockParen(vec![Value::U8(1,),],),],
                    }
                ),
                (
                    "j".into(),
                    Value::Expression(vec![
                        Value::U8(4,),
                        Value::Operation(Operator::Mult,),
                        Value::U8(2,),
                        Value::Operation(Operator::Add,),
                        Value::U8(1,),
                        Value::Operation(Operator::Mult,),
                        Value::BuiltInFunction {
                            fn_type: BuiltInFunctionType::Sqrt,
                            expr: Box::new(Value::BlockParen(vec![
                                Value::Decimal(2.0,),
                            ],)),
                        },
                    ]),
                ),
                (
                    "mm".into(),
                    Value::BlockParen(vec![
                        Value::U8(2,),
                        Value::Operation(Operator::Mult,),
                        Value::U8(2,),
                    ],)
                ),
                (
                    "r".into(),
                    Value::Function {
                        parameters: Box::new(BlockParen(vec![],)),
                        exprs: vec![Value::BlockParen(vec![Value::String(
                            "hello!".into(),
                        ),],),],
                    }
                ),
                ("x".into(), Value::Bool(true,)),
            ]),)),
        }]
    );
    dbg!(res, struc);
}

#[test]
fn test_array_fn_access() {
    let expr = r#"
         x = (i) => { println("hello") }
         n = [1, 2, x]
        n[2](5)
        "#;
    let (r, instr) = parse_instructions(expr).unwrap();
    assert_eq!("", r);
    assert_eq!(
        instr,
        vec![
            Value::VariableExpr {
                name: Box::new(Value::Variable("x".into())),
                expr: Box::new(Value::Function {
                    parameters: Box::new(Value::BlockParen(vec![
                        Value::Variable("i".into())
                    ])),
                    exprs: vec![Value::BlockParen(vec![
                        Value::BuiltInFunction {
                            fn_type: BuiltInFunctionType::Println,
                            expr: Box::new(Value::BlockParen(vec![
                                Value::String("hello".into())
                            ]))
                        }
                    ])]
                })
            },
            Value::VariableExpr {
                name: Box::new(Value::Variable("n".into())),
                expr: Box::new(Value::Array(vec![
                    Value::U8(1),
                    Value::U8(2),
                    Value::Variable("x".into())
                ]))
            },
            Value::FunctionCall {
                parameters: Box::new(Value::BlockParen(vec![Value::U8(5)])),
                function: Box::new(Value::ArrayAccess {
                    arr: Box::new(Value::Variable("n".into())),
                    index: Box::new(Value::U8(2))
                })
            }
        ]
    );
    dbg!(instr);
}

#[test]
fn test_inline_fn1() {
    let expr = r#"
        s = [1, (i) => { println(i) },2]
        a = () => {1 +2}
        hello = (name) => {"hello " + name}

        "#;
    let (r, instr) = parse_instructions(expr).unwrap();
    assert_eq!("", r);

    assert_eq!(
        instr,
        vec![
            Value::VariableExpr {
                name: Box::new(Value::Variable("s".into())),
                expr: Box::new(Value::Array(vec![
                    Value::U8(1),
                    Value::Function {
                        parameters: Box::new(Value::BlockParen(vec![
                            Value::Variable("i".into())
                        ])),
                        exprs: vec![Value::BlockParen(vec![
                            Value::BuiltInFunction {
                                fn_type: BuiltInFunctionType::Println,
                                expr: Box::new(Value::BlockParen(vec![
                                    Value::Variable("i".into())
                                ]))
                            }
                        ])]
                    },
                    Value::U8(2)
                ]))
            },
            Value::VariableExpr {
                name: Box::new(Value::Variable("a".into(),)),
                expr: Box::new(Value::Function {
                    parameters: Box::new(BlockParen(vec![],)),
                    exprs: vec![Value::BlockParen(vec![
                        Value::U8(1,),
                        Value::Operation(Operator::Add,),
                        Value::U8(2,),
                    ],),],
                }),
            },
            Value::VariableExpr {
                name: Box::new(Variable("hello".into(),)),
                expr: Box::new(Value::Function {
                    parameters: Box::new(Value::BlockParen(vec![
                        Value::Variable("name".into(),),
                    ],)),
                    exprs: vec![Value::BlockParen(vec![
                        Value::String("hello ".into(),),
                        Value::Operation(Operator::Add,),
                        Value::Variable("name".into(),),
                    ])],
                }),
            },
        ]
    );

    dbg!(instr);
}

#[test]
fn test_comments_end_arr() {
    let expr = r#"x[1]() # works"#;
    let (r, ins) = parse_instructions(expr).unwrap();
    assert_eq!("", r);
    assert_eq!(
        ins,
        vec![Value::FunctionCall {
            parameters: Box::new(Value::BlockParen(vec![],)),
            function: Box::new(Value::ArrayAccess {
                arr: Box::new(Value::Variable("x".to_string(),)),
                index: Box::new(Value::U8(1,)),
            }),
        }]
    );
    let expr = r#"x =[1, ()=> {print("hello")}] # doesn't work"#;
    let (r, ins) = parse_instructions(expr).unwrap();
    assert_eq!("", r);
    assert_eq!(
        ins,
        vec![Value::VariableExpr {
            name: Box::new(Value::Variable("x".into())),
            expr: Box::new(Value::Array(vec![
                Value::U8(1,),
                Value::Function {
                    parameters: Box::new(Value::BlockParen(vec![],)),
                    exprs: vec![Value::BlockParen(vec![
                        Value::BuiltInFunction {
                            fn_type: BuiltInFunctionType::Print,
                            expr: Box::new(Value::BlockParen(vec![
                                Value::String("hello".into()),
                            ],)),
                        },
                    ],),],
                },
            ],)),
        },]
    );
}

#[test]
fn test_struct_access_1() {
    let expr = r#"
            person = struct {
                name: "nordine",
                age: 34,
            }
            println(person.age)
            x = [9, person.name]
        "#;
    let (r, ins) = parse_instructions(expr).unwrap();
    assert_eq!("", r);
    assert_eq!(
        ins,
        vec![
            Value::VariableExpr {
                name: Box::new(Value::Variable("person".to_string(),)),
                expr: Box::new(Value::Struct(BTreeMap::from([
                    ("name".to_string(), Value::String("nordine".to_string(),)),
                    ("age".to_string(), Value::U8(34,),)
                ]),)),
            },
            Value::BuiltInFunction {
                fn_type: BuiltInFunctionType::Println,
                expr: Box::new(Value::BlockParen(vec![Value::StructAccess {
                    struc: Box::new(Value::Variable("person".to_string(),)),
                    key: "age".to_string(),
                },],)),
            },
            Value::VariableExpr {
                name: Box::new(Value::Variable("x".to_string())),
                expr: Box::new(Value::Array(vec![
                    Value::U8(9),
                    Value::StructAccess {
                        struc: Box::new(Value::Variable("person".to_string())),
                        key: "name".to_string()
                    }
                ]))
            }
        ]
    );
}

#[test]
fn test_parser_array_directly_access() {
    use Value::*;
    let expr = r#"x = [1, 2, 3][0]"#;
    let (r, v) = parse_instructions(expr).unwrap();
    assert_eq!("", r);
    assert_eq!(
        v,
        vec![VariableExpr {
            name: Box::new(Variable("x".into()),),
            expr: Box::new(ArrayAccess {
                arr: Box::new(Array(vec![U8(1,), U8(2,), U8(3,),],)),
                index: Box::new(U8(0,)),
            }),
        }]
    )
}

#[test]
fn test_parser_buggy_fn_call_op() {
    use Value::*;
    let expr = r#"
        f= (x) => {x*2}
        a = 39
        b= 42
        f(a) + f(b)

    "#;
    let (r, v) = parse_instructions(expr).unwrap();
    assert_eq!("", r);
    println!("{v:#?}");
    assert_eq!(
        v,
        vec![
            VariableExpr {
                name: Box::new(Variable("f".to_string())),
                expr: Box::new(Function {
                    parameters: Box::new(BlockParen(vec![Variable(
                        "x".to_string()
                    )])),
                    exprs: vec![BlockParen(vec![
                        Variable("x".to_string()),
                        Operation(Operator::Mult),
                        U8(2)
                    ])]
                })
            },
            VariableExpr {
                name: Box::new(Variable("a".to_string())),
                expr: Box::new(U8(39))
            },
            VariableExpr {
                name: Box::new(Variable("b".to_string())),
                expr: Box::new(U8(42))
            },
            Expression(vec![
                FunctionCall {
                    parameters: Box::new(BlockParen(vec![Variable(
                        "a".to_string()
                    )])),
                    function: Box::new(Variable("f".to_string()))
                },
                Operation(Operator::Add),
                FunctionCall {
                    parameters: Box::new(BlockParen(vec![Variable(
                        "b".to_string()
                    )])),
                    function: Box::new(Variable("f".to_string()))
                }
            ])
        ]
    )
}

#[test]
fn test_parse_string_escaped() {
    let expr = r#""u\nno""#;
    let (r, v) = parse_instructions(expr).unwrap();
    assert_eq!("", r);
    assert_eq!(v, vec![Value::String("u\nno".into())]);
}
#[test]
fn test_bug_fn_hello_a() {
    let expr = r#"fun = (a) => {b = "hello" + a }"#;
    let (r, v) = parse_instructions(expr).unwrap();
    assert_eq!("", r);
    assert_eq!(
        v,
        vec![VariableExpr {
            name: Box::new(Variable("fun".into(),)),
            expr: Box::new(Function {
                parameters: Box::new(BlockParen(vec![Variable("a".into()),],)),
                exprs: vec![VariableExpr {
                    name: Box::new(Variable("b".into())),
                    expr: Box::new(Expression(vec![
                        Value::String("hello".into(),),
                        Operation(Operator::Add,),
                        Variable("a".into(),),
                    ],)),
                },],
            },)
        },]
    )
}
