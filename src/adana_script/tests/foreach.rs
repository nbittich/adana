use std::collections::BTreeMap;

use crate::adana_script::{compute, Primitive};

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
    let _ = compute(expr, &mut ctx).unwrap();
    assert_eq!(Primitive::Int(10), ctx["total"].lock().unwrap().clone());
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
    let _ = compute(expr, &mut ctx).unwrap();
    assert_eq!(Primitive::Int(43), ctx["total"].lock().unwrap().clone());
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
        ctx["reverted"].lock().unwrap().clone()
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
    let _ = compute(expr, &mut ctx).unwrap();
    assert_eq!(
        Primitive::String("Hello World How Are Ya ?".into()),
        ctx["message"].lock().unwrap().clone()
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
    let _ = compute(expr, &mut ctx).unwrap();
    assert_eq!(
        Primitive::String("Hello World How Are Ya ?".into()),
        ctx["message"].lock().unwrap().clone()
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
    let _ = compute(expr, &mut ctx).unwrap();
    assert_eq!(
        Primitive::String("Hello World How Are Ya".into()),
        ctx["message"].lock().unwrap().clone()
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
    let _ = compute(expr, &mut ctx).unwrap();
    assert_eq!(
        Primitive::String("Hello World How Are".into()),
        ctx["message"].lock().unwrap().clone()
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
    let _ = compute(expr, &mut ctx).unwrap();
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
        ctx["matrix"].lock().unwrap().clone()
    );
    assert!(ctx.get("depth").is_none());
}
