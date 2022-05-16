use std::fmt::Display;

use crate::primitives::PrimitiveFunction;

use super::array::{scalar::Scalar, Array};

#[derive(Debug)]
pub enum Function {
    Primitive(PrimitiveFunction),
    Atop(Box<Function>, Box<Function>),
    Fork(Box<Function>, Box<Function>, Box<Function>),
}

impl Function {
    pub fn apply(&self, alpha: Option<Array>, omega: Array) -> Array {
        match self {
            Function::Primitive(primitive) => match primitive {
                PrimitiveFunction::Plus => {
                    if let Some(alpha) = alpha {
                        scalar_pervasion(|a, b| a + b, alpha, omega)
                    } else {
                        omega.pervade(|x| x.conjugate())
                    }
                }
                PrimitiveFunction::Minus => {
                    if let Some(alpha) = alpha {
                        scalar_pervasion(|a, b| a - b, alpha, omega)
                    } else {
                        omega.pervade(|x| x.negation())
                    }
                }
                PrimitiveFunction::Times => {
                    if let Some(alpha) = alpha {
                        scalar_pervasion(|a, b| a * b, alpha, omega)
                    } else {
                        omega.pervade(|x| x.signum())
                    }
                }
                PrimitiveFunction::Divide => {
                    if let Some(alpha) = alpha {
                        scalar_pervasion(|a, b| a / b, alpha, omega)
                    } else {
                        omega.pervade(|x| x.reciprocal())
                    }
                }
                PrimitiveFunction::LeftTack => {
                    if let Some(alpha) = alpha {
                        alpha
                    } else {
                        omega
                    }
                }
                PrimitiveFunction::RightTack => omega,
                PrimitiveFunction::Comma => {
                    if let Some(alpha) = alpha {
                        Array::catenate(alpha, omega)
                    } else {
                        omega.ravel()
                    }
                }
            },
            Function::Atop(f, g) => f.apply(None, g.apply(alpha, omega)),
            Function::Fork(f, g, h) => g.apply(
                Some(f.apply(alpha.clone(), omega.clone())),
                h.apply(alpha, omega),
            ),
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Function::Primitive(p) => write!(f, "{}", p),
            Function::Atop(a, b) => write!(f, "{}{}", a, b),
            Function::Fork(a, b, c) => write!(f, "{}{}{}", a, b, c),
        }
    }
}

fn scalar_pervasion<F>(scalar_fn: F, alpha: Array, omega: Array) -> Array
where
    F: Fn(Scalar, Scalar) -> Scalar,
{
    match (alpha, omega) {
        (Array::Scalar(a), Array::Scalar(b)) => Array::Scalar(scalar_fn(a, b)),
        (Array::Scalar(a), omega) => omega.pervade(|s| scalar_fn(a.clone(), s)),
        (alpha, Array::Scalar(b)) => alpha.pervade(|s| scalar_fn(s, b.clone())),
        (_, _) => todo!(),
    }
}
