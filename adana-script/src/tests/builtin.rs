use std::collections::BTreeMap;

use crate::compute;
use adana_script_core::primitive::Primitive;
#[test]
fn test_builtin_to_int() {
    let mut ctx = BTreeMap::new();
    let res = compute("to_int(2)", &mut ctx, "N/A").unwrap();
    assert_eq!(res, Primitive::U8(2));

    let res = compute("to_int(256)", &mut ctx, "N/A").unwrap();
    assert_eq!(res, Primitive::Int(256));

    let res = compute(r#"to_int("2")"#, &mut ctx, "N/A").unwrap();
    assert_eq!(res, Primitive::Int(2));

    ctx.insert(
        "a".to_string(),
        Primitive::String("123".to_string()).ref_prim(),
    );
    let res = compute("to_int(a)", &mut ctx, "N/A").unwrap();
    assert_eq!(res, Primitive::Int(123));
}

#[test]
fn test_builtin_to_double() {
    let mut ctx = BTreeMap::new();
    let res = compute("to_double(2)", &mut ctx, "N/A").unwrap();
    assert_eq!(res, Primitive::Double(2.0));
    let res = compute(r#"to_double("2.1")"#, &mut ctx, "N/A").unwrap();
    assert_eq!(res, Primitive::Double(2.1));
}

#[test]
fn test_builtin_to_bool() {
    let mut ctx = BTreeMap::new();
    let res = compute("to_bool(2)", &mut ctx, "N/A").unwrap();
    assert_eq!(res, Primitive::Bool(true));
    let res = compute(r#"to_bool("false")"#, &mut ctx, "N/A").unwrap();
    assert_eq!(res, Primitive::Bool(false));
    ctx.insert("a".to_string(), Primitive::Double(0.0).ref_prim());
    let res = compute("to_bool(a)", &mut ctx, "N/A").unwrap();
    assert_eq!(res, Primitive::Bool(false));
}

#[test]
fn test_eval() {
    let mut ctx = BTreeMap::new();
    let _ = compute(r#"eval("z = sqrt(9)")"#, &mut ctx, "N/A").unwrap();
    assert_eq!(*ctx["z"].read().unwrap(), Primitive::Double(3.0));
}

#[test]
fn test_to_hex() {
    let mut ctx = BTreeMap::new();
    let r = compute(r#"to_hex(255)"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::String("0xff".into()));
    let r = compute(r#"to_hex(1)"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::String("0x1".into()));
    let r = compute(r#"to_hex(1024)"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::String("0x400".into()));
}

#[test]
fn test_to_binary() {
    let mut ctx = BTreeMap::new();
    let r = compute(r#"to_binary(255)"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::String("0b11111111".into()));
    let r = compute(r#"to_binary(-127)"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::String("0b10000001".into()));
    let r = compute(r#"to_binary(127)"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::String("0b1111111".into()));
}

#[test]
fn test_floor() {
    let mut ctx = BTreeMap::new();
    let r = compute(r#"floor(4.7)"#, &mut ctx, "N/A").unwrap();
    assert_eq!(Primitive::Double(4.), r);
}

#[test]
fn test_ceil() {
    let mut ctx = BTreeMap::new();
    let r = compute(r#"ceil(4.7)"#, &mut ctx, "N/A").unwrap();
    assert_eq!(Primitive::Double(5.), r);
}
#[test]
fn test_parse_json() {
    let mut ctx = BTreeMap::new();
    let expr = r#"parse_json("""{"a": 9}""")"#;
    let r = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(
        r,
        Primitive::Struct(BTreeMap::from([(
            "a".to_string(),
            Primitive::Int(9)
        )]))
    );
    let expr = r#"
       struct {
            firstName: "Nordine",
            lastName: "Bittich",
            age: 400,
            notes: [1034,1032,3.18,"hello",3334],
            skills: struct {
                programming: "🔥",
                sport:  "⚽",
            }
        }
    "#;
    let expect = compute(expr, &mut ctx, "N/A").unwrap();
    let expr = r#"
      s= parse_json("""{
        "age": 400,
        "firstName": "Nordine",
        "lastName": "Bittich",
        "notes": [
            1034,
            1032,
            3.18,
            "hello",
            3334
        ],
        "skills": {
            "programming": "🔥",
            "sport": "⚽"
        }
        }""")
    "#;
    let curr = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(curr, expect);
}
#[test]
fn test_jsonify() {
    let mut ctx = BTreeMap::new();
    let expr = r#"jsonify(struct {a: 9})"#;
    let r = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(
        r,
        Primitive::String(
            r#"{
  "a": 9
}"#
            .to_string()
        )
    );

    let expr = r#"
        s = struct {
            firstName: "Nordine",
            lastName: "Bittich",
            age: 36,
            notes: [1,2,3.18,"hello",4],
            skills: struct {
                programming: "🔥",
                sport:  "⚽",        
            }
        }
        jsonify(s)
    "#;
    let r = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(
        Primitive::String(
            r#"{
  "age": 36,
  "firstName": "Nordine",
  "lastName": "Bittich",
  "notes": [
    1,
    2,
    3.18,
    "hello",
    4
  ],
  "skills": {
    "programming": "🔥",
    "sport": "⚽"
  }
}"#
            .to_string()
        ),
        r
    );
}

#[test]
fn test_round() {
    let mut ctx = BTreeMap::new();
    let r = compute(r#"round(4.46790, 2)"#, &mut ctx, "N/A").unwrap();
    assert_eq!(Primitive::Double(4.47), r);
    let r = compute(r#"round(4.46790, 3)"#, &mut ctx, "N/A").unwrap();
    assert_eq!(Primitive::Double(4.468), r);
}
#[test]
fn test_to_lower() {
    let mut ctx = BTreeMap::new();
    let r = compute(r#"to_lower("HELLO")"#, &mut ctx, "N/A").unwrap();
    assert_eq!(Primitive::String("hello".into()), r);
}
#[test]
fn test_to_upper() {
    let mut ctx = BTreeMap::new();
    let r = compute(r#"to_upper("hello")"#, &mut ctx, "N/A").unwrap();
    assert_eq!(Primitive::String("HELLO".into()), r);
}
#[test]
fn test_capitalize() {
    let mut ctx = BTreeMap::new();
    let r = compute(r#"capitalize("nordine")"#, &mut ctx, "N/A").unwrap();
    assert_eq!(Primitive::String("Nordine".into()), r);
}

#[test]
fn test_replace() {
    let mut ctx = BTreeMap::new();
    let r = compute(r#"replace("Hello, world! Welcome to the world of JavaScript.", "world", "universe")"#, &mut ctx, "N/A").unwrap();
    assert_eq!(
        Primitive::String(
            "Hello, universe! Welcome to the world of JavaScript.".into()
        ),
        r
    );
    let r = compute(
        r#"replace("AaAaAbbBBBb", "(?i)a+(?-i)b+", "xxx")"#,
        &mut ctx,
        "N/A",
    )
    .unwrap();
    assert_eq!(Primitive::String("xxxBBBb".into()), r);
    let r = compute(r#"replace("The event is on 12/31/2023. Another date is 31-12-2023. And another one is 2023.12.31.", "(\d{2})/(\d{2})/(\d{4})", "$2-$1-$3")"#, &mut ctx, "N/A").unwrap();
    assert_eq!(
        Primitive::String(
            "The event is on 31-12-2023. Another date is 31-12-2023. And another one is 2023.12.31.".into()
        ),
        r
    );
}

#[test]
fn test_replace_all() {
    let mut ctx = BTreeMap::new();
    let r = compute(r#"replace_all("Hello, world! Welcome to the world of JavaScript.", "world", "universe")"#, &mut ctx, "N/A").unwrap();
    assert_eq!(
        Primitive::String(
            "Hello, universe! Welcome to the universe of JavaScript.".into()
        ),
        r
    );
    let r = compute(r#"replace_all("AaAaAbbBBBb", "A", "b")"#, &mut ctx, "N/A")
        .unwrap();
    assert_eq!(Primitive::String("bababbbBBBb".into()), r);
    let r = compute(r#"replace_all("The event is on 12/31/2023. Another date is 31-12-2023. And another one is 12/31/2023.", "(\d{2})/(\d{2})/(\d{4})", "$2-$1-$3")"#, &mut ctx, "N/A").unwrap();
    assert_eq!(
        Primitive::String(
            "The event is on 31-12-2023. Another date is 31-12-2023. And another one is 31-12-2023.".into()
        ),
        r
    );
}
#[test]
fn test_is_match() {
    let mut ctx = BTreeMap::new();
    let r = compute(
        r#"
      pattern = "(?i)a+(?-i)b+"
      text = "AaAaAbbBBBb"
      is_match(text, pattern)
 
    "#,
        &mut ctx,
        "N/A",
    )
    .unwrap();
    assert_eq!(Primitive::Bool(true), r);
    let mut ctx = BTreeMap::new();
    let r = compute(
        r#"
      pattern = "(\w+): \$(\d+)"
      text = "Item1: $100, Item2: $200, Item3: $300"
      is_match(text, pattern)
 
    "#,
        &mut ctx,
        "N/A",
    )
    .unwrap();
    assert_eq!(Primitive::Bool(true), r);
}
#[test]
fn test_match() {
    let mut ctx = BTreeMap::new();
    let r = compute(
        r#"
      pattern = "(?i)a+(?-i)b+"
      text = "AaAaAbbBBBb"
      match(text, pattern)
 
    "#,
        &mut ctx,
        "N/A",
    )
    .unwrap();
    assert_eq!(
        Primitive::Array(vec![Primitive::String("AaAaAbb".to_string())]),
        r
    );
    let mut ctx = BTreeMap::new();
    let r = compute(
        r#"
      pattern = "(\w+): \$(\d+)"
      text = "Item1: $100, Item2: $200, Item3: $300"
      match(text, pattern)
 
    "#,
        &mut ctx,
        "N/A",
    )
    .unwrap();
    assert_eq!(
        Primitive::Array(vec![
            Primitive::Array(vec![
                Primitive::String("Item1: $100".to_string()),
                Primitive::String("Item1".to_string()),
                Primitive::String("100".to_string())
            ]),
            Primitive::Array(vec![
                Primitive::String("Item2: $200".to_string()),
                Primitive::String("Item2".to_string()),
                Primitive::String("200".to_string())
            ]),
            Primitive::Array(vec![
                Primitive::String("Item3: $300".to_string()),
                Primitive::String("Item3".to_string()),
                Primitive::String("300".to_string())
            ])
        ]),
        r
    );
}

#[test]
fn test_type_of() {
    let mut ctx = BTreeMap::new();
    ctx.insert("x".to_string(), Primitive::Int(3).ref_prim());

    ctx.insert("y".to_string(), Primitive::Double(3.).ref_prim());
    ctx.insert(
        "z".to_string(),
        Primitive::Function { parameters: vec![], exprs: vec![] }.ref_prim(),
    );
    ctx.insert("a".to_string(), Primitive::Error("err".to_string()).ref_prim());
    ctx.insert("b".to_string(), Primitive::Array(vec![]).ref_prim());
    ctx.insert("c".to_string(), Primitive::Bool(true).ref_prim());
    ctx.insert("d".to_string(), Primitive::String("a".to_string()).ref_prim());
    ctx.insert("e".to_string(), Primitive::Unit.ref_prim());
    ctx.insert("f".to_string(), Primitive::NoReturn.ref_prim());
    ctx.insert(
        "g".to_string(),
        Primitive::EarlyReturn(Box::new(Primitive::Int(1))).ref_prim(),
    );
    assert_eq!(
        Primitive::String("int".to_string()),
        compute(r#"type_of(x)"#, &mut ctx, "N/A",).unwrap()
    );
    assert_eq!(
        Primitive::String("double".to_string()),
        compute(r#"type_of(y)"#, &mut ctx, "N/A",).unwrap()
    );
    assert_eq!(
        Primitive::String("function".to_string()),
        compute(r#"type_of(z)"#, &mut ctx, "N/A",).unwrap()
    );
    assert_eq!(
        Primitive::String("error".to_string()),
        compute(r#"type_of(a)"#, &mut ctx, "N/A",).unwrap()
    );
    assert_eq!(
        Primitive::String("array".to_string()),
        compute(r#"type_of(b)"#, &mut ctx, "N/A",).unwrap()
    );
    assert_eq!(
        Primitive::String("bool".to_string()),
        compute(r#"type_of(c)"#, &mut ctx, "N/A",).unwrap()
    );
    assert_eq!(
        Primitive::String("string".to_string()),
        compute(r#"type_of(d)"#, &mut ctx, "N/A",).unwrap()
    );
    assert_eq!(
        Primitive::String("unit".to_string()),
        compute(r#"type_of(e)"#, &mut ctx, "N/A",).unwrap()
    );
    assert_eq!(
        Primitive::String("!".to_string()),
        compute(r#"type_of(f)"#, &mut ctx, "N/A",).unwrap()
    );
    assert_eq!(
        Primitive::String("int".to_string()),
        compute(r#"type_of(g)"#, &mut ctx, "N/A",).unwrap()
    );
}
