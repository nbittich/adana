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
            ("c".to_string(), Primitive::Bool(true))
        ]),
        &ctx
    );
    assert_eq!(Primitive::Bool(true), r);
}
