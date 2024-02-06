use std::collections::BTreeMap;

use adana_script_core::primitive::Primitive;

use crate::compute;

#[test]
fn test_op_assign_add() {
    let mut ctx = BTreeMap::new();

    let script = r#"
        x = 2
        x+=1
        x
    "#;
    assert_eq!(Primitive::Int(3), compute(script, &mut ctx, "N/A").unwrap());
}

#[test]
fn test_op_assign_sub() {
    let mut ctx = BTreeMap::new();

    let script = r#"
        x = 2
        x-=1
        x
    "#;
    assert_eq!(Primitive::U8(1), compute(script, &mut ctx, "N/A").unwrap());
}

#[test]
fn test_op_assign_mul() {
    let mut ctx = BTreeMap::new();

    let script = r#"
        x = 2
        x*=2
        x
    "#;
    assert_eq!(Primitive::Int(4), compute(script, &mut ctx, "N/A").unwrap());
}

#[test]
fn test_op_assign_div() {
    let mut ctx = BTreeMap::new();

    let script = r#"
        x = 12
        x/=2
        x
    "#;
    assert_eq!(Primitive::Int(6), compute(script, &mut ctx, "N/A").unwrap());
}

#[test]
fn test_op_assign_mod() {
    let mut ctx = BTreeMap::new();

    let script = r#"
        x = 12
        x%=5
        x
    "#;
    assert_eq!(Primitive::U8(2), compute(script, &mut ctx, "N/A").unwrap());
}
