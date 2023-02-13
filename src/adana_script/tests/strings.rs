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

#[test]
fn test_string_block_with_parameters() {
    let expr = r#"
        name = "nordine"
        age = 34

        s = """Hello ${name}! You are ${age} years old."""
        "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(expr, &mut ctx).unwrap();
    assert_eq!(
        Primitive::String(
            r#"Hello nordine! You are 34 years old."#.to_string()
        ),
        ctx["s"].read().unwrap().clone()
    );
}

#[test]
fn test_string_block_with_parameters_struct() {
    let expr = r#"
        person = struct {
            name  : "nordine",
            wasup : (age) => {
                if (age > 30) {
                    "you are old!"
                } else {
                    "you are young!"
                }
            },
            age  : 34
        }

        s = """Hello ${person.name}! You are ${person.age} years old. ${person.wasup(person.age)}"""
        "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(expr, &mut ctx).unwrap();
    assert_eq!(
        Primitive::String(
            r#"Hello nordine! You are 34 years old. you are old!"#.to_string()
        ),
        ctx["s"].read().unwrap().clone()
    );
}
#[test]
fn test_string_block_complete() {
    let expr = r#"
        person = struct {
                    name  : "nordine",
                    wasup : (age) => {
                        if (age > 30) {
                            "you are old!"
                        } else {
                            "you are young!"
                        }
                    },
                    age  : 34
                }

        s0 = """Hello ${person.name}! You are ${person.age} years old. ${person.wasup(person.age)}"""
        s1 = """Hello ${person.name}! You are ${person.age} years old. ${person.wasup(person.age)}"""

        "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(expr, &mut ctx).unwrap();
    assert_eq!(
        Primitive::String(
            r#"Hello nordine! You are 34 years old. you are old!"#.to_string()
        ),
        ctx["s0"].read().unwrap().clone()
    );
    assert_eq!(
        Primitive::String(
            r#"Hello nordine! You are 34 years old. you are old!"#.to_string()
        ),
        ctx["s1"].read().unwrap().clone()
    );
}
