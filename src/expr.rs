use crate::primitives::{PrimitiveDyadicOperator, PrimitiveFunction, PrimitiveMonadicOperator};

#[derive(Debug)]
pub enum Expr {
    Integer(i64),
    Float(f64),
    PrimitiveFunction(PrimitiveFunction),
    PrimitiveMonadicOperator(PrimitiveMonadicOperator),
    PrimitiveDyadicOperator(PrimitiveDyadicOperator),
    Isolated(Vec<Expr>),
}
