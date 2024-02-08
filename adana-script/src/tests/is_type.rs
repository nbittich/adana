use std::collections::BTreeMap;

use adana_script_core::primitive::Primitive;

use crate::compute;

#[test]
fn test_is_u8() {
    let mut ctx = BTreeMap::new();
    assert_eq!(
        Primitive::Bool(true),
        compute("is_u8(0x1)", &mut ctx, "N/A").unwrap()
    );
}
#[test]
fn test_is_i8() {
    let mut ctx = BTreeMap::new();
    assert_eq!(
        Primitive::Bool(true),
        compute("is_i8(-1)", &mut ctx, "N/A").unwrap()
    );
}
#[test]
fn test_is_int() {
    let mut ctx = BTreeMap::new();
    assert_eq!(
        Primitive::Bool(true),
        compute("is_int(512)", &mut ctx, "N/A").unwrap()
    );
}
#[test]
fn test_is_double() {
    let mut ctx = BTreeMap::new();
    assert_eq!(
        Primitive::Bool(true),
        compute("is_double(512.)", &mut ctx, "N/A").unwrap()
    );
}
#[test]
fn test_is_function() {
    let mut ctx = BTreeMap::new();
    assert_eq!(
        Primitive::Bool(true),
        compute("is_function(()=> {1})", &mut ctx, "N/A").unwrap()
    );
}
#[test]
fn test_is_struct() {
    let mut ctx = BTreeMap::new();
    assert_eq!(
        Primitive::Bool(true),
        compute("is_struct(struct {})", &mut ctx, "N/A").unwrap()
    );
}
#[test]
fn test_is_bool() {
    let mut ctx = BTreeMap::new();
    assert_eq!(
        Primitive::Bool(true),
        compute("is_bool(false)", &mut ctx, "N/A").unwrap()
    );
}
#[test]
fn test_is_array() {
    let mut ctx = BTreeMap::new();
    assert_eq!(
        Primitive::Bool(true),
        compute("is_array([1,2,3])", &mut ctx, "N/A").unwrap()
    );
}
#[test]
fn test_is_error() {
    let mut ctx = BTreeMap::new();
    assert_eq!(
        Primitive::Bool(true),
        compute("is_error(make_err(8))", &mut ctx, "N/A").unwrap()
    );
}
