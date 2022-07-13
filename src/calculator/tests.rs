use std::collections::BTreeMap;

use crate::calculator::{parser::parse_var_expr, Operator::*, Value};

use super::{compute, Primitive};

#[test]
#[should_panic(expected = "invalid expression!")]
fn test_expr_invalid() {
    let expr = "use example";
    let mut ctx = BTreeMap::from([("x".to_string(), Primitive::Double(2.))]);
    compute(expr, &mut ctx).unwrap();
}
#[test]
#[should_panic(expected = "invalid expression!")]
fn test_expr_invalid_drc() {
    let expr = "drc logs -f triplestore";
    let mut ctx = BTreeMap::from([("x".to_string(), Primitive::Double(2.))]);
    compute(expr, &mut ctx).unwrap();
}

#[test]
#[should_panic(expected = "Invalid operation!")]
fn test_op_invalid() {
    let expr = "use example = wesh";
    let mut ctx = BTreeMap::from([("x".to_string(), Primitive::Double(2.))]);
    compute(expr, &mut ctx).unwrap();
}

#[test]
fn test_compute_with_ctx() {
    let expr = "x * 5";
    let mut ctx = BTreeMap::from([("x".to_string(), Primitive::Double(2.))]);

    let res = compute(expr, &mut ctx).unwrap();
    assert_eq!(Primitive::Double(10.), res);
}
#[test]
fn test_compute_assign_with_ctx() {
    let expr = "y = x * 5";
    let mut ctx = BTreeMap::from([("x".to_string(), Primitive::Double(2.))]);

    let res = compute(expr, &mut ctx).unwrap();
    assert_eq!(Primitive::Double(10.), res);

    assert_eq!(ctx.get("y"), Some(&Primitive::Double(10.)));
}

#[test]
fn test_variable() {
    let expr = "x*5+9*y/8";
    let (_, op) = parse_var_expr(expr).unwrap();
    assert_eq!(
        op,
        Value::Expression(vec![
            Value::Variable("x",),
            Value::Operation(Mult,),
            Value::Integer(5,),
            Value::Operation(Add,),
            Value::Integer(9,),
            Value::Operation(Mult,),
            Value::Variable("y",),
            Value::Operation(Div,),
            Value::Integer(8,),
        ],),
    );
}
#[test]
fn test_variable_expr() {
    let expr = "z = x*5+9*y/8";
    let (_, op) = parse_var_expr(expr).unwrap();
    assert_eq!(
        op,
        Value::VariableExpr {
            name: Box::new(Value::Variable("z")),
            expr: Box::new(Value::Expression(vec![
                Value::Variable("x",),
                Value::Operation(Mult,),
                Value::Integer(5,),
                Value::Operation(Add,),
                Value::Integer(9,),
                Value::Operation(Mult,),
                Value::Variable("y",),
                Value::Operation(Div,),
                Value::Integer(8,),
            ]))
        },
    );
}

#[test]
fn test_modulo() {
    let mut ctx = BTreeMap::new();
    assert_eq!(Primitive::Int(1), compute("3%2", &mut ctx).unwrap());
    assert_eq!(Primitive::Double(1.), compute("3%2.", &mut ctx).unwrap());
    assert_eq!(Primitive::Double(0.625), compute("5/8.%2", &mut ctx).unwrap());
    assert_eq!(
        Primitive::Double(3278.9),
        compute("2* (9*(5-(1/2.))) ^2 -1 / 5. * 8 - 4 %4", &mut ctx).unwrap()
    );
    assert_eq!(
        Primitive::Double(-1.1),
        compute("2* (9*(5-(1/2.))) ^2 %2 -1 / 5. * 8 - 4 %4", &mut ctx)
            .unwrap()
    );
}

#[test]
fn test_compute() {
    let mut ctx = BTreeMap::new();
    assert_eq!(
        Primitive::Double(3280.3),
        compute("x=2* (9*(5-(1./2.))) ^2 -1 / 5.", &mut ctx).unwrap()
    );
    assert_eq!(
        Primitive::Double(3274.9),
        compute("y = 2* (9*(5-(1/2.))) ^2 -1 / 5. * 8 - 4", &mut ctx).unwrap()
    );
    assert_eq!(
        Primitive::Double(-670.9548307564088),
        compute("z = 78/5.-4.5*(9+7^2.5)-12*4+1-8/3.*4-5", &mut ctx).unwrap()
    );
    assert_eq!(
            Primitive::Int(37737),
            compute("f = 1988*19-(((((((9*2))))+2*4)-3))/6-1^2*1000/(7-4*(3/9-(9+3/2-4)))", &mut ctx).unwrap()
        );
    assert_eq!(
            Primitive::Double(37736.587719298244),
            compute("f = 1988*19-(((((((9*2))))+2*4)-3))/6.-1^2*1000/(7-4*(3/9.-(9+3/2.-4)))", &mut ctx).unwrap()
        );
    assert_eq!(Primitive::Int(0), compute("0", &mut ctx).unwrap());
    assert_eq!(Primitive::Int(9), compute("9", &mut ctx).unwrap());
    assert_eq!(Primitive::Int(-9), compute("-9", &mut ctx).unwrap());
    assert_eq!(
        Primitive::Int(6 / 2 * (2 + 1)),
        compute("6/2*(2+1)", &mut ctx).unwrap()
    );
    assert_eq!(
        Primitive::Double(2. - 1. / 5.),
        compute("2 -1 / 5.", &mut ctx).unwrap()
    );
    // todo maybe should panic in these cases
    assert_eq!(Primitive::Int(2 * 4), compute("2* * *4", &mut ctx).unwrap());
    assert_eq!(Primitive::Int(2 * 4), compute("2* ** *4", &mut ctx).unwrap());
    assert_eq!(Primitive::Int(4), compute("*4", &mut ctx).unwrap());

    // compute with variables
    assert_eq!(
            Primitive::Double(-4765.37866215695),
            compute("f = 555*19-(((((((9*2))))+2*f)-x))/6.-1^2*y/(z-4*(3/9.-(9+3/2.-4))) - x", &mut ctx).unwrap()
        );

    assert_eq!(ctx.get("f"), Some(&Primitive::Double(-4765.37866215695)));
    assert_eq!(ctx.get("z"), Some(&Primitive::Double(-670.9548307564088)));
    assert_eq!(ctx.get("y"), Some(&Primitive::Double(3274.9)));
    assert_eq!(ctx.get("x"), Some(&Primitive::Double(3280.3)));
}

#[test]
fn test_negate() {
    let mut ctx = BTreeMap::new();
    assert_eq!(Primitive::Int(-5 / -1), compute("-5/-1", &mut ctx).unwrap());
    assert_eq!(Primitive::Int(5 / -1), compute("5/-1", &mut ctx).unwrap());
    assert_eq!(Primitive::Int(--5), compute("--5", &mut ctx).unwrap());
}
#[test]
fn test_pow() {
    let mut ctx = BTreeMap::new();
    assert_eq!(Primitive::Double(-0.5), compute("-2^-1", &mut ctx).unwrap());
    assert_eq!(Primitive::Double(-0.04), compute("-5^-2", &mut ctx).unwrap());
    assert_eq!(Primitive::Int(-25), compute("-5^2", &mut ctx).unwrap());
    assert_eq!(Primitive::Double(0.04), compute("5^-2", &mut ctx).unwrap());
    assert_eq!(Primitive::Int(3125), compute("5^5", &mut ctx).unwrap());
    assert_eq!(Primitive::Int(1), compute("5^0", &mut ctx).unwrap());
}

#[test]
fn test_consts() {
    let mut ctx = BTreeMap::new();
    assert_eq!(
        Primitive::Double(std::f64::consts::PI),
        compute("π", &mut ctx).unwrap()
    );
    assert_eq!(
        Primitive::Double(std::f64::consts::PI * 2.),
        compute("π*2", &mut ctx).unwrap()
    );
    assert_eq!(
        Primitive::Double(std::f64::consts::E),
        compute("γ", &mut ctx).unwrap()
    );
    assert_eq!(
        Primitive::Double(std::f64::consts::TAU),
        compute("τ", &mut ctx).unwrap()
    );
}
#[test]
fn test_fn_sqrt() {
    let mut ctx = BTreeMap::new();
    assert_eq!(
        Primitive::Double(2.23606797749979),
        compute("sqrt(5)", &mut ctx).unwrap()
    );
    assert_eq!(Primitive::Double(5.), compute("sqrt(5*5)", &mut ctx).unwrap());
    assert_eq!(
        Primitive::Double(8983.719357816115),
        compute("sqrt(2*(3/4.-12%5 +7^9) --6/12.*4)", &mut ctx).unwrap()
    );
}

#[test]
fn test_fn_abs() {
    let mut ctx = BTreeMap::new();
    assert_eq!(Primitive::Int(5), compute("abs(5)", &mut ctx).unwrap());
    assert_eq!(Primitive::Int(25), compute("abs(-5*5)", &mut ctx).unwrap());
    assert_eq!(
        Primitive::Double(80707209.5),
        compute("abs(-2*(3/4.-12%5 +7^9) --6/12.*4)", &mut ctx).unwrap()
    );
}
#[test]
fn test_fn_log() {
    let mut ctx = BTreeMap::new();
    assert_eq!(
        Primitive::Double(0.6989700043360189),
        compute("log(5)", &mut ctx).unwrap()
    );
    assert_eq!(
        Primitive::Double(1.3979400086720377),
        compute("log(abs(-5*5))", &mut ctx).unwrap()
    );
    assert_eq!(
        Primitive::Double(7.906912331577292),
        compute("log(abs(-2*(3/4.-12%5 +7^9) --6/12.*4))", &mut ctx).unwrap()
    );
}

#[test]
fn test_fn_ln() {
    let mut ctx = BTreeMap::new();
    assert_eq!(
        Primitive::Double(1.6094379124341003),
        compute("ln(5)", &mut ctx).unwrap()
    );
    assert_eq!(
        Primitive::Double(3.2188758248682006),
        compute("ln(abs(-5*5))", &mut ctx).unwrap()
    );
    assert_eq!(
        Primitive::Double(18.206338466300664),
        compute("ln(abs(-2*(3/4.-12%5 +7^9) --6/12.*4))", &mut ctx).unwrap()
    );
}
#[test]
fn test_fn_sin() {
    let mut ctx = BTreeMap::new();
    assert_eq!(
        Primitive::Double(-0.9589242746631385),
        compute("sin(5)", &mut ctx).unwrap()
    );
    assert_eq!(
        Primitive::Double(0.13235175009777303),
        compute("sin(-5*5)", &mut ctx).unwrap()
    );
    assert_eq!(
        Primitive::Double(-0.8604918893688305),
        compute("sin(-2*(3/4.-12%5 +7^9) --6/12.*4)", &mut ctx).unwrap()
    );
}
#[test]
fn test_fn_cos() {
    let mut ctx = BTreeMap::new();
    assert_eq!(
        Primitive::Double(0.28366218546322625),
        compute("cos(5)", &mut ctx).unwrap()
    );
    assert_eq!(
        Primitive::Double(0.9912028118634736),
        compute("cos(abs(-5*5))", &mut ctx).unwrap()
    );
    assert_eq!(
        Primitive::Double(-0.509464138414531),
        compute("cos(abs(-2*(3/4.-12%5 +7^9) --6/12.*4))", &mut ctx).unwrap()
    );
}

#[test]
fn test_fn_tan() {
    let mut ctx = BTreeMap::new();
    assert_eq!(
        Primitive::Double(-3.380515006246586),
        compute("tan(5)", &mut ctx).unwrap()
    );
    assert_eq!(
        Primitive::Double(-0.13352640702153587),
        compute("tan(abs(-5*5))", &mut ctx).unwrap()
    );
    assert_eq!(
        Primitive::Double(-1.6890136606017243),
        compute("tan(abs(-2*(3/4.-12%5 +7^9) --6/12.*4))", &mut ctx).unwrap()
    );
}

#[test]
fn test_extra() {
    let mut ctx = BTreeMap::new();
    assert_eq!(
        Primitive::Double(44721.45950030539),
        compute("sqrt((2*10^9-5*abs(8/9.))) + abs(1/10.)", &mut ctx).unwrap()
    );
    assert_eq!(
        Primitive::Double(161414423.89420456),
        compute(
            "
                2*(3/4.-12%5 +7^9) -6/12.*4 / 
                sqrt(2*(3/4.-12%5 +7^9) --6/12.*4) + 
                abs(-2*(3/4.-12%5 +7^9) -6/12.*4 / sqrt(5))
            ",
            &mut ctx
        )
        .unwrap()
    );
    assert_eq!(
        Primitive::Double(507098311.0925626),
        compute(
            "
                (2*(3/4.-12%5 +7^9) -6/12.*4 / 
                sqrt(2*(3/4.-12%5 +7^9) --6/12.*4) + 
                abs(-2*(3/4.-12%5 +7^9) -6/12.*4 / sqrt(5)) -
                ln(abs(-2*(3/4.-12%5 +7^9 --8*4^9. % 2) --6/12.*4))) * π

            ",
            &mut ctx
        )
        .unwrap()
    );
    assert_eq!(
        Primitive::Double(438769845.8328427),
        compute(
            "
                (2*(3/4.-12%5 +7^9) -6/12.*4 / 
                sqrt(2*(3/4.-12%5 +7^9) --6/12.*4) + 
                abs(-2*(3/4.-12%5 +7^9) -6/12.*4 / sqrt(5)) -
                ln(abs(-2*(3/4.-12%5 +7^9 --8*4^9. % 2) --6/12.*4))) * γ

            ",
            &mut ctx
        )
        .unwrap()
    );
    assert_eq!(Primitive::Double(-1.), compute("cos(π)", &mut ctx).unwrap());
}
