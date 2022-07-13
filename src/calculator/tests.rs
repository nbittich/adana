use std::collections::BTreeMap;

use crate::calculator::{parser::parse_var_expr, Operator::*, Value};

use super::{compute, Number};

#[test]
#[should_panic(expected = "invalid expression!")]
fn test_expr_invalid() {
    let expr = "use example";
    let mut ctx = BTreeMap::from([("x".to_string(), Number::Double(2.))]);
    compute(expr, &mut ctx).unwrap();
}
#[test]
#[should_panic(expected = "invalid expression!")]
fn test_expr_invalid_drc() {
    let expr = "drc logs -f triplestore";
    let mut ctx = BTreeMap::from([("x".to_string(), Number::Double(2.))]);
    compute(expr, &mut ctx).unwrap();
}

#[test]
#[should_panic(expected = "Invalid operation!")]
fn test_op_invalid() {
    let expr = "use example = wesh";
    let mut ctx = BTreeMap::from([("x".to_string(), Number::Double(2.))]);
    compute(expr, &mut ctx).unwrap();
}

#[test]
fn test_compute_with_ctx() {
    let expr = "x * 5";
    let mut ctx = BTreeMap::from([("x".to_string(), Number::Double(2.))]);

    let res = compute(expr, &mut ctx).unwrap();
    assert_eq!(Number::Double(10.), res);
}
#[test]
fn test_compute_assign_with_ctx() {
    let expr = "y = x * 5";
    let mut ctx = BTreeMap::from([("x".to_string(), Number::Double(2.))]);

    let res = compute(expr, &mut ctx).unwrap();
    assert_eq!(Number::Double(10.), res);

    assert_eq!(ctx.get("y"), Some(&Number::Double(10.)));
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
    assert_eq!(Number::Int(1), compute("3%2", &mut ctx).unwrap());
    assert_eq!(Number::Double(1.), compute("3%2.", &mut ctx).unwrap());
    assert_eq!(Number::Double(0.625), compute("5/8.%2", &mut ctx).unwrap());
    assert_eq!(
        Number::Double(3278.9),
        compute("2* (9*(5-(1/2.))) ^2 -1 / 5. * 8 - 4 %4", &mut ctx).unwrap()
    );
    assert_eq!(
        Number::Double(-1.1),
        compute("2* (9*(5-(1/2.))) ^2 %2 -1 / 5. * 8 - 4 %4", &mut ctx)
            .unwrap()
    );
}

#[test]
fn test_compute() {
    let mut ctx = BTreeMap::new();
    assert_eq!(
        Number::Double(3280.3),
        compute("x=2* (9*(5-(1./2.))) ^2 -1 / 5.", &mut ctx).unwrap()
    );
    assert_eq!(
        Number::Double(3274.9),
        compute("y = 2* (9*(5-(1/2.))) ^2 -1 / 5. * 8 - 4", &mut ctx).unwrap()
    );
    assert_eq!(
        Number::Double(-670.9548307564088),
        compute("z = 78/5.-4.5*(9+7^2.5)-12*4+1-8/3.*4-5", &mut ctx).unwrap()
    );
    assert_eq!(
            Number::Int(37737),
            compute("f = 1988*19-(((((((9*2))))+2*4)-3))/6-1^2*1000/(7-4*(3/9-(9+3/2-4)))", &mut ctx).unwrap()
        );
    assert_eq!(
            Number::Double(37736.587719298244),
            compute("f = 1988*19-(((((((9*2))))+2*4)-3))/6.-1^2*1000/(7-4*(3/9.-(9+3/2.-4)))", &mut ctx).unwrap()
        );
    assert_eq!(Number::Int(0), compute("0", &mut ctx).unwrap());
    assert_eq!(Number::Int(9), compute("9", &mut ctx).unwrap());
    assert_eq!(Number::Int(-9), compute("-9", &mut ctx).unwrap());
    assert_eq!(
        Number::Int(6 / 2 * (2 + 1)),
        compute("6/2*(2+1)", &mut ctx).unwrap()
    );
    assert_eq!(
        Number::Double(2. - 1. / 5.),
        compute("2 -1 / 5.", &mut ctx).unwrap()
    );
    // todo maybe should panic in these cases
    assert_eq!(Number::Int(2 * 4), compute("2* * *4", &mut ctx).unwrap());
    assert_eq!(Number::Int(2 * 4), compute("2* ** *4", &mut ctx).unwrap());
    assert_eq!(Number::Int(4), compute("*4", &mut ctx).unwrap());

    // compute with variables
    assert_eq!(
            Number::Double(-4765.37866215695),
            compute("f = 555*19-(((((((9*2))))+2*f)-x))/6.-1^2*y/(z-4*(3/9.-(9+3/2.-4))) - x", &mut ctx).unwrap()
        );

    assert_eq!(ctx.get("f"), Some(&Number::Double(-4765.37866215695)));
    assert_eq!(ctx.get("z"), Some(&Number::Double(-670.9548307564088)));
    assert_eq!(ctx.get("y"), Some(&Number::Double(3274.9)));
    assert_eq!(ctx.get("x"), Some(&Number::Double(3280.3)));
}

#[test]
fn test_negate() {
    let mut ctx = BTreeMap::new();
    assert_eq!(Number::Int(-5 / -1), compute("-5/-1", &mut ctx).unwrap());
    assert_eq!(Number::Int(5 / -1), compute("5/-1", &mut ctx).unwrap());
    assert_eq!(Number::Int(--5), compute("--5", &mut ctx).unwrap());
}
#[test]
fn test_pow() {
    let mut ctx = BTreeMap::new();
    assert_eq!(Number::Double(-0.5), compute("-2^-1", &mut ctx).unwrap());
    assert_eq!(Number::Double(-0.04), compute("-5^-2", &mut ctx).unwrap());
    assert_eq!(Number::Int(-25), compute("-5^2", &mut ctx).unwrap());
    assert_eq!(Number::Double(0.04), compute("5^-2", &mut ctx).unwrap());
    assert_eq!(Number::Int(3125), compute("5^5", &mut ctx).unwrap());
    assert_eq!(Number::Int(1), compute("5^0", &mut ctx).unwrap());
}

#[test]
fn test_consts() {
    let mut ctx = BTreeMap::new();
    assert_eq!(
        Number::Double(std::f64::consts::PI),
        compute("π", &mut ctx).unwrap()
    );
    assert_eq!(
        Number::Double(std::f64::consts::E),
        compute("e", &mut ctx).unwrap()
    );
}
#[test]
fn test_fn_sqrt() {
    let mut ctx = BTreeMap::new();
    assert_eq!(
        Number::Double(2.23606797749979),
        compute("sqrt(5)", &mut ctx).unwrap()
    );
    assert_eq!(Number::Double(5.), compute("sqrt(5*5)", &mut ctx).unwrap());
    assert_eq!(
        Number::Double(8983.719357816115),
        compute("sqrt(2*(3/4.-12%5 +7^9) --6/12.*4)", &mut ctx).unwrap()
    );
}

#[test]
fn test_fn_abs() {
    let mut ctx = BTreeMap::new();
    assert_eq!(Number::Int(5), compute("abs(5)", &mut ctx).unwrap());
    assert_eq!(Number::Int(25), compute("abs(-5*5)", &mut ctx).unwrap());
    assert_eq!(
        Number::Double(80707209.5),
        compute("abs(-2*(3/4.-12%5 +7^9) --6/12.*4)", &mut ctx).unwrap()
    );
}

#[test]
fn test_extra() {
    let mut ctx = BTreeMap::new();
    assert_eq!(
        Number::Double(44721.45950030539),
        compute("sqrt((2*10^9-5*abs(8/9.))) + abs(1/10.)", &mut ctx).unwrap()
    );
    assert_eq!(
        Number::Double(161414423.89420456),
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
}
