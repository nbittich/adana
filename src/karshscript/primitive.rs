use std::{
    cmp::Ordering,
    fmt::Display,
    iter::Sum,
    ops::{Add, Div, Mul, Rem, Sub},
};

use anyhow::Result;

use crate::prelude::{Deserialize, Serialize};

const MAX_U32_AS_I128: i128 = u32::MAX as i128;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Primitive {
    Int(i128),
    Bool(bool),
    Double(f64),
    String(String),
    Error(&'static str),
    Unit,
}

// region: traits

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
                "call to is_greater_than() for two different types",
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
                "call to is_greater_or_equal() for two different types",
            ),
        }
    }
    pub fn is_less_than(&self, other: &Primitive) -> Primitive {
        match self.partial_cmp(other) {
            Some(Ordering::Less) => Primitive::Bool(true),
            Some(Ordering::Greater) => Primitive::Bool(false),
            Some(Ordering::Equal) => Primitive::Bool(false),
            None => Primitive::Error(
                "call to is_less_than() for two different types",
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
                "call to is_less_or_equal() for two different types",
            ),
        }
    }
    pub fn is_equal(&self, other: &Primitive) -> Primitive {
        match self.partial_cmp(other) {
            Some(Ordering::Equal) => Primitive::Bool(true),
            Some(Ordering::Less) | Some(Ordering::Greater) => {
                Primitive::Bool(false)
            }
            None => {
                Primitive::Error("call to is_equal() for two different types")
            }
        }
    }
    pub fn as_ref_ok(&self) -> Result<&Primitive> {
        match self {
            Primitive::Error(msg) => Err(anyhow::Error::msg(*msg)),

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
            Primitive::Error(e) => write!(f, "{e}"),
            Primitive::String(s) => write!(f, "{s}"),
            Primitive::Unit => Ok(()),
        }
    }
}

impl Sin for Primitive {
    fn sin(&self) -> Self {
        match self {
            Primitive::Int(i) => Primitive::Double((*i as f64).sin()),
            Primitive::Double(d) => Primitive::Double(d.sin()),
            Primitive::Bool(_b) => {
                Primitive::Error("call to sin() on a boolean value")
            }
            Primitive::String(_s) => {
                Primitive::Error("call to sin() on a string value")
            }
            Primitive::Unit => {
                Primitive::Error("call to sin() on an unit value")
            }
            Primitive::Error(e) => panic!("call to sin() on an error. {e}"),
        }
    }
}

impl Cos for Primitive {
    fn cos(&self) -> Self {
        match self {
            Primitive::Int(i) => Primitive::Double((*i as f64).cos()),
            Primitive::Double(d) => Primitive::Double(d.cos()),
            Primitive::Bool(_b) => {
                Primitive::Error("call to cos() on a boolean value")
            }
            Primitive::String(_s) => {
                Primitive::Error("call to cos() on a string value")
            }
            Primitive::Unit => {
                Primitive::Error("call to cos() on an unit value")
            }
            Primitive::Error(e) => panic!("call to cos() on an error. {e}"),
        }
    }
}

impl Tan for Primitive {
    fn tan(&self) -> Self {
        match self {
            Primitive::Int(i) => Primitive::Double((*i as f64).tan()),
            Primitive::Double(d) => Primitive::Double(d.tan()),
            Primitive::Bool(_b) => {
                Primitive::Error("call to tan() on a boolean value")
            }
            Primitive::String(_s) => {
                Primitive::Error("call to tan() on a string value")
            }
            Primitive::Unit => {
                Primitive::Error("call to tan() on an unit value")
            }
            Primitive::Error(e) => panic!("call to tan() on an error. {e}"),
        }
    }
}

impl Logarithm for Primitive {
    fn log(&self) -> Self {
        match self {
            Primitive::Int(i) => Primitive::Double((*i as f64).log10()),
            Primitive::Double(d) => Primitive::Double(d.log10()),
            Primitive::Bool(_b) => {
                Primitive::Error("call to log() on a boolean value")
            }
            Primitive::String(_s) => {
                Primitive::Error("call to log() on a string value")
            }
            Primitive::Unit => {
                Primitive::Error("call to log() on an unit value")
            }
            Primitive::Error(e) => panic!("call to log() on an error. {e}"),
        }
    }
    fn ln(&self) -> Self {
        match self {
            Primitive::Int(i) => Primitive::Double((*i as f64).ln()),
            Primitive::Double(d) => Primitive::Double(d.ln()),
            Primitive::Bool(_b) => {
                Primitive::Error("call to ln() on a boolean value")
            }
            Primitive::String(_s) => {
                Primitive::Error("call to ln() on a string value")
            }
            Primitive::Unit => {
                Primitive::Error("call to ln() on an unit value")
            }
            Primitive::Error(e) => panic!("call to ln() on an error. {e}"),
        }
    }
}

impl Sqrt for Primitive {
    fn sqrt(&self) -> Self {
        match self {
            Primitive::Int(i) => Primitive::Double((*i as f64).sqrt()),
            Primitive::Double(d) => Primitive::Double(d.sqrt()),
            Primitive::Bool(_b) => {
                Primitive::Error("call to sqrt() on a boolean value")
            }
            Primitive::String(_s) => {
                Primitive::Error("call to sqrt() on a string value")
            }
            Primitive::Unit => {
                Primitive::Error("call to sqrt() on an unit value")
            }
            Primitive::Error(e) => panic!("call to sqrt() on an error. {e}"),
        }
    }
}
impl Abs for Primitive {
    fn abs(&self) -> Self {
        match self {
            Primitive::Int(i) => Primitive::Int(i.abs()),
            Primitive::Double(d) => Primitive::Double(d.abs()),
            Primitive::Bool(_b) => {
                Primitive::Error("call to abs() on a boolean value")
            }
            Primitive::String(_s) => {
                Primitive::Error("call to abs() on a string value")
            }
            Primitive::Unit => {
                Primitive::Error("call to abs() on an unit value")
            }
            Primitive::Error(e) => panic!("call to abs() on an error. {e}"),
        }
    }
}

impl Pow for Primitive {
    fn pow(&self, rhs: Self) -> Self {
        match self {
            Primitive::Int(l) => match rhs {
                #[allow(clippy::manual_range_contains)]
                Primitive::Int(r) if r >= 0 && r <= MAX_U32_AS_I128 => {
                    Primitive::Int(l.pow(r as u32))
                }
                Primitive::Int(r) => {
                    Primitive::Double((*l as f64).powf(r as f64))
                }
                Primitive::Double(r) => Primitive::Double((*l as f64).powf(r)),
                Primitive::Bool(_b) => {
                    Primitive::Error("call to pow() on a boolean value")
                }
                Primitive::String(_s) => {
                    Primitive::Error("call to pow() on a string value")
                }
                Primitive::Unit => {
                    Primitive::Error("call to pow() on an unit value")
                }
                Primitive::Error(e) => panic!("call to pow() on an error. {e}"),
            },
            Primitive::Double(l) => match rhs {
                Primitive::Int(r) => Primitive::Double(l.powf(r as f64)),
                Primitive::Double(r) => Primitive::Double((*l as f64).powf(r)),
                Primitive::Bool(_b) => {
                    Primitive::Error("call to pow() on a boolean value")
                }
                Primitive::String(_s) => {
                    Primitive::Error("call to pow() on a string value")
                }
                Primitive::Unit => {
                    Primitive::Error("call to pow() on an unit value")
                }
                Primitive::Error(e) => panic!("call to pow() on an error. {e}"),
            },
            Primitive::String(_s) => {
                Primitive::Error("call to pow() on a string value")
            }
            Primitive::Bool(_b) => {
                Primitive::Error("call to pow() on a boolean value")
            }
            Primitive::Unit => {
                Primitive::Error("call to pow() on an unit value")
            }
            Primitive::Error(e) => panic!("call to pow() on an error. {e}"),
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
        match self {
            Primitive::Int(l) => match rhs {
                Primitive::Int(r) => Primitive::Int(l + r),
                Primitive::Double(r) => Primitive::Double(l as f64 + r),
                Primitive::Bool(_b) => {
                    Primitive::Error("call to add() on a boolean value")
                }
                Primitive::Unit => {
                    Primitive::Error("call to add() on an unit value")
                }
                Primitive::String(s) => Primitive::String(format!("{l}{s}")),
                Primitive::Error(e) => panic!("call to add() on an error. {e}"),
            },
            Primitive::Double(l) => match rhs {
                Primitive::Int(r) => Primitive::Double(l + r as f64),
                Primitive::Double(r) => Primitive::Double(l as f64 + r),
                Primitive::Bool(_b) => {
                    Primitive::Error("call to add() on a boolean value")
                }
                Primitive::Unit => {
                    Primitive::Error("call to add() on an unit value")
                }
                Primitive::String(s) => Primitive::String(format!("{l}{s}")),
                Primitive::Error(e) => panic!("call to add() on an error. {e}"),
            },
            Primitive::String(s) => Primitive::String(format!("{s}{rhs}")),
            Primitive::Bool(_b) => {
                Primitive::Error("call to add() on a boolean value")
            }
            Primitive::Unit => {
                Primitive::Error("call to add() on an unit value")
            }
            Primitive::Error(e) => panic!("call to add() on an error. {e}"),
        }
    }
}

impl Sub for Primitive {
    type Output = Primitive;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Primitive::Int(l) => match rhs {
                Primitive::Int(r) => Primitive::Int(l - r),
                Primitive::Double(r) => Primitive::Double(l as f64 - r),
                Primitive::Bool(_) | Primitive::String(_) => Primitive::Error(
                    "call to sub() on a boolean or string value",
                ),
                Primitive::Error(e) => panic!("call to sub() on an error. {e}"),
                Primitive::Unit => {
                    Primitive::Error("call to sub() on an unit value")
                }
            },
            Primitive::Double(l) => match rhs {
                Primitive::Int(r) => Primitive::Double(l - r as f64),
                Primitive::Double(r) => Primitive::Double(l as f64 - r),
                Primitive::Bool(_) | Primitive::String(_) => Primitive::Error(
                    "call to sub() on a boolean or string value",
                ),
                Primitive::Unit => {
                    Primitive::Error("call to sub() on an unit value")
                }
                Primitive::Error(e) => panic!("call to sub() on an error. {e}"),
            },
            Primitive::Bool(_) | Primitive::String(_) => {
                Primitive::Error("call to sub() on a boolean or string value")
            }
            Primitive::Unit => {
                Primitive::Error("call to sub() on an unit value")
            }
            Primitive::Error(e) => panic!("call to sub() on an error. {e}"),
        }
    }
}

impl Rem for Primitive {
    type Output = Primitive;

    fn rem(self, rhs: Self) -> Self::Output {
        match self {
            Primitive::Int(l) => match rhs {
                Primitive::Int(r) if r != 0 => Primitive::Int(l % r),
                Primitive::Double(r) => Primitive::Double(l as f64 % r),
                Primitive::Bool(_) | Primitive::String(_) => Primitive::Error(
                    "call to rem() on a boolean or string value",
                ),
                Primitive::Unit => {
                    Primitive::Error("call to rem() on an unit value")
                }
                Primitive::Error(e) => panic!("call to rem() on an error. {e}"),
                _ => Primitive::Double(f64::NAN),
            },
            Primitive::Double(l) => match rhs {
                Primitive::Int(r) => Primitive::Double(l % r as f64),
                Primitive::Double(r) => Primitive::Double(l as f64 % r),
                Primitive::Bool(_) | Primitive::String(_) => Primitive::Error(
                    "call to rem() on a boolean or string value",
                ),
                Primitive::Unit => {
                    Primitive::Error("call to rem() on an unit value")
                }
                Primitive::Error(e) => panic!("call to rem() on an error. {e}"),
            },
            Primitive::Bool(_) | Primitive::String(_) => {
                Primitive::Error("call to rem() on a boolean or string  value")
            }
            Primitive::Unit => {
                Primitive::Error("call to rem() on an unit value")
            }
            Primitive::Error(e) => panic!("call to rem() on an error. {e}"),
        }
    }
}
impl Mul for Primitive {
    type Output = Primitive;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Primitive::Int(l) => match rhs {
                Primitive::Int(r) => Primitive::Int(l.wrapping_mul(r)),
                Primitive::Double(r) => Primitive::Double(l as f64 * r),
                Primitive::Bool(_) | Primitive::String(_) => Primitive::Error(
                    "call to mul() on a boolean or string value",
                ),
                Primitive::Unit => {
                    Primitive::Error("call to mul() on an unit value")
                }
                Primitive::Error(e) => panic!("call to mul() on an error. {e}"),
            },
            Primitive::Double(l) => match rhs {
                Primitive::Int(r) => Primitive::Double(l * r as f64),
                Primitive::Double(r) => Primitive::Double(l as f64 * r),
                Primitive::Bool(_) | Primitive::String(_) => Primitive::Error(
                    "call to mul() on a boolean or string value",
                ),

                Primitive::Unit => {
                    Primitive::Error("call to mul() on an unit value")
                }
                Primitive::Error(e) => panic!("call to mul() on an error. {e}"),
            },
            Primitive::String(l) => match rhs {
                Primitive::Int(r) => Primitive::String(l.repeat(r as usize)),
                _ => Primitive::Error(
                    "call to mul() for a string on an invalid value",
                ),
            },
            Primitive::Bool(_b) => {
                Primitive::Error("call to mul() on a boolean value")
            }
            Primitive::Unit => {
                Primitive::Error("call to mul() on an unit value")
            }
            Primitive::Error(e) => panic!("call to mul() on an error. {e}"),
        }
    }
}
impl Div for Primitive {
    type Output = Primitive;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Primitive::Int(l) => match rhs {
                Primitive::Int(r) if r != 0 => Primitive::Int(l / r),
                Primitive::Double(r) => Primitive::Double(l as f64 / r),
                Primitive::Int(_) if l >= 1 => Primitive::Double(f64::INFINITY),
                Primitive::Bool(_) | Primitive::String(_) => Primitive::Error(
                    "call to div() on a boolean or string value",
                ),
                Primitive::Unit => {
                    Primitive::Error("call to div() on an unit value")
                }
                Primitive::Error(e) => panic!("call to div() on an error. {e}"),
                _ => Primitive::Double(f64::NAN),
            },
            Primitive::Double(l) => match rhs {
                Primitive::Int(r) => Primitive::Double(l / r as f64),
                Primitive::Double(r) => Primitive::Double(l as f64 / r),
                Primitive::Bool(_) | Primitive::String(_) => Primitive::Error(
                    "call to div() on a boolean or string value",
                ),

                Primitive::Unit => {
                    Primitive::Error("call to div() on an unit value")
                }
                Primitive::Error(e) => panic!("call to div() on an error. {e}"),
            },
            Primitive::Bool(_) | Primitive::String(_) => {
                Primitive::Error("call to div() on a boolean or string value")
            }
            Primitive::Unit => {
                Primitive::Error("call to div() on an unit value")
            }
            Primitive::Error(e) => panic!("call to div() on an error. {e}"),
        }
    }
}

impl std::ops::Neg for Primitive {
    type Output = Primitive;

    fn neg(self) -> Self::Output {
        match self {
            Primitive::Int(n) => Primitive::Int(-n),
            Primitive::Double(n) => Primitive::Double(-n),
            Primitive::Bool(_) | Primitive::String(_) => {
                Primitive::Error("call to neg() on a boolean or string value")
            }
            Primitive::Unit => {
                Primitive::Error("call to neg() on an unit value")
            }
            Primitive::Error(e) => panic!("call to div() on an error. {e}"),
        }
    }
}

impl std::ops::Not for Primitive {
    type Output = Primitive;

    fn not(self) -> Self::Output {
        match self {
            Primitive::Bool(b) => Primitive::Bool(!b),
            Primitive::Int(_) => {
                Primitive::Error("call to not() on an int value")
            }
            Primitive::Double(_) => {
                Primitive::Error("call to not() on a double value")
            }
            Primitive::String(_) => {
                Primitive::Error("call to not() on a string value")
            }
            Primitive::Unit => {
                Primitive::Error("call to not() on an unit value")
            }
            Primitive::Error(e) => panic!("call to div() on an error. {e}"),
        }
    }
}
impl Or for Primitive {
    fn or(&self, n: Self) -> Self {
        match self {
            Primitive::Bool(l) => match n {
                Primitive::Bool(r) => Primitive::Bool(*l || r),
                Primitive::Int(_) => Primitive::Error("'or' on an int value"),
                Primitive::Double(_) => {
                    Primitive::Error("'or'on a double value")
                }
                Primitive::String(_) => {
                    Primitive::Error("'or'on a string value")
                }
                Primitive::Unit => Primitive::Error("'or' on an unit value"),
                Primitive::Error(e) => panic!("'or' on an error. {e}"),
            },
            Primitive::Int(_) => Primitive::Error("'or' on an int value"),
            Primitive::String(_) => Primitive::Error("'or' on an string value"),
            Primitive::Double(_) => Primitive::Error("'or'on a double value"),
            Primitive::Unit => Primitive::Error("'or' on an unit value"),
            Primitive::Error(e) => panic!("'or' on an error. {e}"),
        }
    }
}
impl And for Primitive {
    fn and(&self, n: Self) -> Self {
        match self {
            Primitive::Bool(l) => match n {
                Primitive::Bool(r) => Primitive::Bool(*l && r),
                Primitive::Int(_) => Primitive::Error("'and' on an int value"),
                Primitive::Double(_) => {
                    Primitive::Error("'and'on a double value")
                }
                Primitive::String(_) => {
                    Primitive::Error("'and'on a string value")
                }
                Primitive::Unit => Primitive::Error("'and' on an unit value"),
                Primitive::Error(e) => panic!("'and' on an error. {e}"),
            },
            Primitive::Int(_) => Primitive::Error("'and' on an int value"),
            Primitive::String(_) => {
                Primitive::Error("'and' on an string value")
            }
            Primitive::Unit => Primitive::Error("'and' on an unit value"),
            Primitive::Double(_) => Primitive::Error("'and'on a double value"),
            Primitive::Error(e) => panic!("'and' on an error. {e}"),
        }
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

            (Primitive::String(l), Primitive::String(r)) => l.partial_cmp(r),

            (Primitive::String(_), Primitive::Error(_)) => None,
            (Primitive::Error(_), Primitive::String(_)) => None,
            (Primitive::Int(_), Primitive::Error(_)) => None,
            (Primitive::Bool(_), Primitive::Int(_)) => None,
            (Primitive::Bool(_), Primitive::Double(_)) => None,
            (Primitive::Bool(_), Primitive::Error(_)) => None,
            (Primitive::Double(_), Primitive::Bool(_)) => None,
            (Primitive::Double(_), Primitive::Error(_)) => None,
            (Primitive::Error(_), Primitive::Int(_)) => None,
            (Primitive::Error(_), Primitive::Bool(_)) => None,
            (Primitive::Error(_), Primitive::Double(_)) => None,
            (Primitive::Error(_), Primitive::Error(_)) => None,
            (Primitive::Int(_), Primitive::Bool(_)) => None,

            (Primitive::Int(_), Primitive::String(_)) => None,
            (Primitive::Bool(_), Primitive::String(_)) => None,
            (Primitive::Double(_), Primitive::String(_)) => None,
            (Primitive::String(_), Primitive::Int(_)) => None,
            (Primitive::String(_), Primitive::Bool(_)) => None,
            (Primitive::String(_), Primitive::Double(_)) => None,
            (Primitive::Int(_), Primitive::Unit) => None,
            (Primitive::Bool(_), Primitive::Unit) => None,
            (Primitive::Double(_), Primitive::Unit) => None,
            (Primitive::String(_), Primitive::Unit) => None,
            (Primitive::Error(_), Primitive::Unit) => None,
            (Primitive::Unit, Primitive::Int(_)) => None,
            (Primitive::Unit, Primitive::Bool(_)) => None,
            (Primitive::Unit, Primitive::Double(_)) => None,
            (Primitive::Unit, Primitive::String(_)) => None,
            (Primitive::Unit, Primitive::Error(_)) => None,
            (Primitive::Unit, Primitive::Unit) => Some(Ordering::Equal),
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
