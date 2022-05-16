use std::fmt::{Debug, Display};

use self::scalar::Scalar;

pub mod scalar;

#[derive(Debug, Clone)]
pub enum Array {
    Scalar(Scalar),
    Vector(Vec<Array>),
}

impl Display for Array {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Array::Scalar(s) => write!(f, "{}", s),
            Array::Vector(v) => {
                write!(f, "[ ")?;
                for a in v {
                    write!(f, "{} ", a)?;
                }
                write!(f, "]")
            }
        }
    }
}

impl Array {
    pub fn rank(&self) -> usize {
        match self {
            Array::Scalar(_) => 0,
            Array::Vector(_) => 1,
        }
    }

    pub fn shape(&self) -> Vec<usize> {
        match self {
            Array::Scalar(_) => vec![],
            Array::Vector(v) => vec![v.len()],
        }
    }

    pub fn pick(&self, index: &[usize]) -> &Array {
        if index.len() != self.rank() {
            panic!("RANK ERROR");
        }

        match self {
            Array::Scalar(_) => self,
            Array::Vector(v) => v.get(index[0]).unwrap(),
        }
    }

    pub fn pervade<F>(&self, scalar_fn: F) -> Array
    where
        F: Clone + Fn(Scalar) -> Scalar,
    {
        match self {
            Array::Scalar(s) => Array::Scalar(scalar_fn(s.clone())),
            Array::Vector(v) => {
                Array::Vector(v.iter().map(|a| a.pervade(scalar_fn.clone())).collect())
            }
        }
    }
}

impl FromIterator<Array> for Array {
    fn from_iter<T: IntoIterator<Item = Array>>(iter: T) -> Self {
        Array::Vector(iter.into_iter().collect())
    }
}
