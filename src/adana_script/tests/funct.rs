use std::collections::BTreeMap;

use crate::adana_script::compute;

use adana_script_core::{primitive::Primitive, Operator, Value};
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
        *ctx["z"].read().unwrap(),
        Primitive::Function {
            parameters: vec![
                Value::Variable("a".to_string()),
                Value::Variable("b".to_string()),
            ],
            exprs: vec![
                Value::VariableExpr {
                    name: Box::new(Value::Variable("c".to_string(),)),
                    expr: Box::new(Value::Integer(4,),),
                },
                Value::Expression(vec![
                    Value::Variable("a".to_string(),),
                    Value::Operation(Operator::Add,),
                    Value::Variable("b".to_string(),),
                    Value::Operation(Operator::Mult,),
                    Value::Variable("c".to_string(),),
                ],),
            ],
        }
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
    assert_eq!(Primitive::Null, res); // todo change that with null or smth
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
        *ctx["m"].read().unwrap(),
        Primitive::Array(vec![Primitive::Array(vec![
            Primitive::String("nordine".to_string(),),
            Primitive::Int(35,),
        ],),],)
    );
}

#[test]
#[serial_test::serial]
fn test_drop() {
    let script = r#"
        include("file_tests/test_fn.adana")
        m = map()
        m = push_v("nordine", 34, m)
        m = push_v("nordine", 35, m)
        z = get_v("nordine", m)
        drop(m)
    "#;
    let mut ctx = BTreeMap::new();

    let res = compute(script, &mut ctx).unwrap();
    assert_eq!(Primitive::Unit, res);

    assert_eq!(*ctx["z"].read().unwrap(), Primitive::Int(35,));
    assert!(!ctx.contains_key("m"));
}

#[test]
fn test_inline_fn() {
    let script = r#"
        hello = (name) => { "hello " + name } 
        hello_me = hello("nordine")
        hello_world = hello("world")
        null
    "#;
    let mut ctx = BTreeMap::new();
    let res = compute(script, &mut ctx).unwrap();

    assert_eq!(Primitive::Null, res);

    assert_eq!(
        *ctx["hello_me"].read().unwrap(),
        Primitive::String("hello nordine".into())
    );
    assert_eq!(
        *ctx["hello_world"].read().unwrap(),
        Primitive::String("hello world".into())
    );

    let script = "hello = (name) => { \"hello \" + name}";

    let mut ctx = BTreeMap::new();
    let _ = compute(script, &mut ctx).unwrap();

    let script = "hello_me = hello(\"nordine\")";
    let res = compute(script, &mut ctx).unwrap();

    assert_eq!(Primitive::String("hello nordine".into()), res);
    assert_eq!(
        *ctx["hello_me"].read().unwrap(),
        Primitive::String("hello nordine".into())
    );

    let script = "hello_world = hello(\"world\")";
    let res = compute(script, &mut ctx).unwrap();
    assert_eq!(Primitive::String("hello world".into()), res);

    assert_eq!(
        *ctx["hello_world"].read().unwrap(),
        Primitive::String("hello world".into())
    );
}

#[test]
#[serial_test::serial]
fn test_recursive() {
    let s = r#"
        include("file_tests/test_recursion.adana")
        fact6 = fact(6)
    "#;
    let mut ctx = BTreeMap::new();
    let r = compute(s, &mut ctx).unwrap();

    assert_eq!(Primitive::Int(720), r);
}
#[test]
fn test_fn_param() {
    let s = r#"
    map_arr = (arr, f) => {
        count = 0
        len = length(arr)
        new_arr = []
        while(count < len) {
            new_arr = new_arr + [f(arr[count])]
            count = count + 1
        }
        new_arr
    }
    arr = map_arr([1,2,3], (a) => {
        a + 1
    }) 

    "#;
    let mut ctx = BTreeMap::new();
    let r = compute(s, &mut ctx).unwrap();
    assert_eq!(
        Primitive::Array(vec![
            Primitive::Int(2),
            Primitive::Int(3),
            Primitive::Int(4),
        ]),
        r
    );
}

#[test]
#[serial_test::serial]
fn test_if_else_file() {
    let file_path = r#"
    include("file_tests/string.adana")
    res = split("kekeke=lekeke=meme=me", "=")
"#;
    let mut ctx = BTreeMap::new();
    let r = compute(file_path, &mut ctx).unwrap();

    assert_eq!(
        r,
        Primitive::Array(vec![
            Primitive::String("kekeke".into()),
            Primitive::String("lekeke".into()),
            Primitive::String("meme".into()),
            Primitive::String("me".into()),
        ])
    );
    let file_path = r#"
    include("file_tests/string.adana")
    res = split("kekeke=akalekeke=meme=me", "=")
"#;
    let mut ctx = BTreeMap::new();
    let r = compute(file_path, &mut ctx).unwrap();

    assert_eq!(r, Primitive::Array(vec![Primitive::String("aka".into()),]));

    let file_path = r#"
    include("file_tests/string.adana")
    res = split("kekeke=akalekeke=meme=me", "")
"#;
    let mut ctx = BTreeMap::new();
    let r = compute(file_path, &mut ctx).unwrap();

    assert_eq!(r, Primitive::Null,);

    let file_path = r#"
    include("file_tests/string.adana")
    res = split("", "k")
"#;
    let mut ctx = BTreeMap::new();
    let r = compute(file_path, &mut ctx).unwrap();

    assert_eq!(r, Primitive::Null,);

    let file_path = r#"
    include("file_tests/string.adana")
    res = split(null, "k")
"#;
    let mut ctx = BTreeMap::new();
    let r = compute(file_path, &mut ctx).unwrap();

    assert_eq!(r, Primitive::Null,);

    let file_path = r#"
    include("file_tests/string.adana")
    res = split("sksksksk", null)
"#;
    let mut ctx = BTreeMap::new();
    let r = compute(file_path, &mut ctx).unwrap();

    assert_eq!(r, Primitive::Null,);
}

#[test]
#[serial_test::serial]
fn test_array_access_fn_call() {
    let mut ctx = BTreeMap::new();
    let expr = r#"
             s = [1, (i) => { 1 + i  }, 2 , (name)=> { println("hello " + name)}, 6, (name) => { "hello " + name}]
             z = s[1](5)
             y = s[5]("nordine")
             s[5]("nordine2")

        "#;
    let r = compute(expr, &mut ctx).unwrap();
    assert_eq!(r, Primitive::String("hello nordine2".into()));
    assert_eq!(*ctx["z"].read().unwrap(), Primitive::Int(6));
    assert_eq!(
        *ctx["y"].read().unwrap(),
        Primitive::String("hello nordine".into())
    );
}
