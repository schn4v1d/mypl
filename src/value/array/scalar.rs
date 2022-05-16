use std::fmt::Display;

use super::Array;

#[derive(Debug, Clone)]
pub enum Scalar {
    Integer(i64),
    Float(f64),
    Array(Box<Array>),
}

impl Scalar {
    pub fn round_whole(&self) -> Scalar {
        match self {
            Scalar::Integer(i) => Scalar::Integer(*i),
            Scalar::Float(f) => {
                if *f % 1.0 == 0.0 {
                    Scalar::Integer(*f as i64)
                } else {
                    Scalar::Float(*f)
                }
            }
            Scalar::Array(a) => Scalar::Array(Box::new(a.pervade(|x| x.round_whole()))),
        }
    }

    pub fn conjugate(&self) -> Scalar {
        match self {
            Scalar::Integer(i) => Scalar::Integer(*i),
            Scalar::Float(f) => Scalar::Float(*f),
            Scalar::Array(a) => Scalar::Array(Box::new(a.pervade(|x| x.conjugate()))),
        }
    }

    pub fn negation(&self) -> Scalar {
        match self {
            Scalar::Integer(i) => Scalar::Integer(-*i),
            Scalar::Float(f) => Scalar::Float(-*f),
            Scalar::Array(a) => Scalar::Array(Box::new(a.pervade(|x| x.negation()))),
        }
    }

    pub fn reciprocal(&self) -> Scalar {
        match self {
            Scalar::Integer(i) => Scalar::Float(1.0 / *i as f64).round_whole(),
            Scalar::Float(f) => Scalar::Float(1.0 / *f).round_whole(),
            Scalar::Array(a) => Scalar::Array(Box::new(a.pervade(|x| x.reciprocal()))),
        }
    }

    pub fn signum(&self) -> Scalar {
        match self {
            Scalar::Integer(i) => Scalar::Integer(if *i >= 0 { 1 } else { -1 }),
            Scalar::Float(f) => Scalar::Integer(if *f >= 0.0 { 1 } else { -1 }),
            Scalar::Array(a) => Scalar::Array(Box::new(a.pervade(|x| x.signum()))),
        }
    }
}

impl Display for Scalar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Scalar::Integer(n) => {
                if *n < 0 {
                    write!(f, "¯{}", n.abs())
                } else {
                    write!(f, "{}", n)
                }
            }
            Scalar::Float(n) => {
                if *n < 0.0 {
                    write!(f, "¯{}", n.abs())
                } else {
                    write!(f, "{}", n)
                }
            }
            Scalar::Array(_) => todo!(),
        }
    }
}

impl std::ops::Add for Scalar {
    type Output = Scalar;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Scalar::Integer(a), Scalar::Integer(b)) => Scalar::Integer(a + b),
            (Scalar::Integer(a), Scalar::Float(b)) => Scalar::Float(a as f64 + b),
            (Scalar::Float(a), Scalar::Integer(b)) => Scalar::Float(a + b as f64),
            (Scalar::Float(a), Scalar::Float(b)) => Scalar::Float(a + b),
            (a, Scalar::Array(b)) => Scalar::Array(Box::new(b.pervade(|s| a.clone() + s))),
            (Scalar::Array(a), b) => Scalar::Array(Box::new(a.pervade(|s| s + b.clone()))),
        }
    }
}

impl std::ops::Sub for Scalar {
    type Output = Scalar;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Scalar::Integer(a), Scalar::Integer(b)) => Scalar::Integer(a - b),
            (Scalar::Integer(a), Scalar::Float(b)) => Scalar::Float(a as f64 - b),
            (Scalar::Float(a), Scalar::Integer(b)) => Scalar::Float(a - b as f64),
            (Scalar::Float(a), Scalar::Float(b)) => Scalar::Float(a - b),
            (a, Scalar::Array(b)) => Scalar::Array(Box::new(b.pervade(|s| a.clone() - s))),
            (Scalar::Array(a), b) => Scalar::Array(Box::new(a.pervade(|s| s - b.clone()))),
        }
    }
}

impl std::ops::Mul for Scalar {
    type Output = Scalar;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Scalar::Integer(a), Scalar::Integer(b)) => Scalar::Integer(a * b),
            (Scalar::Integer(a), Scalar::Float(b)) => Scalar::Float(a as f64 * b),
            (Scalar::Float(a), Scalar::Integer(b)) => Scalar::Float(a * b as f64),
            (Scalar::Float(a), Scalar::Float(b)) => Scalar::Float(a * b),
            (a, Scalar::Array(b)) => Scalar::Array(Box::new(b.pervade(|s| a.clone() * s))),
            (Scalar::Array(a), b) => Scalar::Array(Box::new(a.pervade(|s| s * b.clone()))),
        }
    }
}

impl std::ops::Div for Scalar {
    type Output = Scalar;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            (Scalar::Integer(a), Scalar::Integer(b)) => {
                Scalar::Float(a as f64 / b as f64).round_whole()
            }
            (Scalar::Integer(a), Scalar::Float(b)) => Scalar::Float(a as f64 / b),
            (Scalar::Float(a), Scalar::Integer(b)) => Scalar::Float(a / b as f64),
            (Scalar::Float(a), Scalar::Float(b)) => Scalar::Float(a / b),
            (a, Scalar::Array(b)) => Scalar::Array(Box::new(b.pervade(|s| a.clone() / s))),
            (Scalar::Array(a), b) => Scalar::Array(Box::new(a.pervade(|s| s / b.clone()))),
        }
    }
}
