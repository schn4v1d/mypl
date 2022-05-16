use crate::{
    expr::Expr,
    primitives::{PrimitiveDyadicOperator, PrimitiveFunction, PrimitiveMonadicOperator},
};

use self::binding::{binding_strengths, combine};

mod binding;

#[derive(Debug, Clone)]
pub enum EvalTree {
    Array(Vec<EvalTree>),
    Function(Box<EvalTree>),
    MonadicFunctionApplication(Box<EvalTree>, Box<EvalTree>),
    DyadicFunctionApplication(Box<EvalTree>, Box<EvalTree>, Box<EvalTree>),
    BoundLeftArgument(Box<EvalTree>, Box<EvalTree>),
    MonadicOperatorApplication(Box<EvalTree>, Box<EvalTree>),
    BoundRightOperand(Box<EvalTree>, Box<EvalTree>),
    DyadicOperatorApplication(Box<EvalTree>, Box<EvalTree>, Box<EvalTree>),
    Atop(Box<EvalTree>, Box<EvalTree>),
    Fork(Box<EvalTree>, Box<EvalTree>, Box<EvalTree>),

    Integer(i64),
    Float(f64),
    PrimitiveFunction(PrimitiveFunction),
    PrimitiveMonadicOperator(PrimitiveMonadicOperator),
    PrimitiveDyadicOperator(PrimitiveDyadicOperator),

    Unfinished(Vec<EvalTree>),
}

impl EvalTree {
    fn from(expr: Expr) -> EvalTree {
        match expr {
            Expr::Integer(n) => EvalTree::Integer(n),
            Expr::Float(n) => EvalTree::Float(n),
            Expr::PrimitiveFunction(f) => EvalTree::PrimitiveFunction(f),
            Expr::PrimitiveMonadicOperator(o) => EvalTree::PrimitiveMonadicOperator(o),
            Expr::PrimitiveDyadicOperator(o) => EvalTree::PrimitiveDyadicOperator(o),
            Expr::Isolated(es) => {
                EvalTree::Unfinished(es.into_iter().map(EvalTree::from).collect())
            }
        }
    }

    fn build(self) -> EvalTree {
        if let EvalTree::Unfinished(ts) = self {
            let mut ts: Vec<_> = ts.into_iter().map(EvalTree::build).collect();

            while ts.len() > 1 {
                let strengths = binding_strengths(&ts);

                let strongest = strengths
                    .iter()
                    .enumerate()
                    .max_by(|x, y| x.1.cmp(y.1))
                    .map(|x| x.0)
                    .unwrap();

                let x = ts.remove(strongest);
                let y = ts.remove(strongest);

                ts.insert(strongest, combine(x, y));
            }

            ts[0].clone()
        } else {
            self
        }
    }
}

pub fn build_tree(exprs: Vec<Expr>) -> EvalTree {
    let unfinished = EvalTree::Unfinished(exprs.into_iter().map(EvalTree::from).collect());

    unfinished.build()
}
