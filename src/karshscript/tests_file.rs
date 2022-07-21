use std::collections::BTreeMap;

use super::{compute, Primitive};

#[test]
fn test_simple_file() {
    let file_path = r#"
        k_load("file_tests/test1.karsher")
    "#;
    let mut ctx = BTreeMap::new();
    let r = compute(file_path, &mut ctx).unwrap();
    assert_eq!(
        &BTreeMap::from([
            ("a".to_string(), Primitive::Int(25)),
            ("b".to_string(), Primitive::Bool(true)),
            ("c".to_string(), Primitive::Bool(true)),
            ("d".to_string(), Primitive::Int(150)),
        ]),
        &ctx
    );
    assert_eq!(Primitive::Int(150), r);
}

#[test]
fn test_if_statement() {
    let file_path = r#"
     k_load("file_tests/test2.karsher")
    "#;
    let mut ctx = BTreeMap::new();
    let r = compute(file_path, &mut ctx).unwrap();
    assert_eq!(
        &BTreeMap::from([
            ("a".to_string(), Primitive::Int(25)),
            ("b".to_string(), Primitive::Int(12)),
            ("c".to_string(), Primitive::Int(20)),
            ("x".to_string(), Primitive::Int(15)),
            ("r".to_string(), Primitive::Int(11)),
            ("z".to_string(), Primitive::Int(18)),
        ]),
        &ctx
    );
    assert_eq!(Primitive::Int(20), r);
}
#[test]
fn test_while_statement() {
    let file_path = r#"
     k_load("file_tests/testfib.karsher")
    "#;
    let mut ctx = BTreeMap::new();

    fn fib(mut n: i128) -> i128 {
        let mut a = 0;
        let mut b = 1;
        let mut c = n;
        while n > 1 {
            c = a + b;
            a = b;
            b = c;
            n = n - 1;
        }
        c
    }

    for n in 0..=10 {
        ctx.insert("n".to_string(), Primitive::Int(n));
        let r = compute(file_path, &mut ctx).unwrap();
        let fibonacci = fib(n);
        assert_eq!(Some(&Primitive::Int(fibonacci)), ctx.get("c"));
        assert_eq!(Primitive::Int(fibonacci), r);
        if fibonacci < 55 {
            assert_eq!(
                Some(&Primitive::String(format!(
                    "this is a complex program: {fibonacci}"
                ))),
                ctx.get("x")
            );
        }
    }
}

#[test]
fn test_nested_file() {
    let file_path = r#"
        k_load("file_tests/test_nested.karsher")
    "#;
    let mut ctx = BTreeMap::new();
    let r = compute(file_path, &mut ctx).unwrap();

    assert_eq!(
        &BTreeMap::from([
            ("a".to_string(), Primitive::Int(0)),
            ("b".to_string(), Primitive::Int(240)),
            ("x".to_string(), Primitive::Int(50)),
            ("s".to_string(), Primitive::String("mod 3".to_string())),
            ("z".to_string(), Primitive::String("mod 1".to_string())),
        ]),
        &ctx
    );
    assert_eq!(Primitive::Int(240), r);
}

#[test]
fn test_fizz_buzz() {
    let file_path = r#"
        k_load("file_tests/test_fizzbuzz.karsher")
    "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(file_path, &mut ctx);

    assert_eq!(
        Some(&Primitive::String("100= Buzz".to_string())),
        ctx.get("text")
    );
}

#[test]
fn test_multiline_file() {
    let file_path = r#"
    k_load("file_tests/test_multiline.karsher")
"#;
    let mut ctx = BTreeMap::new();
    let _ = compute(file_path, &mut ctx);

    assert_eq!(Some(&Primitive::Int(77)), ctx.get("s"));
    assert_eq!(
        Some(&Primitive::String(
            "multiline\n    sin v\n    text\n  ".to_string()
        )),
        ctx.get("v")
    );
    assert_eq!(
        Some(&Primitive::String("sincostanmultiline".to_string())),
        ctx.get("x")
    );
    assert_eq!(Some(&Primitive::Int(1)), ctx.get("xy"));
    assert_eq!(Some(&Primitive::Int(2)), ctx.get("ze"));
    assert_eq!(Some(&Primitive::Int(3)), ctx.get("de"));
}

#[test]
fn test_if_else_file() {
    let file_path = r#"
    k_load("file_tests/test_if_else.karsher")
"#;
    let mut ctx = BTreeMap::new();
    ctx.insert("count".to_string(), Primitive::Int(102));
    let _ = compute(file_path, &mut ctx);

    assert_eq!(ctx.get("count"), Some(&Primitive::Int(101)));
    let _ = compute(file_path, &mut ctx);
    assert_eq!(ctx.get("count"), Some(&Primitive::Int(51)));
    let file_path = r#"
    k_load("file_tests/test_fizzbuzz_else.karsher")
"#;
    let mut ctx = BTreeMap::new();
    let _ = compute(file_path, &mut ctx);

    assert_eq!(Some(&Primitive::String("".to_string())), ctx.get("text"));
    assert_eq!(Some(&Primitive::Int(101)), ctx.get("count"));
}
