use std::collections::BTreeMap;

use adana_script_core::primitive::Primitive;
use serial_test::serial;

use crate::adana_script::compute;

#[test]
#[serial]
fn test_simple_file() {
    let file_path = r#"
    include("file_tests/test1.adana")
    "#;
    let mut ctx = BTreeMap::new();
    let r = compute(file_path, &mut ctx, "N/A").unwrap();

    let ctx: BTreeMap<String, Primitive> = ctx
        .iter()
        .map(|(k, v)| (k.to_string(), v.read().unwrap().clone()))
        .collect();
    assert_eq!(
        &BTreeMap::from([
            ("a".to_string(), Primitive::U8(25)),
            ("b".to_string(), Primitive::Bool(true)),
            ("c".to_string(), Primitive::Bool(true)),
            ("d".to_string(), Primitive::U8(150)),
        ]),
        &ctx
    );
    assert_eq!(Primitive::U8(150), r);
}

#[test]
#[serial]
fn test_if_statement() {
    let file_path = r#"
    include("file_tests/test2.adana")
    "#;
    let mut ctx = BTreeMap::new();
    let r = compute(file_path, &mut ctx, "N/A").unwrap();

    let ctx: BTreeMap<String, Primitive> = ctx
        .iter()
        .map(|(k, v)| (k.to_string(), v.read().unwrap().clone()))
        .collect();
    assert_eq!(
        &BTreeMap::from([
            ("a".to_string(), Primitive::U8(25)),
            ("b".to_string(), Primitive::U8(12)),
            ("c".to_string(), Primitive::U8(20)),
            ("x".to_string(), Primitive::U8(15)),
            ("r".to_string(), Primitive::U8(11)),
            ("z".to_string(), Primitive::U8(18)),
        ]),
        &ctx
    );
    assert_eq!(Primitive::U8(20), r);
}
#[test]
#[serial]
fn test_while_statement() {
    let file_path = r#"
    include("file_tests/testfib.adana")
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
            n -= 1;
        }
        c
    }

    for n in 0..=10 {
        ctx.insert("n".to_string(), Primitive::U8(n as u8).ref_prim());
        let r = compute(file_path, &mut ctx, "N/A").unwrap();
        let fibonacci = fib(n) as u8;
        assert_eq!(Primitive::U8(fibonacci), ctx["c"].read().unwrap().clone());
        assert_eq!(Primitive::U8(fibonacci), r);
        if fibonacci < 55 {
            assert_eq!(
                Primitive::String(format!(
                    "this is a complex program: {fibonacci}"
                )),
                ctx["x"].read().unwrap().clone()
            );
        }
    }
}

#[test]
#[serial]
fn test_nested_file() {
    let file_path = r#"
    include("file_tests/test_nested.adana")
    "#;
    let mut ctx = BTreeMap::new();
    let r = compute(file_path, &mut ctx, "N/A").unwrap();

    let ctx: BTreeMap<String, Primitive> = ctx
        .iter()
        .map(|(k, v)| (k.to_string(), v.read().unwrap().clone()))
        .collect();
    assert_eq!(
        &BTreeMap::from([
            ("a".to_string(), Primitive::U8(0)),
            ("b".to_string(), Primitive::U8(240)),
            ("x".to_string(), Primitive::U8(50)),
            ("s".to_string(), Primitive::String("mod 3".to_string())),
            ("z".to_string(), Primitive::String("mod 1".to_string())),
        ]),
        &ctx
    );
    assert_eq!(Primitive::U8(240), r);
}

#[test]
#[serial]
fn test_fizz_buzz() {
    let file_path = r#"
    include("file_tests/test_fizzbuzz.adana")
    "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(file_path, &mut ctx, "N/A");

    assert_eq!(
        Primitive::String("100 = Buzz".to_string()),
        ctx["text"].read().unwrap().clone()
    );
}

#[test]
#[serial]
fn test_includes() {
    let mut ctx = BTreeMap::new();
    let file_path = r#"
    include("file_tests/includes/reverse.adana")
"#;
    let _ = compute(file_path, &mut ctx, "N/A").unwrap();

    let ctx: BTreeMap<String, Primitive> = ctx
        .iter()
        .map(|(k, v)| (k.to_string(), v.read().unwrap().clone()))
        .collect();
    dbg!("wesh", &ctx);
    assert_eq!(
        ctx,
        BTreeMap::from([
            ("a".to_string(), Primitive::U8(144,)),
            (
                "arr".to_string(),
                Primitive::Array(vec![
                    Primitive::U8(1,),
                    Primitive::U8(2,),
                    Primitive::U8(3,),
                ],)
            ),
            (
                "arr2".to_string(),
                Primitive::Array(vec![
                    Primitive::U8(4,),
                    Primitive::U8(5,),
                    Primitive::U8(6,),
                ],)
            ),
            ("b".to_string(), Primitive::U8(233,)),
            ("bfr".to_string(), Primitive::U8(2,)),
            ("c".to_string(), Primitive::U8(233,)),
            ("n".to_string(), Primitive::U8(1,)),
            ("x".to_string(), Primitive::U8(4,))
        ]),
    );
}

#[test]
#[serial]
fn test_multiline_file() {
    let file_path = r#"
    include("file_tests/test_multiline.adana")
"#;
    let mut ctx = BTreeMap::new();
    let _ = compute(file_path, &mut ctx, "N/A");

    let ctx: BTreeMap<String, Primitive> = ctx
        .iter()
        .map(|(k, v)| (k.to_string(), v.read().unwrap().clone()))
        .collect();
    assert_eq!(Some(&Primitive::U8(77)), ctx.get("s"));
    assert_eq!(
        Some(&Primitive::String(
            "\n    multiline\n    sin v\n    text\n  ".to_string()
        )),
        ctx.get("v")
    );
    assert_eq!(
        Some(&Primitive::String("sincostanmultiline".to_string())),
        ctx.get("x")
    );
    assert_eq!(Some(&Primitive::U8(1)), ctx.get("xy"));
    assert_eq!(Some(&Primitive::U8(2)), ctx.get("ze"));
    assert_eq!(Some(&Primitive::U8(3)), ctx.get("de"));
}

#[test]
#[serial]
fn test_if_else_file1() {
    let file_path = r#"
    include("file_tests/test_if_else.adana")
"#;
    let mut ctx = BTreeMap::new();
    ctx.insert("count".to_string(), Primitive::U8(102).ref_prim());
    let _ = compute(file_path, &mut ctx, "N/A");

    assert_eq!(ctx["count"].read().unwrap().clone(), Primitive::U8(101));
    let _ = compute(file_path, &mut ctx, "N/A");
    assert_eq!(ctx["count"].read().unwrap().clone(), Primitive::U8(51));
    let file_path = r#"
    include("file_tests/test_fizzbuzz_else.adana")
"#;
    let mut ctx = BTreeMap::new();
    let _ = compute(file_path, &mut ctx, "N/A");

    assert_eq!(
        Primitive::String("100 = Buzz".to_string()),
        ctx["text"].read().unwrap().clone()
    );
    assert_eq!(Primitive::I8(101), ctx["count"].read().unwrap().clone());
}
