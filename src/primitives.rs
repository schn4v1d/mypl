use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub enum PrimitiveFunction {
    Plus,
    Minus,
    Times,
    Divide,
    LeftTack,
    RightTack,
    Comma,
    Epsilon,
}

impl Display for PrimitiveFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrimitiveFunction::Plus => write!(f, "+"),
            PrimitiveFunction::Minus => write!(f, "-"),
            PrimitiveFunction::Times => write!(f, "×"),
            PrimitiveFunction::Divide => write!(f, "÷"),
            PrimitiveFunction::LeftTack => write!(f, "⊣"),
            PrimitiveFunction::RightTack => write!(f, "⊢"),
            PrimitiveFunction::Comma => write!(f, ","),
            PrimitiveFunction::Epsilon => write!(f, "∊"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum PrimitiveMonadicOperator {
    Commute,
}

#[derive(Debug, Clone, Copy)]
pub enum PrimitiveDyadicOperator {
    Atop,
}
