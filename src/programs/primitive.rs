use std::{
    fmt::Display,
    ops::{Add, Div, Mul, Sub},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Copy, Serialize, Deserialize)]
pub enum Primitive {
    Int(i64),
    Double(f64),
}

// region: impl primitive

impl Display for Primitive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Primitive::Int(i) => write!(f, "{i}"),
            Primitive::Double(d) => write!(f, "{d}"),
        }
    }
}

pub trait Pow {
    fn pow(&self, n: Self) -> Self;
}

impl Pow for Primitive {
    fn pow(&self, n: Self) -> Self {
        match self {
            Primitive::Int(l) if let Primitive::Int(r) = n => Primitive::Int(l.pow(r as u32)),
            Primitive::Int(l) if let Primitive::Double(r) = n => Primitive::Double((*l as f64).powf(r)),
            Primitive::Double(l) if let Primitive::Int(r) = n => Primitive::Double(l.powi(r as i32)),
            Primitive::Double(l) if let Primitive::Double(r) = n => Primitive::Double((*l as f64).powf(r)),
            Primitive::Int(_) | Primitive::Double(_) => panic!("Pow not allowed {self}")
        }
    }
}

impl Add for Primitive {
    type Output = Primitive;

    fn add(self, rhs: Self) -> Self::Output {
        match self {
            Primitive::Int(l) if let Primitive::Int(r) = rhs  => Primitive::Int(l+r),
            Primitive::Int(l) if let Primitive::Double(r) = rhs  => Primitive::Double(l as f64 +r),

            Primitive::Double(l) if let Primitive::Int(r) = rhs  => Primitive::Double(l+r as f64),
            Primitive::Double(l) if let Primitive::Double(r) = rhs  => Primitive::Double(l as f64 +r),

            Primitive::Int(_) | Primitive::Double(_) => panic!("Addition not allowed {self}")
        }
    }
}

impl Sub for Primitive {
    type Output = Primitive;

    fn sub(self, rhs: Self) -> Self::Output {
        match self {
            Primitive::Int(l) if let Primitive::Int(r) = rhs  => Primitive::Int(l-r),
            Primitive::Int(l) if let Primitive::Double(r) = rhs  => Primitive::Double(l as f64 - r),

            Primitive::Double(l) if let Primitive::Int(r) = rhs  => Primitive::Double(l-r as f64),
            Primitive::Double(l) if let Primitive::Double(r) = rhs  => Primitive::Double(l as f64 - r),

            Primitive::Int(_) | Primitive::Double(_) => panic!("Subtract not allowed {self}")
        }
    }
}

impl std::ops::Rem for Primitive {
    type Output = Primitive;

    fn rem(self, rhs: Self) -> Self::Output {
        match self {
            Primitive::Int(l) if let Primitive::Int(r) = rhs  => Primitive::Int(l%r),
            Primitive::Int(l) if let Primitive::Double(r) = rhs  => Primitive::Double(l as f64 % r),

            Primitive::Double(l) if let Primitive::Int(r) = rhs  => Primitive::Double(l%r as f64),
            Primitive::Double(l) if let Primitive::Double(r) = rhs  => Primitive::Double(l as f64 % r),

            Primitive::Int(_) | Primitive::Double(_) => panic!("Modulo not allowed for {self}") 
        }
    }
}
impl Mul for Primitive {
    type Output = Primitive;

    fn mul(self, rhs: Self) -> Self::Output {
        match self {
            Primitive::Int(l) if let  Primitive::Int(r) = rhs  => Primitive::Int(l*r),
            Primitive::Int(l) if let  Primitive::Double(r) = rhs  => Primitive::Double(l as f64 * r),

            Primitive::Double(l) if let  Primitive::Int(r) = rhs  => Primitive::Double(l*r as f64),
            Primitive::Double(l) if let  Primitive::Double(r) = rhs  => Primitive::Double(l as f64 * r),

            Primitive::Int(_) | Primitive::Double(_) => panic!("Multiply not allowe {self}")
        }
    }
}
impl Div for Primitive {
    type Output = Primitive;

    fn div(self, rhs: Self) -> Self::Output {
        match self {
            Primitive::Int(l) if let Primitive::Int(r) = rhs  => Primitive::Int(l/r),
            Primitive::Int(l) if let Primitive::Double(r) = rhs  => Primitive::Double(l as f64 / r),

            Primitive::Double(l) if let Primitive::Int(r) = rhs  => Primitive::Double(l/r as f64),
            Primitive::Double(l) if let Primitive::Double(r) = rhs  => Primitive::Double(l as f64 / r),

            Primitive::Int(_) | Primitive::Double(_) => panic!("Division not allowed for {self}")
        }
    }
}

impl std::ops::Neg for Primitive {
    type Output = Primitive;

    fn neg(self) -> Self::Output {
        match self {
            Primitive::Int(n) => Primitive::Int(-n),
            Primitive::Double(n) => Primitive::Double(-n),
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
