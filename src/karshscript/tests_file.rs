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
