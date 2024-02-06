use std::collections::BTreeMap;

use adana_script_core::primitive::Primitive;

use crate::compute;

#[test]
fn bitwise_or_test() {
    let mut ctx = BTreeMap::new();

    let r = compute(r#"1|2"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::U8(3));
    let r = compute(r#"1|1"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::U8(1));
    let r = compute(r#"1|0"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::U8(1));
    let r = compute(r#"0|0"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::U8(0));
    let r = compute(r#"127|135"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::U8(255));
    let r = compute(r#"127|9"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::U8(127));
    let r = compute(r#"-1|1"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::Int(-1));
    let r = compute(r#"-98|1"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::Int(-97));
}

#[test]
fn bitwise_xor_test() {
    let mut ctx = BTreeMap::new();

    let r = compute(r#"1$2"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::U8(3));
    let r = compute(r#"1$1"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::U8(0));
    let r = compute(r#"1$0"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::U8(1));
    let r = compute(r#"0$0"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::U8(0));
    let r = compute(r#"127$135"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::U8(248));
    let r = compute(r#"127$9"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::U8(118));
    let r = compute(r#"-1$1"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::Int(-2));
    let r = compute(r#"-98$1"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::Int(-97));
}

#[test]
fn bitwise_not() {
    let mut ctx = BTreeMap::new();

    let r = compute(r#"~255"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::Int(-256));
    let r = compute(r#"~127"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::Int(-128));
    let r = compute(r#"~128"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::Int(-129));
}

#[test]
fn bitwise_and() {
    let mut ctx = BTreeMap::new();

    let r = compute(r#"1@2"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::U8(1 & 2));
    let r = compute(r#"1@1"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::U8(1));
    let r = compute(r#"1@0"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::U8(0));
    let r = compute(r#"0@0"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::U8(0));
    let r = compute(r#"127@135"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::U8(127 & 135));
    let r = compute(r#"127@9"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::U8(127 & 9));
    let r = compute(r#"-1@1"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::Int(1));
    let r = compute(r#"-98@1"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::Int(-98 & 1));
}

#[test]
fn test_left_shift() {
    let mut ctx = BTreeMap::new();

    let r = compute(r#"1<<2"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::Int(1 << 2));
    let r = compute(r#"1 <<1"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::Int(1 << 1));
    let r = compute(r#"1<< 0"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::Int(1 << 0));
    let r = compute(r#"0 << 0"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::Int(0));
    // FIXME probably need to use a library for this
    // let r = compute(r#"127 <<135"#, &mut ctx, "N/A", ).unwrap();
    // assert_eq!(r, Primitive::Int((127u64 << 135u64) as i128));
    let r = compute(r#"127<<9"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::Int(127 << 9));
    let r = compute(r#"-1 << 1"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::Int(-1 << 1));
    let r = compute(r#"-98 << 1"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::Int(-98 << 1));
}

#[test]
fn test_right_shift() {
    let mut ctx = BTreeMap::new();

    let r = compute(r#"1>>2"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::Int(1 >> 2));
    let r = compute(r#"1 >>1"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::Int(1 >> 1));
    let r = compute(r#"1>> 0"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::Int(1));
    let r = compute(r#"0 >> 0"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::Int(0));
    // FIXME probably need to use a library for this
    // let r = compute(r#"127 >>135"#, &mut ctx, "N/A", ).unwrap();
    // assert_eq!(r, Primitive::Int((127u64 >> 135u64) as i128));
    let r = compute(r#"127>>9"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::Int(127 >> 9));
    let r = compute(r#"-1 >> 1"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::Int(-1 >> 1));
    let r = compute(r#"-98 >> 1"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::Int(-98 >> 1));
}
#[test]
fn test_complex_math_wise() {
    let mut ctx = BTreeMap::new();

    let r = compute(r#"30*9 @9 -5/~3"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::Int(((30 * 9) & (9 - 5 / !3)) as i128));

    let r = compute(r#"30*9 @9 -5/~3 << 1"#, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::Int(((30 * 9) & (9 - 5 / !3) << 1) as i128));
}
