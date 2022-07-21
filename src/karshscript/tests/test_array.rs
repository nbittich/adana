use std::collections::BTreeMap;

use crate::karshscript::{compute, Primitive};

#[test]
fn test_simple_array() {
    let file_path = r#"
        k_load("file_tests/test_simple_array.karsher")
    "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(file_path, &mut ctx).unwrap();
    assert_eq!(
        &BTreeMap::from([
            (
                "x".to_string(),
                Primitive::Array(vec![
                    Primitive::String("hello".to_string()),
                    Primitive::Int(1),
                    Primitive::String("World".to_string()),
                    Primitive::Bool(true)
                ])
            ),
            (
                "y".to_string(),
                Primitive::Array(vec![
                    Primitive::String("hello".to_string()),
                    Primitive::Int(1),
                    Primitive::String("World".to_string()),
                    Primitive::Bool(true),
                    Primitive::String("hello".to_string()),
                    Primitive::Int(1),
                    Primitive::String("World".to_string()),
                    Primitive::Bool(true),
                    Primitive::String("hello".to_string()),
                    Primitive::Int(1),
                    Primitive::String("World".to_string()),
                    Primitive::Bool(true),
                    Primitive::String("hello".to_string()),
                    Primitive::Int(1),
                    Primitive::String("World".to_string()),
                    Primitive::Bool(true),
                    Primitive::String("hello".to_string()),
                    Primitive::Int(1),
                    Primitive::String("World".to_string()),
                    Primitive::Bool(true)
                ])
            ),
            ("z".to_string(), Primitive::String("World".to_string())),
            ("a".to_string(), Primitive::Bool(true)),
        ]),
        &ctx
    );
}
