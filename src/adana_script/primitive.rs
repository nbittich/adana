use std::{
    cmp::Ordering,
    collections::HashMap,
    fmt::Display,
    iter::Sum,
    ops::{Add, Div, Mul, Rem, Sub},
};

use anyhow::Result;

use crate::prelude::{Deserialize, Serialize};

use super::{constants::NULL, Value};

const MAX_U32_AS_I128: i128 = u32::MAX as i128;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Primitive {
    Int(i128),
    Bool(bool),
    Null,
    Double(f64),
    String(String),
    Array(Vec<Primitive>),
    Struct(HashMap<String, Primitive>),
    Error(String),
    Function { parameters: Vec<String>, exprs: Vec<Value> },
    Unit,
    NoReturn,
    EarlyReturn(Box<Primitive>),
}

// region: traits
pub trait TypeOf {
    fn type_of(&self) -> Self;
}
pub trait ToBool {
    fn to_bool(&self) -> Self;
}
pub trait ToNumber {
    fn to_int(&self) -> Self;
    fn to_double(&self) -> Self;
}
pub trait Pow {
    fn pow(&self, n: Self) -> Self;
}

pub trait And {
    fn and(&self, n: Self) -> Self;
}
pub trait Or {
    fn or(&self, n: Self) -> Self;
}
pub trait Sqrt {
    fn sqrt(&self) -> Self;
}
pub trait Abs {
    fn abs(&self) -> Self;
}
pub trait Logarithm {
    fn log(&self) -> Self;
    fn ln(&self) -> Self;
}

pub trait Sin {
    fn sin(&self) -> Self;
}
pub trait Array {
    fn index_at(&self, rhs: &Self) -> Self;
    fn len(&self) -> Primitive;
    fn swap_mem(&mut self, rhs: &mut Self, index: &Primitive) -> Primitive;
}

pub trait Cos {
    fn cos(&self) -> Self;
}
pub trait Tan {
    fn tan(&self) -> Self;
}

// endregion traits

// region: impl primitive
#[allow(dead_code)]
impl Primitive {
    pub fn is_greater_than(&self, other: &Primitive) -> Primitive {
        match self.partial_cmp(other) {
            Some(Ordering::Greater) => Primitive::Bool(true),
            Some(Ordering::Less) => Primitive::Bool(false),
            Some(Ordering::Equal) => Primitive::Bool(false),
            None => Primitive::Error(
                format!("call to is_greater_than() for two different types {self} => {other}")
            ),
        }
    }
    pub fn is_greater_or_equal(&self, other: &Primitive) -> Primitive {
        match self.partial_cmp(other) {
            Some(Ordering::Greater) | Some(Ordering::Equal) => {
                Primitive::Bool(true)
            }
            Some(Ordering::Less) => Primitive::Bool(false),
            None => Primitive::Error(
                format!("call to is_greater_or_equal() for two different types {self} => {other}")
            ),
        }
    }
    pub fn is_less_than(&self, other: &Primitive) -> Primitive {
        match self.partial_cmp(other) {
            Some(Ordering::Less) => Primitive::Bool(true),
            Some(Ordering::Greater) => Primitive::Bool(false),
            Some(Ordering::Equal) => Primitive::Bool(false),
            None => Primitive::Error(
                format!("call to is_less_than() for two different types {self} => {other}")
            ),
        }
    }
    pub fn is_less_or_equal(&self, other: &Primitive) -> Primitive {
        match self.partial_cmp(other) {
            Some(Ordering::Less) | Some(Ordering::Equal) => {
                Primitive::Bool(true)
            }
            Some(Ordering::Greater) => Primitive::Bool(false),
            None => Primitive::Error(
                    format!("call to is_less_or_equal() for two different types {self} => {other}")
            ),
        }
    }
    pub fn is_equal(&self, other: &Primitive) -> Primitive {
        match self.partial_cmp(other) {
            Some(Ordering::Equal) => Primitive::Bool(true),
            Some(Ordering::Less) | Some(Ordering::Greater) => {
                Primitive::Bool(false)
            }
            None => match (self, other) {
                (Primitive::Null, _) | (_, Primitive::Null) => {
                    Primitive::Bool(false)
                }
                (Primitive::Struct(_), Primitive::Struct(_)) => {
                    Primitive::Bool(false)
                }
                _ => Primitive::Error(
                    format!("call to is_equal() for two different types {self} => {other}"),
                ),
            },
        }
    }
    pub fn as_ref_ok(&self) -> Result<&Primitive> {
        match self {
            Primitive::Error(msg) => Err(anyhow::Error::msg(msg.to_string())),

            _ => Ok(self),
        }
    }
}

impl Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Primitive::Int(i) => write!(f, "{i}"),
            Primitive::Double(d) => write!(f, "{d}"),
            Primitive::Bool(b) => write!(f, "{b}"),
            Primitive::Error(e) => write!(f, "Err: {e}"),
            Primitive::String(s) => write!(f, "{s}"),
            Primitive::Unit => write!(f, "()"),
            Primitive::Array(arr) => {
                let joined_arr = arr
                    .iter()
                    .map(|p| match p {
                        Primitive::String(s) => format!(r#""{s}""#),
                        _ => p.to_string(),
                    })
                    .collect::<Vec<_>>();
                write!(f, "[{}]", joined_arr[..].join(", "))
            }
            Primitive::Struct(struc) => {
                let joined_arr = struc
                    .iter()
                    .map(|(k, p)| {
                        format!(
                            "\t{k}: {}",
                            if let Primitive::String(s) = p {
                                format!(r#""{s}""#)
                            } else {
                                p.to_string()
                            }
                        )
                    })
                    .collect::<Vec<_>>();
                write!(f, "struct {{\n{}\n}}", joined_arr[..].join(", \n"))
            }
            Primitive::Function { parameters, exprs: _ } => {
                write!(f, "({}) => {{...}}", parameters.join(", "))
            }
            Primitive::NoReturn => write!(f, "!"),
            Primitive::Null => write!(f, "{NULL}"),
            Primitive::EarlyReturn(p) => write!(f, "{p}"),
        }
    }
}

impl Sin for Primitive {
    fn sin(&self) -> Self {
        match self {
            Primitive::Int(i) => Primitive::Double((*i as f64).sin()),
            Primitive::Double(d) => Primitive::Double(d.sin()),

            Primitive::Error(e) => panic!("call to sin() on an error. {e}"),
            _ => Primitive::Error(format!("illegal call to sin() => {self}")),
        }
    }
}

impl Cos for Primitive {
    fn cos(&self) -> Self {
        match self {
            Primitive::Int(i) => Primitive::Double((*i as f64).cos()),
            Primitive::Double(d) => Primitive::Double(d.cos()),
            Primitive::Error(e) => panic!("call to cos() on an error. {e}"),
            _ => Primitive::Error(format!("illegal call to cos() => {self}")),
        }
    }
}

impl Tan for Primitive {
    fn tan(&self) -> Self {
        match self {
            Primitive::Int(i) => Primitive::Double((*i as f64).tan()),
            Primitive::Double(d) => Primitive::Double(d.tan()),
            Primitive::Error(e) => panic!("call to tan() on an error. {e}"),
            _ => Primitive::Error(format!("illegal call to tan() => {self}")),
        }
    }
}

impl Logarithm for Primitive {
    fn log(&self) -> Self {
        match self {
            Primitive::Int(i) => Primitive::Double((*i as f64).log10()),
            Primitive::Double(d) => Primitive::Double(d.log10()),
            Primitive::Error(e) => panic!("call to log() on an error. {e}"),
            _ => Primitive::Error(format!("illegal call to log() => {self}")),
        }
    }
    fn ln(&self) -> Self {
        match self {
            Primitive::Int(i) => Primitive::Double((*i as f64).ln()),
            Primitive::Double(d) => Primitive::Double(d.ln()),
            Primitive::Error(e) => panic!("call to ln() on an error. {e}"),
            _ => Primitive::Error(format!("illegal call to ln() => {self}")),
        }
    }
}

impl Sqrt for Primitive {
    fn sqrt(&self) -> Self {
        match self {
            Primitive::Int(i) => Primitive::Double((*i as f64).sqrt()),
            Primitive::Double(d) => Primitive::Double(d.sqrt()),
            Primitive::Error(e) => panic!("call to sqrt() on an error. {e}"),
            _ => Primitive::Error(format!("illegal call to sqrt() => {self}")),
        }
    }
}
impl Abs for Primitive {
    fn abs(&self) -> Self {
        match self {
            Primitive::Int(i) => Primitive::Int(i.abs()),
            Primitive::Double(d) => Primitive::Double(d.abs()),
            Primitive::Error(e) => panic!("call to abs() on an error. {e}"),
            _ => Primitive::Error(format!("illegal call to abs() => {self}")),
        }
    }
}

impl Pow for Primitive {
    fn pow(&self, rhs: Self) -> Self {
        match (self, rhs) {
            #[allow(clippy::manual_range_contains)]
            (Primitive::Int(l), Primitive::Int(r))
                if r >= 0 && r <= MAX_U32_AS_I128 =>
            {
                Primitive::Int(l.pow(r as u32))
            }
            (Primitive::Int(l), Primitive::Int(r)) => {
                Primitive::Double((*l as f64).powf(r as f64))
            }
            (Primitive::Int(l), Primitive::Double(r)) => {
                Primitive::Double((*l as f64).powf(r))
            }
            (Primitive::Double(l), Primitive::Int(r)) => {
                Primitive::Double(l.powf(r as f64))
            }
            (Primitive::Double(l), Primitive::Double(r)) => {
                Primitive::Double(l.powf(r))
            }
            (l, r) => Primitive::Error(format!(
                "illegal call to pow() => left: {l} right: {r}"
            )),
        }
    }
}

impl Sum for Primitive {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut first = Primitive::Int(0);
        for next in iter {
            first = first + next;
        }
        first
    }
}

impl Add for Primitive {
    type Output = Primitive;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Primitive::Int(l), Primitive::Int(r)) => Primitive::Int(l + r),
            (Primitive::Int(l), Primitive::Double(r)) => {
                Primitive::Double(l as f64 + r)
            }
            (Primitive::Int(l), Primitive::String(s)) => {
                Primitive::String(format!("{l}{s}"))
            }

            (Primitive::Double(l), Primitive::Int(r)) => {
                Primitive::Double(l + r as f64)
            }
            (Primitive::Double(l), Primitive::Double(r)) => {
                Primitive::Double(l + r)
            }
            (l, Primitive::String(s)) => Primitive::String(format!("{l}{s}")),

            (Primitive::String(s), r) => Primitive::String(format!("{s}{r}")),
            (Primitive::Array(mut l), Primitive::Array(mut r)) => {
                l.append(&mut r);
                Primitive::Array(l)
            }
            (l, r) => Primitive::Error(format!(
                "illegal call to add() => left: {l} right: {r}"
            )),
        }
    }
}

impl Sub for Primitive {
    type Output = Primitive;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Primitive::Int(l), Primitive::Int(r)) => Primitive::Int(l - r),
            (Primitive::Int(l), Primitive::Double(r)) => {
                Primitive::Double(l as f64 - r)
            }
            (Primitive::Double(l), Primitive::Int(r)) => {
                Primitive::Double(l - r as f64)
            }
            (Primitive::Double(l), Primitive::Double(r)) => {
                Primitive::Double(l - r)
            }
            (l, r) => Primitive::Error(format!(
                "illegal call to sub() => left: {l} right: {r}"
            )),
        }
    }
}

impl Rem for Primitive {
    type Output = Primitive;

    fn rem(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Primitive::Int(l), Primitive::Int(r)) if r != 0 => {
                Primitive::Int(l % r)
            }
            (Primitive::Int(l), Primitive::Double(r)) => {
                Primitive::Double(l as f64 % r)
            }
            (Primitive::Int(_), _) => Primitive::Double(f64::NAN),

            (Primitive::Double(l), Primitive::Int(r)) => {
                Primitive::Double(l % r as f64)
            }
            (Primitive::Double(l), Primitive::Double(r)) => {
                Primitive::Double(l % r)
            }

            (l, r) => Primitive::Error(format!(
                "illegal call to rem() => left: {l} right: {r}"
            )),
        }
    }
}
impl Mul for Primitive {
    type Output = Primitive;

    fn mul(self, rhs: Self) -> Self::Output {
        fn multiply_array(arr: Vec<Primitive>, n: i128) -> Vec<Primitive> {
            let arr_size = arr.len();
            arr.into_iter().cycle().take(n as usize * arr_size).collect()
        }
        match (self, rhs) {
            (Primitive::Int(l), Primitive::Int(r)) => {
                Primitive::Int(l.wrapping_mul(r))
            }
            (Primitive::Int(l), Primitive::Double(r)) => {
                Primitive::Double(l as f64 * r)
            }
            (Primitive::Int(l), Primitive::Array(r)) => {
                Primitive::Array(multiply_array(r, l))
            }
            (Primitive::Double(l), Primitive::Int(r)) => {
                Primitive::Double(l * r as f64)
            }
            (Primitive::Double(l), Primitive::Double(r)) => {
                Primitive::Double(l * r)
            }
            (Primitive::String(l), Primitive::Int(r)) => {
                Primitive::String(l.repeat(r as usize))
            }
            (Primitive::Array(l), Primitive::Int(n)) => {
                Primitive::Array(multiply_array(l, n))
            }

            (l, r) => Primitive::Error(format!(
                "illegal call to mul() => left: {l} right: {r}"
            )),
        }
    }
}
impl Div for Primitive {
    type Output = Primitive;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Primitive::Int(l), Primitive::Int(r)) if r != 0 => {
                Primitive::Int(l / r)
            }
            (Primitive::Int(l), Primitive::Double(r)) => {
                Primitive::Double(l as f64 / r)
            }
            (Primitive::Int(l), Primitive::Int(_)) if l >= 1 => {
                Primitive::Double(f64::INFINITY)
            }
            (Primitive::Int(_), _) => Primitive::Double(f64::NAN),
            (Primitive::Double(l), Primitive::Int(r)) => {
                Primitive::Double(l / r as f64)
            }
            (Primitive::Double(l), Primitive::Double(r)) => {
                Primitive::Double(l / r)
            }
            (l, r) => Primitive::Error(format!(
                "illegal call to div() => left: {l} right: {r}"
            )),
        }
    }
}

impl std::ops::Neg for Primitive {
    type Output = Primitive;

    fn neg(self) -> Self::Output {
        match self {
            Primitive::Int(n) => Primitive::Int(-n),
            Primitive::Double(n) => Primitive::Double(-n),
            _ => Primitive::Error(format!("invalid call to neg() {self}")),
        }
    }
}

impl std::ops::Not for Primitive {
    type Output = Primitive;

    fn not(self) -> Self::Output {
        match self {
            Primitive::Bool(b) => Primitive::Bool(!b),
            _ => Primitive::Error(format!("invalid call to not() {self}")),
        }
    }
}

impl ToBool for Primitive {
    fn to_bool(&self) -> Self {
        match self {
            v @ Primitive::Bool(_) => v.clone(),
            Primitive::Double(n) => Primitive::Bool(n > &0.0),
            Primitive::Int(n) => Primitive::Bool(n > &0),
            Primitive::Null => Primitive::Bool(false),
            Primitive::Array(a) => Primitive::Bool(!a.is_empty()),
            Primitive::String(s) => match s.parse::<bool>() {
                Ok(b) => Primitive::Bool(b),
                Err(e) => Primitive::Error(format!(
                    "invalid cast to bool: {self}, {e}"
                )),
            },
            _ => Primitive::Error(format!("invalide cast too bool: {self}")),
        }
    }
}

impl ToNumber for Primitive {
    fn to_int(&self) -> Self {
        match self {
            v @ Primitive::Int(_) => v.clone(),
            Primitive::Bool(false) => Primitive::Int(0),
            Primitive::Bool(true) => Primitive::Int(1),
            Primitive::Double(d) => Primitive::Int(*d as i128),
            Primitive::String(s) => match s.parse::<i128>() {
                Ok(number) => Primitive::Int(number),
                Err(e) => Primitive::Error(format!(
                    "invalid cast to int: {self}, {e}"
                )),
            },
            _ => Primitive::Error(format!("invalid cast to int: {self}")),
        }
    }

    fn to_double(&self) -> Self {
        match self {
            Primitive::Int(d) => Primitive::Double(*d as f64),
            v @ Primitive::Double(_) => v.clone(),
            Primitive::String(s) => match s.parse::<f64>() {
                Ok(number) => Primitive::Double(number),
                Err(e) => Primitive::Error(format!(
                    "invalid cast to double: {self}, {e}"
                )),
            },
            _ => Primitive::Error(format!("invalid cast to double: {self}")),
        }
    }
}
impl Or for Primitive {
    fn or(&self, rhs: Self) -> Self {
        if let &Primitive::Bool(true) = &self {
            return Primitive::Bool(true);
        }
        if !matches!((self, &rhs), (Primitive::Bool(_), Primitive::Bool(_))) {
            return Primitive::Error(format!(
                "illegal call to 'or' => left: {self} right: {rhs}"
            ));
        }
        rhs
    }
}
impl And for Primitive {
    fn and(&self, rhs: Self) -> Self {
        if let &Primitive::Bool(false) = &self {
            return Primitive::Bool(false);
        }

        if !matches!((self, &rhs), (Primitive::Bool(_), Primitive::Bool(_))) {
            return Primitive::Error(format!(
                "illegal call to 'and' => left: {self} right: {rhs}"
            ));
        }

        rhs
    }
}

impl PartialOrd for Primitive {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Primitive::Int(l), Primitive::Int(r)) => l.partial_cmp(r),
            (Primitive::Int(l), Primitive::Double(r)) => {
                (*l as f64).partial_cmp(r)
            }
            (Primitive::Double(l), Primitive::Int(r)) => {
                l.partial_cmp(&(*r as f64))
            }
            (Primitive::Double(l), Primitive::Double(r)) => l.partial_cmp(r),
            (Primitive::Bool(a), Primitive::Bool(b)) => a.partial_cmp(b),
            (l @ Primitive::Bool(_), r) => l.partial_cmp(&(r.to_bool())),

            (Primitive::String(l), Primitive::String(r)) => l.partial_cmp(r),
            (Primitive::Unit, Primitive::Unit) => Some(Ordering::Equal),
            (Primitive::Array(l), Primitive::Array(r)) => l.partial_cmp(r),
            (
                l @ Primitive::Function { parameters: _, exprs: _ },
                r @ Primitive::Function { parameters: _, exprs: _ },
            ) => l.partial_cmp(r),
            (Primitive::Null, Primitive::Null) => Some(Ordering::Equal),
            (Primitive::EarlyReturn(l), Primitive::EarlyReturn(r)) => {
                l.partial_cmp(r)
            }
            (Primitive::EarlyReturn(l), a) => l.as_ref().partial_cmp(a),
            (l, Primitive::EarlyReturn(r)) => l.partial_cmp(r),
            (Primitive::Struct(l), Primitive::Struct(r)) => {
                if l.eq(r) {
                    Some(Ordering::Equal)
                } else {
                    None
                }
            }
            (Primitive::Struct(_), _) => None,
            (Primitive::Int(_), _) => None,
            (Primitive::Double(_), _) => None,
            (Primitive::String(_), _) => None,
            (Primitive::NoReturn, _) => None,
            (Primitive::Null, _) => None,
            (Primitive::Array(_), _) => None,
            (Primitive::Error(_), _) => None,
            (Primitive::Unit, _) => None,
            (Primitive::Function { parameters: _, exprs: _ }, _) => None,
        }
    }
}

impl TypeOf for Primitive {
    fn type_of(&self) -> Self {
        match self {
            Primitive::Int(_) => Primitive::String("int".to_string()),
            Primitive::Bool(_) => Primitive::String("bool".to_string()),
            Primitive::Null => Primitive::String("null".to_string()),
            Primitive::Double(_) => Primitive::String("double".to_string()),
            Primitive::String(_) => Primitive::String("string".to_string()),
            Primitive::Array(_) => Primitive::String("array".to_string()),
            Primitive::Error(_) => Primitive::String("error".to_string()),
            Primitive::Function { parameters: _, exprs: _ } => {
                Primitive::String("function".to_string())
            }
            Primitive::Struct(_) => Primitive::String("struct".to_string()),
            Primitive::Unit => Primitive::String("unit".to_string()),
            Primitive::NoReturn => Primitive::String("!".to_string()),
            Primitive::EarlyReturn(v) => v.type_of(),
        }
    }
}

impl Array for Primitive {
    fn index_at(&self, rhs: &Primitive) -> Primitive {
        match (self, rhs) {
            (Primitive::Array(arr), Primitive::Int(idx)) => {
                let idx = *idx as usize;
                if idx < arr.len() {
                    arr[idx].clone()
                } else {
                    Primitive::Error("index out of range".to_string())
                }
            }
            (Primitive::String(s), Primitive::Int(idx)) => {
                let idx = *idx as usize;
                if idx < s.len() {
                    let s: String = s.chars().skip(idx).take(1).collect();
                    Primitive::String(s)
                } else {
                    Primitive::Error("index out of range".to_string())
                }
            }
            (Primitive::Struct(struc), Primitive::String(key)) => {
                if let Some(p) = struc.get(key) {
                    p.clone()
                } else {
                    Primitive::Null
                }
            }
            _ => Primitive::Error("illegal access to array!!!".to_string()),
        }
    }

    fn len(&self) -> Primitive {
        match self {
            Primitive::String(s) => Primitive::Int(s.len() as i128),
            Primitive::Array(a) => Primitive::Int(a.len() as i128),
            Primitive::Struct(s) => Primitive::Int(s.len() as i128),
            _ => Primitive::Error(format!(
                "call to len() on a non array value => {self}"
            )),
        }
    }

    fn swap_mem(
        &mut self,
        rhs: &mut Primitive,
        index: &Primitive,
    ) -> Primitive {
        match (self, index) {
            (Primitive::Array(arr), Primitive::Int(idx)) => {
                let idx = *idx as usize;
                if !matches!(rhs, Primitive::Error(_) | Primitive::Unit)
                    && idx < arr.len()
                {
                    std::mem::swap(&mut arr[idx], rhs);
                    arr[idx].clone()
                } else {
                    Primitive::Error("index out of range".to_string())
                }
            }
            (Primitive::Struct(s), Primitive::String(k)) => {
                if s.contains_key(k) {
                    *s.get_mut(k).unwrap() = rhs.clone();
                } else {
                    s.insert(k.clone(), rhs.clone());
                }
                s[k].clone()
            }
            (Primitive::String(s), Primitive::Int(idx)) => {
                let idx = *idx as usize;
                if !matches!(rhs, Primitive::Error(_) | Primitive::Unit)
                    && idx < s.len()
                {
                    s.remove(idx);
                    s.insert_str(idx, &rhs.to_string());
                    rhs.clone()
                } else {
                    Primitive::Error("index out of range".to_string())
                }
            }
            _ => Primitive::Error("invalid call to swap_mem()".to_string()),
        }
    }
}

// endregion

#[cfg(test)]
mod test {
    use super::Primitive;

    #[test]
    fn test_add_valid() {
        let l = Primitive::Int(1);
        let r = Primitive::Int(2);
        assert_eq!(l + r, Primitive::Int(3));

        let l = Primitive::Int(1);
        let r = Primitive::Double(2.);
        assert_eq!(l + r, Primitive::Double(3.));

        let l = Primitive::Double(1.);
        let r = Primitive::Int(2);
        assert_eq!(l + r, Primitive::Double(3.));

        let l = Primitive::Double(1.);
        let r = Primitive::Double(2.);
        assert_eq!(l + r, Primitive::Double(3.));
    }
}
