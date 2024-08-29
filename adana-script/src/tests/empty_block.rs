use std::collections::BTreeMap;

use crate::compute;

use adana_script_core::primitive::Primitive;
#[test]
fn empty_while() {
    let expr = r#"
         x =1024
         y = 3
         while (x!=1024) {}
         x
       "#;
    let mut ctx = BTreeMap::new();
    let res = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(Primitive::Int(1024), ctx["x"].read().unwrap().clone());
    assert_eq!(Primitive::Int(1024), res);
}
#[test]
fn empty_if() {
    let expr = r#"
         x =1024
         y = 3
         if (x==1024) {}
         if (x==1024) {}else {}
         x
       "#;
    let mut ctx = BTreeMap::new();
    let res = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(Primitive::Int(1024), ctx["x"].read().unwrap().clone());
    assert_eq!(Primitive::Int(1024), res);
}

#[test]
fn empty_for() {
    let expr = r#"
         x =1024
         y = 3
         for i in 0..x {}
         x
    "#;
    let mut ctx = BTreeMap::new();
    let res = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(Primitive::Int(1024), ctx["x"].read().unwrap().clone());
    assert_eq!(Primitive::Int(1024), res);
}
