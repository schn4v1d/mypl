use crate::primitives::PrimitiveFunction;

use super::array::{scalar::Scalar, Array};

#[derive(Debug)]
pub enum Function {
    Primitive(PrimitiveFunction),
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
            },
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
