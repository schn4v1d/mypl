#[derive(Debug, Clone, Copy)]
pub enum PrimitiveFunction {
    Plus,
    Minus,
    Times,
    Divide,
}

#[derive(Debug, Clone, Copy)]
pub enum PrimitiveMonadicOperator {
    Commute,
}

#[derive(Debug, Clone, Copy)]
pub enum PrimitiveDyadicOperator {
    Atop,
}
