use std::collections::BTreeMap;

use crate::adana_script::compute;
use adana_script_core::primitive::Primitive::*;
#[test]
fn test_example1() {
    let mut ctx = BTreeMap::new();

    let expr = include_str!("../../../examples/example1.adana");

    let res = compute(expr, &mut ctx, "N/A").unwrap();

    assert_eq!(
        ctx["students"].read().unwrap().clone(),
        Array(vec![
            Struct(BTreeMap::from([
                ("first_name".into(), String("John".into(),)),
                ("last_name".into(), String("Doe".into(),)),
                ("note".into(), U8(18,)),
            ])),
            Struct(BTreeMap::from([
                ("first_name".into(), String("Jane".into(),)),
                ("last_name".into(), String("Dow".into(),)),
                ("note".into(), U8(9,)),
            ])),
            Struct(BTreeMap::from([
                ("first_name".into(), String("Bryan".into(),)),
                ("last_name".into(), String("Bower".into(),)),
                ("note".into(), I8(-10,)),
            ])),
        ],)
    );
    assert_eq!(
        ctx["sorted_students"].read().unwrap().clone(),
        Array(vec![
            Struct(BTreeMap::from([
                ("first_name".into(), String("Bryan".into(),)),
                ("last_name".into(), String("Bower".into(),)),
                ("note".into(), I8(-10,)),
            ])),
            Struct(BTreeMap::from([
                ("first_name".into(), String("Jane".into(),)),
                ("last_name".into(), String("Dow".into(),)),
                ("note".into(), U8(9,)),
            ])),
            Struct(BTreeMap::from([
                ("first_name".into(), String("John".into(),)),
                ("last_name".into(), String("Doe".into(),)),
                ("note".into(), U8(18,)),
            ])),
        ],)
    );

    assert_eq!(ctx["sorted_students"].read().unwrap().clone(), res);
}

#[test]
fn test_example2() {
    let mut ctx = BTreeMap::new();
    let expr = include_str!("../../../examples/example2.adana");
    let res = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(res, Array(vec![Int(15), Int(1), Int(11), Int(22)]));
    assert_eq!(
        compute("fun([1])", &mut ctx, "N/A").unwrap(),
        Array(vec![U8(0)])
    );
}
