use std::collections::BTreeMap;

use serial_test::serial;

use crate::compute;

use adana_script_core::{primitive::Primitive, Value};
#[test]
#[serial]
fn test_simple_struc() {
    let mut ctx = BTreeMap::new();
    let expr = "x = struct {x: 8}";
    let _ = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(ctx.len(), 1);
    assert_eq!(
        ctx["x"].read().unwrap().clone(),
        Primitive::Struct(BTreeMap::from([(
            "x".to_string(),
            Primitive::U8(8)
        )]))
    );
}

#[test]
#[serial]
fn test_simple_struc_with_more_stuff_in_it() {
    let mut ctx = BTreeMap::new();
    let expr = r#"x = struct {
                x: 8,
                y: "hello;",
                z: ()=> {println("hello")}
           }"#;
    let _ = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(ctx.len(), 1);
    assert_eq!(
        ctx["x"].read().unwrap().clone(),
        Primitive::Struct(BTreeMap::from([
            ("x".to_string(), Primitive::U8(8)),
            ("y".to_string(), Primitive::String("hello;".to_string())),
            (
                "z".to_string(),
                Primitive::Function {
                    parameters: vec![],
                    exprs: vec![Value::BlockParen(vec![
                        Value::BuiltInFunction {
                            fn_type:
                                adana_script_core::BuiltInFunctionType::Println,
                            expr: Box::new(Value::BlockParen(vec![
                                Value::String("hello".to_string())
                            ]))
                        }
                    ])]
                }
            )
        ]))
    );
}

#[test]
#[serial]
fn test_struct_eq() {
    let mut ctx = BTreeMap::new();
    let expr = r#"
        x = struct {
                x: 8,
                y: "hello;",
                z: ()=> {println("hello")},
            }
        y = struct {
          z: () => {println("hello")},
          x: 8,
          y: "hello;"
        }
        x == y
    "#;
    let res = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(Primitive::Bool(true), res);

    let expr = r#"
        x = struct {
                x: 8,
                y: "hello;",
                z: ()=> {println("hello")}
            }
        y = struct {
          z: () => {println("hello")},
          x: 8
        }
        x == y
    "#;
    let res = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(Primitive::Bool(false), res);
}

#[test]
#[serial]
fn test_struct_access() {
    let mut ctx = BTreeMap::new();
    let expr = r#"
        person = struct {
                    name: "hello",
                    age: 20,
                 }
        person.age
        "#;
    let res = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(res, Primitive::U8(20));
}

#[test]
#[serial]
fn test_struct_variable_assign() {
    let mut ctx = BTreeMap::new();
    let expr = r#"
        person = struct {
                    name: "hello",
                    age: 20,
                 }
        person.age = 34
        person.age
        "#;
    let res = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(res, Primitive::U8(34));
}
#[test]
#[serial]
fn test_struct_complex_ish() {
    let mut ctx = BTreeMap::new();
    let expr = r#"
        person = struct {
                    name: "hello",
                    age: 14,
                    full_name: null,
                 }
        # person.age

        person_service = struct {
            say_hi:    (person) => { "hi " + person.name },
            check_age: (person) => {
                if (person.age < 18) {
                  return "you are too young"
                } else {
                  return "you are too old"
             }
            },
            boom: (person) => {
                if(person.full_name ==null) {
                    return "John Doe"
                }
                person.full_name
            },
        }
        test1 = person_service.say_hi(person)
        test2 = person_service.check_age(person)
        person.age = 34
        test3 = person_service.check_age(person)
        test4 = person_service.boom(person)
        person.full_name = "Nordine Bittich"
        test5 = person_service.boom(person)
        test6 = person_service["boom"](person)
        "#;
    let _ = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(
        ctx["test1"].read().unwrap().clone(),
        Primitive::String("hi hello".to_string())
    );
    assert_eq!(
        ctx["test2"].read().unwrap().clone(),
        Primitive::String("you are too young".to_string())
    );
    assert_eq!(
        ctx["test3"].read().unwrap().clone(),
        Primitive::String("you are too old".to_string())
    );
    assert_eq!(
        ctx["test4"].read().unwrap().clone(),
        Primitive::String("John Doe".to_string())
    );
    assert_eq!(
        ctx["test5"].read().unwrap().clone(),
        Primitive::String("Nordine Bittich".to_string())
    );
    assert_eq!(
        ctx["test6"].read().unwrap().clone(),
        Primitive::String("Nordine Bittich".to_string())
    );
}

#[test]
fn test_struct_access_key() {
    let mut ctx = BTreeMap::new();
    let expr = r#"
        s = struct {
            name: "nordine",
            age: 34,
            members: ["natalie", "roger","fred"],
        }
        s["name"]

       "#;
    let r = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::String("nordine".into()));
}
#[test]
fn test_struct_access_key2() {
    let mut ctx = BTreeMap::new();
    let expr = r#"
        struct {
            name: "nordine",
            age: 34,
            members: ["natalie", "roger","fred"],
        }["name"]

       "#;
    let r = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::String("nordine".into()));
}

#[test]
fn test_struct_access_key3() {
    let mut ctx = BTreeMap::new();
    let expr = r#"
        struct {
            name: "nordine",
            age: 34,
            members: ["natalie", "roger","fred"],
        }.name

       "#;
    let r = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::String("nordine".into()));
}

#[test]
fn test_struct_access_key4() {
    let mut ctx = BTreeMap::new();
    let expr = r#"
        struct {
            name: () => {"nordine"},
            age: 34,
            members: ["natalie", "roger","fred"],
        }.name()

       "#;
    let r = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::String("nordine".into()));
}

#[test]
fn test_struct_access_key5() {
    let mut ctx = BTreeMap::new();
    let expr = r#"
        struct {
            name: () => {"nordine"},
            age: 34,
            members: ["natalie", "roger","fred"],
        }["name"]()

       "#;
    let r = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::String("nordine".into()));
}
#[test]
fn test_struct_access_key6() {
    let mut ctx = BTreeMap::new();
    let expr = r#"
        struct {
            other_struct: struct {
                name: () => {"nordine"},
            },
            age: 34,
            members: ["natalie", "roger","fred"],
        }.other_struct["name"]()

       "#;
    let r = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::String("nordine".into()));
}

#[test]
fn test_struct_access_key7() {
    let mut ctx = BTreeMap::new();
    let expr = r#"
        x= struct {
            other_struct: struct {
                name: struct {
                    first_name: () => {"nordine"}
                    last_name: () => {"bittich"}
                    age: 36
                },
            },
            age: 34,
            members: ["natalie", "roger","fred"],
        }
        x.other_struct.name.first_name() + " " + x.other_struct["name"]["last_name"]() + ":" + x.other_struct["name"]["age"]

       "#;
    let r = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::String("nordine bittich:36".into()));
}

#[test]
fn test_struct_empty() {
    let mut ctx = BTreeMap::new();
    let expr = r#"
        s = struct {}
        s.x = "nordine"
        s.x
       "#;
    let r = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::String("nordine".into()));
}
