use std::collections::BTreeMap;

use serial_test::serial;

use crate::adana_script::{
    compute,
    primitive::{
        Array as Arr,
        Primitive::{Array, Bool, Double, Int, String as Str},
    },
    Primitive,
};

#[test]
#[serial]
fn test_simple_array() {
    let file_path = r#"
        include("file_tests/test_simple_array.adana")
    "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(file_path, &mut ctx).unwrap();
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
                    Int(1),
                    Str("World".to_string()),
                    Bool(true)
                ])
            ),
            (
                "y".to_string(),
                Array(vec![
                    Str("hello".to_string()),
                    Int(1),
                    Str("World".to_string()),
                    Bool(true),
                    Str("hello".to_string()),
                    Int(1),
                    Str("World".to_string()),
                    Bool(true),
                    Str("hello".to_string()),
                    Int(1),
                    Str("World".to_string()),
                    Bool(true),
                    Str("hello".to_string()),
                    Int(1),
                    Str("World".to_string()),
                    Bool(true),
                    Str("hello".to_string()),
                    Int(1),
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
    let _ = compute(file_path, &mut ctx).unwrap();
    assert_eq!(ctx["arrlen"].read().unwrap().clone(), Int(18));

    let arr = Array(vec![
        Str("a".to_string()),
        Bool(true),
        Str("bababa".to_string()),
        Str("zezezeze".to_string()),
        Int(1),
        Double(2.1),
        Double(3.0),
        Int(69),
        Int(420),
        Str("Yolo".to_string()),
        Bool(true),
        Str("bababa".to_string()),
        Str("zezezeze".to_string()),
        Int(1),
        Double(2.1),
        Double(3.0),
        Int(69),
        Int(420),
    ]);

    let mut copy = arr.clone();
    copy.swap_mem(&mut Str("a".to_string()), &Int(9));

    assert_eq!(ctx["arr"].read().unwrap().clone(), arr);
    assert_eq!(ctx["copy"].read().unwrap().clone(), copy);

    let fancy_list = Array(vec![
        Int(1),
        Array(vec![
            Int(2),
            Array(vec![
                Int(3),
                Array(vec![
                    Int(4),
                    Array(vec![
                        Int(5),
                        Array(vec![
                            Int(6),
                            Array(vec![
                                Int(7),
                                Array(vec![
                                    Int(8),
                                    Array(vec![Int(9), Array(vec![])]),
                                ]),
                            ]),
                        ]),
                    ]),
                ]),
            ]),
        ]),
    ]);
    assert_eq!(ctx["list"].read().unwrap().clone(), fancy_list);

    let res = compute("arr[2]", &mut ctx).unwrap();

    assert_eq!(Str("bababa".to_string()), res)
}

#[test]
#[serial]
fn test_string_array() {
    let file_path = r#"
        include("file_tests/test_string_arr.adana")
    "#;
    let mut ctx = BTreeMap::new();
    let _ = compute(file_path, &mut ctx).unwrap();
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
    let r = compute(expr, &mut ctx).unwrap();
    assert_eq!(ctx["x"].read().unwrap().clone(), Primitive::Int(1));
    assert_eq!(r, Primitive::Int(1));
}

#[test]
#[serial]
fn test_array_expr_access_not_assigned() {
    let expr = r#"
           [1, 2, 3][1]
        "#;
    let mut ctx = BTreeMap::new();
    let r = compute(expr, &mut ctx).unwrap();
    assert_eq!(r, Primitive::Int(2));
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
    let _ = compute(expr, &mut ctx).unwrap();

    assert_eq!(
        ctx["arr_ints"].read().unwrap().clone(),
        Array(vec![
            Int(-1),
            Int(1),
            Int(2),
            Int(3),
            Int(7),
            Int(8),
            Int(9),
            Int(12),
            Int(19),
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
