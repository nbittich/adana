use std::{
    fmt::Display,
    iter::Sum,
    ops::{Add, Div, Mul, Rem, Sub},
};

use crate::prelude::{Deserialize, Serialize};

const MAX_U32_AS_I128: i128 = u32::MAX as i128;

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Number {
    Int(i128),
    Double(f64),
}

// region: traits

pub trait Pow {
    fn pow(&self, n: Self) -> Self;
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

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Int(i) => write!(f, "{i}"),
            Number::Double(d) => write!(f, "{d}"),
        }
    }
}

impl Sin for Number {
    fn sin(&self) -> Self {
        match self {
            Number::Int(i) => Number::Double((*i as f64).sin()),
            Number::Double(d) => Number::Double(d.sin()),
        }
    }
}

impl Cos for Number {
    fn cos(&self) -> Self {
        match self {
            Number::Int(i) => Number::Double((*i as f64).cos()),
            Number::Double(d) => Number::Double(d.cos()),
        }
    }
}

impl Tan for Number {
    fn tan(&self) -> Self {
        match self {
            Number::Int(i) => Number::Double((*i as f64).tan()),
            Number::Double(d) => Number::Double(d.tan()),
        }
    }
}

impl Logarithm for Number {
    fn log(&self) -> Self {
        match self {
            Number::Int(i) => Number::Double((*i as f64).log10()),
            Number::Double(d) => Number::Double(d.log10()),
        }
    }
    fn ln(&self) -> Self {
        match self {
            Number::Int(i) => Number::Double((*i as f64).ln()),
            Number::Double(d) => Number::Double(d.ln()),
        }
    }
}

impl Sqrt for Number {
    fn sqrt(&self) -> Self {
        match self {
            Number::Int(i) => Number::Double((*i as f64).sqrt()),
            Number::Double(d) => Number::Double(d.sqrt()),
        }
    }
}
impl Abs for Number {
    fn abs(&self) -> Self {
        match self {
            Number::Int(i) => Number::Int(i.abs()),
            Number::Double(d) => Number::Double(d.abs()),
        }
    }
}

impl Pow for Number {
    fn pow(&self, rhs: Self) -> Self {
        match self {
            Number::Int(l) => match rhs {
                #[allow(clippy::manual_range_contains)]
                Number::Int(r) if r >= 0 && r <= MAX_U32_AS_I128 => {
                    Number::Int(l.pow(r as u32))
                }
                Number::Int(r) => Number::Double((*l as f64).powf(r as f64)),
                Number::Double(r) => Number::Double((*l as f64).powf(r)),
            },
            Number::Double(l) => match rhs {
                Number::Int(r) => Number::Double(l.powf(r as f64)),
                Number::Double(r) => Number::Double((*l as f64).powf(r)),
            },
        }
    }
}

impl Sum for Number {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        let mut first = Number::Int(0);
        for next in iter {
            first = first + next;
        }
        first
    }
}

impl Add for Number {
    type Output = Number;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Number::Int(l) => match rhs {
                Number::Int(r) => Number::Int(l + r),
                Number::Double(r) => Number::Double(l as f64 + r),
            },
            Number::Double(l) => match rhs {
                Number::Int(r) => Number::Double(l + r as f64),
                Number::Double(r) => Number::Double(l as f64 + r),
            },
        }
    }
}

impl Sub for Number {
    type Output = Number;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Number::Int(l) => match rhs {
                Number::Int(r) => Number::Int(l - r),
                Number::Double(r) => Number::Double(l as f64 - r),
            },
            Number::Double(l) => match rhs {
                Number::Int(r) => Number::Double(l - r as f64),
                Number::Double(r) => Number::Double(l as f64 - r),
            },
        }
    }
}

impl Rem for Number {
    type Output = Number;

    fn rem(self, rhs: Self) -> Self::Output {
        match self {
            Number::Int(l) => match rhs {
                Number::Int(r) if r != 0 => Number::Int(l % r),
                Number::Double(r) => Number::Double(l as f64 % r),
                _ => Number::Double(f64::NAN),
            },
            Number::Double(l) => match rhs {
                Number::Int(r) => Number::Double(l % r as f64),
                Number::Double(r) => Number::Double(l as f64 % r),
            },
        }
    }
}
impl Mul for Number {
    type Output = Number;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Number::Int(l) => match rhs {
                Number::Int(r) => Number::Int(l * r),
                Number::Double(r) => Number::Double(l as f64 * r),
            },
            Number::Double(l) => match rhs {
                Number::Int(r) => Number::Double(l * r as f64),
                Number::Double(r) => Number::Double(l as f64 * r),
            },
        }
    }
}
impl Div for Number {
    type Output = Number;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Number::Int(l) => match rhs {
                Number::Int(r) if r != 0 => Number::Int(l / r),
                Number::Double(r) => Number::Double(l as f64 / r),
                Number::Int(_) if l >= 1 => Number::Double(f64::INFINITY),
                _ => Number::Double(f64::NAN),
            },
            Number::Double(l) => match rhs {
                Number::Int(r) => Number::Double(l / r as f64),
                Number::Double(r) => Number::Double(l as f64 / r),
            },
        }
    }
}

impl std::ops::Neg for Number {
    type Output = Number;

    fn neg(self) -> Self::Output {
        match self {
            Number::Int(n) => Number::Int(-n),
            Number::Double(n) => Number::Double(-n),
        }
    }
}

// endregion

#[cfg(test)]
mod test {
    use super::Number;

    #[test]
    fn test_add_valid() {
        let l = Number::Int(1);
        let r = Number::Int(2);
        assert_eq!(l + r, Number::Int(3));

        let l = Number::Int(1);
        let r = Number::Double(2.);
        assert_eq!(l + r, Number::Double(3.));

        let l = Number::Double(1.);
        let r = Number::Int(2);
        assert_eq!(l + r, Number::Double(3.));

        let l = Number::Double(1.);
        let r = Number::Double(2.);
        assert_eq!(l + r, Number::Double(3.));
    }
}
