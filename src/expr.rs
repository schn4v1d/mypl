use crate::primitives::{PrimitiveDyadicOperator, PrimitiveFunction, PrimitiveMonadicOperator};

pub enum Expr {
    Integer(i64),
    Float(f64),
    PrimitiveFunction(PrimitiveFunction),
    PrimitiveMonadicOperator(PrimitiveMonadicOperator),
    PrimitiveDyadicOperator(PrimitiveDyadicOperator),
    Isolated(Box<Expr>),
}
