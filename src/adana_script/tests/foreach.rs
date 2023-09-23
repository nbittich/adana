use std::collections::BTreeMap;

use crate::adana_script::compute;

use adana_script_core::primitive::Primitive;
#[test]
fn simple_foreach() {
    let expr = r#"
         arr = [1,2,3,4]
         total = 0
         for a in arr {
             total = total + a
         }
       "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(Primitive::Int(10), ctx["total"].read().unwrap().clone());
    assert!(ctx.get("a").is_none());
}

#[test]
fn simple_foreach_string() {
    use crate::Primitive::{Array, String};
    let expr = r#"
         arr = "Salut le monde j'espère que vous allez bien"
         total = 0
         reverted = []
         for a in arr {
             total = total + 1
             reverted =  a + reverted
         }
       "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(Primitive::Int(43), ctx["total"].read().unwrap().clone());
    assert_eq!(
        Array(vec![
            String("n".to_string()),
            String("e".to_string()),
            String("i".to_string()),
            String("b".to_string()),
            String(" ".to_string()),
            String("z".to_string()),
            String("e".to_string()),
            String("l".to_string()),
            String("l".to_string()),
            String("a".to_string()),
            String(" ".to_string()),
            String("s".to_string()),
            String("u".to_string()),
            String("o".to_string()),
            String("v".to_string()),
            String(" ".to_string()),
            String("e".to_string()),
            String("u".to_string()),
            String("q".to_string()),
            String(" ".to_string()),
            String("e".to_string()),
            String("r".to_string()),
            String("è".to_string()),
            String("p".to_string()),
            String("s".to_string()),
            String("e".to_string()),
            String("'".to_string()),
            String("j".to_string()),
            String(" ".to_string()),
            String("e".to_string()),
            String("d".to_string()),
            String("n".to_string()),
            String("o".to_string()),
            String("m".to_string()),
            String(" ".to_string()),
            String("e".to_string()),
            String("l".to_string()),
            String(" ".to_string()),
            String("t".to_string()),
            String("u".to_string()),
            String("l".to_string()),
            String("a".to_string()),
            String("S".to_string())
        ]),
        ctx["reverted"].read().unwrap().clone()
    );
    assert!(ctx.get("a").is_none());
}
#[test]
fn simple_foreach_not_assigned() {
    let expr = r#"
         message = ""
         for word in ["Hello","World","How","Are", "Ya","?"] {
             if (length(message) == 0) {
                 message = word
             } else {
                message = message + " " + word
             }
         }
       "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(
        Primitive::String("Hello World How Are Ya ?".into()),
        ctx["message"].read().unwrap().clone()
    );
    assert!(ctx.get("word").is_none());
}

#[test]
fn simple_foreach_string_not_assigned() {
    let expr = r#"
         message = ""
         for letter in "Hello World How Are Ya ?" {
            message = message + letter
         }
       "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(
        Primitive::String("Hello World How Are Ya ?".into()),
        ctx["message"].read().unwrap().clone()
    );
    assert!(ctx.get("letter").is_none());
}

#[test]
fn simple_foreach_break() {
    let expr = r#"
         message = ""
         for word in ["Hello","World","How","Are", "Ya","?"] {
             if (length(message) == 0) {
                 message = word
             } else {
                message = message + " " + word
                if(word == "Ya") {
                   break
                }
             }
         }
       "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(
        Primitive::String("Hello World How Are Ya".into()),
        ctx["message"].read().unwrap().clone()
    );
    assert!(ctx.get("word").is_none());
}

#[test]
fn simple_foreach_return() {
    let expr = r#"
         message = ""
         for word in ["Hello","World","How","Are", "Ya","?"] {
             if (length(message) == 0) {
                 message = word
             } else {
                if(word == "Ya") {
                   return
                }
                message = message + " " + word
             }
         }
       "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(
        Primitive::String("Hello World How Are".into()),
        ctx["message"].read().unwrap().clone()
    );
    assert!(ctx.get("word").is_none());
}
#[test]
fn simple_foreach_two_depth() {
    use crate::Primitive::Int;
    let expr = r#"
         x_arr = [5,10,15,20,25]
         y_arr = [0,1,0,0,1]
         matrix = []
         for x in x_arr {
             depth = [x]
             for y in y_arr {
                 depth = depth + [y]
             }
             matrix = matrix + [depth]
         }
       "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(
        Primitive::Array(vec![
            Primitive::Array(vec![
                Primitive::Int(5),
                Int(0),
                Int(1),
                Int(0),
                Int(0),
                Int(1)
            ]),
            Primitive::Array(vec![
                Int(10),
                Int(0),
                Int(1),
                Int(0),
                Int(0),
                Int(1)
            ]),
            Primitive::Array(vec![
                Int(15),
                Int(0),
                Int(1),
                Int(0),
                Int(0),
                Int(1)
            ]),
            Primitive::Array(vec![
                Int(20),
                Int(0),
                Int(1),
                Int(0),
                Int(0),
                Int(1)
            ]),
            Primitive::Array(vec![
                Int(25),
                Int(0),
                Int(1),
                Int(0),
                Int(0),
                Int(1)
            ]),
        ]),
        ctx["matrix"].read().unwrap().clone()
    );
    assert!(ctx.get("depth").is_none());
}

#[test]
fn simple_foreach_with_idx() {
    let expr = r#"
         arr = [1,2,3,4]
         total = 0
         idx_total = 0
         for index,a in arr {
             total = total + a
             idx_total = idx_total + index
         }
       "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(Primitive::Int(10), ctx["total"].read().unwrap().clone());
    assert_eq!(Primitive::Int(6), ctx["idx_total"].read().unwrap().clone());
    assert!(ctx.get("a").is_none());
}
#[test]
fn simple_foreach_with_idx_from_fn() {
    let expr = r#"
         arr = () => { [1,2,3,4] }
         total = 0
         idx_total = 0
         for index, a in arr() {
             total = total + a
             idx_total = idx_total + index
         }
       "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(Primitive::Int(10), ctx["total"].read().unwrap().clone());
    assert_eq!(Primitive::Int(6), ctx["idx_total"].read().unwrap().clone());
    assert!(ctx.get("a").is_none());
}
#[test]
fn simple_foreach_with_idx_from_struct() {
    let expr = r#"
         struc = struct {
             arr: [1, 2, 3, 4]
         }
         total = 0
         idx_total = 0
         for index,a in struc.arr {
             total = total + a
             idx_total = idx_total + index
         }
       "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(Primitive::Int(10), ctx["total"].read().unwrap().clone());
    assert_eq!(Primitive::Int(6), ctx["idx_total"].read().unwrap().clone());
    assert!(ctx.get("a").is_none());
}

#[test]
fn simple_foreach_with_paren() {
    let expr = r#"
         arr = [1,2,3,4]
         total = 0
         for (a in arr) {
             total = total + a
         }
       "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(Primitive::Int(10), ctx["total"].read().unwrap().clone());
    assert!(ctx.get("a").is_none());
}

#[test]
fn simple_foreach_with_idx_with_paren() {
    let expr = r#"
         arr = [1,2,3,4]
         total = 0
         idx_total = 0
         for (index ,   a in arr) {
             total = total + a
             idx_total = idx_total + index
         }
       "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(Primitive::Int(10), ctx["total"].read().unwrap().clone());
    assert_eq!(Primitive::Int(6), ctx["idx_total"].read().unwrap().clone());
    assert!(ctx.get("a").is_none());
}

#[test]
fn simple_foreach_with_idx_from_struct_with_paren() {
    let expr = r#"
         struc = struct {
             arr: [1, 2, 3, 4],
         }
         total = 0
         idx_total = 0
         for (index,a in struc.arr) { # comment
             total = total + a
                 idx_total = idx_total + index # comment
         }
       "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(Primitive::Int(10), ctx["total"].read().unwrap().clone());
    assert_eq!(Primitive::Int(6), ctx["idx_total"].read().unwrap().clone());
    assert!(ctx.get("a").is_none());
}

#[test]
fn test_handle_error() {
    let expr = r#"
        x = 1
        for n in x {
            println(n)
        }
        "#;

    let mut ctx = BTreeMap::new();
    let r = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(Primitive::Error("not an iterable Int(1)".into()), r);
}

#[test]
fn test_foreach_struct() {
    let expr = r#"
        result = [] 
        s = struct {
            name: "nordine",
            age: 34,
            members: ["natalie", "roger","fred"]
        }
        for  id, entry in s {
            result = result + ("Id: "+id +" Key: "+entry.key + " Value: " +to_string(entry.value))
        }

        "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(expr, &mut ctx, "N/A").unwrap();

    let result = ctx["result"].read().unwrap();
    let _expected = vec![
        Primitive::String(
            r#"Id: 0 Key: members Value: ["natalie", "roger", "fred"]"#.into(),
        ),
        Primitive::String("Id: 1 Key: age Value: 34".into()),
        Primitive::String("Id: 2 Key: name Value: nordine".into()),
    ];
    let actual = result.as_ref_ok().unwrap();
    assert!(matches!(actual, &Primitive::Array(_)));
    //assert!(actual.0)
}
