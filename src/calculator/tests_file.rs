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
            ("z".to_string(), Primitive::Int(18)),
        ]),
        &ctx
    );
    assert_eq!(Primitive::Int(20), r);
}
