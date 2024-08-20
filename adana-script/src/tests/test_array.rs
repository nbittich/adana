use std::collections::BTreeMap;

use serial_test::serial;

use crate::compute;
use adana_script_core::primitive::{
    Array as Arr,
    Primitive::{self, Array, Bool, Double, Int, String as Str, I8, U8},
};
#[test]
#[serial]
fn test_simple_array() {
    let file_path = r#"
        include("file_tests/test_simple_array.adana")
    "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(file_path, &mut ctx, "N/A").unwrap();
    assert_eq!(ctx.len(), 4);
    let ctx: BTreeMap<String, Primitive> = ctx
        .iter()
        .map(|(k, v)| (k.to_string(), v.read().unwrap().clone()))
        .collect();
    assert_eq!(
        &BTreeMap::from([
            (
                "x".to_string(),
                Array(vec![
                    Str("hello".to_string()),
                    U8(1),
                    Str("World".to_string()),
                    Bool(true)
                ])
            ),
            (
                "y".to_string(),
                Array(vec![
                    Str("hello".to_string()),
                    U8(1),
                    Str("World".to_string()),
                    Bool(true),
                    Str("hello".to_string()),
                    U8(1),
                    Str("World".to_string()),
                    Bool(true),
                    Str("hello".to_string()),
                    U8(1),
                    Str("World".to_string()),
                    Bool(true),
                    Str("hello".to_string()),
                    U8(1),
                    Str("World".to_string()),
                    Bool(true),
                    Str("hello".to_string()),
                    U8(1),
                    Str("World".to_string()),
                    Bool(true)
                ])
            ),
            ("z".to_string(), Str("World".to_string())),
            ("a".to_string(), Bool(true)),
        ]),
        &ctx
    );
}
#[test]
#[serial]
fn test_file_array() {
    let file_path = r#"
    include("file_tests/test_array.adana")
    "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(file_path, &mut ctx, "N/A").unwrap();
    assert_eq!(ctx["arrlen"].read().unwrap().clone(), Int(18));

    let arr = Array(vec![
        Str("a".to_string()),
        Bool(true),
        Str("bababa".to_string()),
        Str("zezezeze".to_string()),
        Primitive::U8(1),
        Double(2.1),
        Double(3.0),
        Primitive::U8(69),
        Int(420),
        Str("Yolo".to_string()),
        Bool(true),
        Str("bababa".to_string()),
        Str("zezezeze".to_string()),
        Primitive::U8(1),
        Double(2.1),
        Double(3.0),
        Primitive::U8(69),
        Int(420),
    ]);

    let mut copy = arr.clone();
    copy.swap_mem(&mut Str("a".to_string()), &Primitive::U8(9));

    assert_eq!(ctx["arr"].read().unwrap().clone(), arr);
    assert_eq!(ctx["copy"].read().unwrap().clone(), copy);

    let fancy_list = Array(vec![
        Primitive::U8(1),
        Array(vec![
            Primitive::U8(2),
            Array(vec![
                Primitive::U8(3),
                Array(vec![
                    Primitive::U8(4),
                    Array(vec![
                        Primitive::U8(5),
                        Array(vec![
                            Primitive::U8(6),
                            Array(vec![
                                Primitive::U8(7),
                                Array(vec![
                                    Primitive::U8(8),
                                    Array(vec![
                                        Primitive::U8(9),
                                        Array(vec![]),
                                    ]),
                                ]),
                            ]),
                        ]),
                    ]),
                ]),
            ]),
        ]),
    ]);
    assert_eq!(ctx["list"].read().unwrap().clone(), fancy_list);

    let res = compute("arr[2]", &mut ctx, "N/A").unwrap();

    assert_eq!(Str("bababa".to_string()), res)
}

#[test]
#[serial]
fn test_string_array() {
    let file_path = r#"
        include("file_tests/test_string_arr.adana")
    "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(file_path, &mut ctx, "N/A").unwrap();
    let ctx: BTreeMap<String, Primitive> = ctx
        .iter()
        .map(|(k, v)| (k.to_string(), v.read().unwrap().clone()))
        .collect();
    assert_eq!(
        &BTreeMap::from([
            ("v".to_string(), Str("nordine".to_string())),
            ("copy".to_string(), Str("eodrnin".to_string())),
            ("s".to_string(), Str("kekeke".to_string())),
            ("count".to_string(), Int(7)),
            ("i".to_string(), Int(6)),
        ]),
        &ctx
    );
}

#[test]
#[serial]
fn test_array_expr_access() {
    let expr = r#"
           x = [1, 2, 3][0]
        "#;
    let mut ctx = BTreeMap::new();
    let r = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(ctx["x"].read().unwrap().clone(), Primitive::U8(1));
    assert_eq!(r, Primitive::U8(1));
}

#[test]
#[serial]
fn test_array_expr_access_not_assigned() {
    let expr = r#"
           [1, 2, 3][1]
        "#;
    let mut ctx = BTreeMap::new();
    let r = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(r, Primitive::U8(2));
}
#[test]
#[serial]
fn test_array_access_expr() {
    let expr = r#"
        include("file_tests/sort.adana")
        arr_ints = [9,2,8,19,3,7,1,-1,12]
        arr_str = ["s","b","z","a","d","f","j","h"]
        arr_ints = sort(arr_ints)
        arr_str = sort(arr_str)
    "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(expr, &mut ctx, "N/A").unwrap();

    assert_eq!(
        ctx["arr_ints"].read().unwrap().clone(),
        Array(vec![
            I8(-1),
            U8(1),
            U8(2),
            U8(3),
            U8(7),
            U8(8),
            U8(9),
            U8(12),
            U8(19),
        ])
    );

    assert_eq!(
        ctx["arr_str"].read().unwrap().clone(),
        Array(vec![
            Str("a".into()),
            Str("b".into()),
            Str("d".into()),
            Str("f".into()),
            Str("h".into()),
            Str("j".into()),
            Str("s".into()),
            Str("z".into()),
        ])
    );
}
#[test]
#[serial]
fn test_array_access_from_fn_return() {
    let mut ctx = BTreeMap::new();
    let expr = r#"x = ()=> {[1,2,7,8,9]}() + 4"#;
    let _ = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(
        ctx["x"].read().unwrap().clone(),
        Primitive::Array(vec![
            Primitive::U8(1),
            Primitive::U8(2),
            Primitive::U8(7),
            Primitive::U8(8),
            Primitive::U8(9),
            Primitive::U8(4),
        ])
    );
}
#[test]
#[serial]
fn test_array_access_key_index() {
    let mut ctx = BTreeMap::new();
    let expr = r#"x = ()=> {[1,2,7,8,9]}()[3] + 4"#;
    let _ = compute(expr, &mut ctx, "N/A").unwrap();
    assert_eq!(ctx["x"].read().unwrap().clone(), Primitive::Int(12));
}
