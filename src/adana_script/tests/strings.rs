use std::collections::BTreeMap;

use crate::adana_script::{compute, Primitive};

#[test]
fn test_string_block() {
    let expr = r#"
        s = """For strings, you can use string blocks:
            I hope you are well.
            This is a string block. you can use stuff like "string"
            there, nothing will stop you"""
        "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(expr, &mut ctx).unwrap();
    assert_eq!(
        Primitive::String(
            r#"For strings, you can use string blocks:
            I hope you are well.
            This is a string block. you can use stuff like "string"
            there, nothing will stop you"#
                .to_string()
        ),
        ctx["s"].read().unwrap().clone()
    );
}
