use std::collections::BTreeMap;

use crate::compute;

use adana_script_core::primitive::Primitive;
#[test]
fn test_string_block2() {
    let expr = r#"
        s = """For strings, you can use string blocks:
            I hope you are well.
            This is a string block. you can use stuff like "string"
            there, nothing will stop you"""
        "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(expr, &mut ctx, "N/A").unwrap();
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
fn test_string_escape() {
    let expr = r#"
        s = "\"gipitou engine, gipitou\""
        "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(
        Primitive::String(r#""gipitou engine, gipitou""#.to_string()),
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
    let _ = compute(expr, &mut ctx, "N/A").unwrap();
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
            age : 34
        }

        s = """Hello ${person.name}! You are ${person.age} years old. ${person.wasup(person.age)}"""
        "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(expr, &mut ctx, "N/A").unwrap();
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
        s3 = ""

        for n in 1..10 {
           s3 = s3+"""Hey: ${n}"""
        }
        println(s3)

        x = 0

        while(person.age > 1){
            person.age = person.age -1
        }
        
        "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(expr, &mut ctx, "N/A").unwrap();
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

#[test]
fn parse_multi_values() {
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
                    age  : 12,
                    calc: (a1, a2) => { a1* a2 },
                    x: "hi"
                }

        s0 = """Hello ${person.name}! You are ${person.age + person.calc(11,2)} years old.
            ${person.wasup(person.age)}"""
        x = 1
        while(person.age <10) {
            person.age = person.age - x
            if(x % 2 == 0) {
                x = x-2
            }else{
                x = x +1
            }
        println("""Loop ${person.age}""")
            
        }
        println(s0)

        "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(
        Primitive::String(
            r#"Hello nordine! You are 34 years old.
            you are young!"#
                .to_string()
        ),
        ctx["s0"].read().unwrap().clone()
    );
}

#[test]
fn parse_f_strings_fn() {
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
                    calc: (a1, a2) => { a1* a2 },
                    age  : 12,
                    s0 : (person) => { """Hello ${person.name}! You are ${person.age + person.calc(11,2)} years old.
            ${person.wasup(person.age)}""" }
        }
        x = 1
        while(person.age <10) {
            person.age = person.age - x
            if(x % 2 == 0) {
                x = x-2
            }else{
                x = x +1
            }
        println("""Loop ${person.age}""")
            
        }

        println(person.s0(person))
            s0 = person.s0(person)

        "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(
        Primitive::String(
            r#"Hello nordine! You are 34 years old.
            you are young!"#
                .to_string()
        ),
        ctx["s0"].read().unwrap().clone()
    );
}
