use std::collections::BTreeMap;

use adana_script_core::primitive::Primitive;

use crate::adana_script::compute;

#[test]
fn test_if_scope_simple() {
    let mut ctx = BTreeMap::new();
    let program = r#"
            x = 5
            if (x >= 5) {
                x = x -1
                z = 8
            }
        "#;
    let _ = compute(program, &mut ctx, "N/A").unwrap();
    assert!(!ctx.contains_key("z"));
    assert_eq!(Primitive::Int(4), ctx["x"].read().unwrap().clone());
}

#[test]
fn test_if_else_scope_simple() {
    let mut ctx = BTreeMap::new();
    let program = r#"
            x = 4
            if (x >= 5) {
                x = x -1
                z = 8
            }else{
                b = 12
                x = x +1
            }
        "#;
    let _ = compute(program, &mut ctx, "N/A").unwrap();
    assert_eq!(Primitive::Int(5), ctx["x"].read().unwrap().clone());
    assert!(!ctx.contains_key("z"));
    assert!(!ctx.contains_key("b"));
}
#[test]
fn test_if_scope_complex() {
    let mut ctx = BTreeMap::new();
    let program = r#"
            x = 6
            y = 8
            if (x >= 5) {
                x = x -1
                l = 9
                if(x == 5) {
                    if (l > 7) {
                        x = x + 2
                        y = y - 1
                        h = y - 9
                    }
                    f = x + y
                    # println(h)
                }
                x = x + 1
                y = y -1
                z = 8
            }
        "#;
    let _ = compute(program, &mut ctx, "N/A").unwrap();
    assert!(!ctx.contains_key("z"));
    assert!(!ctx.contains_key("l"));
    assert!(!ctx.contains_key("h"));
    assert!(!ctx.contains_key("f"));
    assert_eq!(Primitive::Int(8), ctx["x"].read().unwrap().clone());
    assert_eq!(Primitive::Int(6), ctx["y"].read().unwrap().clone());
}
#[test]
fn test_if_else_scope_complex() {
    let mut ctx = BTreeMap::new();
    let program = r#"
            x = 6
            y = 8
            if (x >= 5) {
                x = x -1
                l = 9
                if(x == 5) {
                    if (l < 7) {
                        x = x + 2
                        y = y - 1
                        h = y - 9
                    } else if(l == 8) {
                        x = x -1
                        y = 4
                        z = 2
                    }else if(l == 9) {
                        x = x +1
                        y = y -3
                        t = "salut"
                    }
                    f = x + y
                    # println(h)
                }
                x = x + 1
                y = y -1
                z = 8
            }
        "#;
    let _ = compute(program, &mut ctx, "N/A").unwrap();
    assert!(!ctx.contains_key("z"));
    assert!(!ctx.contains_key("l"));
    assert!(!ctx.contains_key("h"));
    assert!(!ctx.contains_key("f"));
    assert!(!ctx.contains_key("t"));
    assert_eq!(Primitive::Int(7), ctx["x"].read().unwrap().clone());
    assert_eq!(Primitive::Int(4), ctx["y"].read().unwrap().clone());
}

#[test]
fn test_while_scope_simple() {
    let mut ctx = BTreeMap::new();
    let program = r#"
            x = 5
            while (x >= 5) {
                x = x -1
                z = 8
            }
        "#;
    let _ = compute(program, &mut ctx, "N/A").unwrap();
    assert!(!ctx.contains_key("z"));
    assert_eq!(Primitive::Int(4), ctx["x"].read().unwrap().clone());
}
#[test]
fn test_while_scope_complex() {
    let mut ctx = BTreeMap::new();
    let program = r#"
            x = 5
            g = 9
            while (x >= 5) {
                x = x -1
                z = g
                while (z > 0) {
                     x = x -1
                     if(g == 9) {
                         g = g-1
                         p = 2
                     }
                     d = 3
                     z = z-1
                }
                
            }
        "#;
    let _ = compute(program, &mut ctx, "N/A").unwrap();
    assert!(!ctx.contains_key("z"));
    assert!(!ctx.contains_key("p"));
    assert!(!ctx.contains_key("d"));
    assert_eq!(Primitive::Int(-5), ctx["x"].read().unwrap().clone());
    assert_eq!(Primitive::Int(8), ctx["g"].read().unwrap().clone());
}
