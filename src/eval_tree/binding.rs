use std::iter::once;

use super::EvalTree;

enum BindingType {
    A,
    F,
    H,
    AF,
    MOP,
    DOP,
    JOT,
    DOT,
    REF,
    IDX,
}

impl BindingType {
    fn from(t: &EvalTree) -> BindingType {
        use BindingType::*;
        use EvalTree::*;

        match t {
            Array(_) => A,
            Function(_) => F,
            MonadicFunctionApplication(_, _) => A,
            DyadicFunctionApplication(_, _, _) => A,
            BoundLeftArgument(_, _) => AF,
            MonadicOperatorApplication(_, _) => F,
            BoundRightOperand(_, _) => MOP,
            DyadicOperatorApplication(_, _, _) => F,
            Atop(_, _) => F,
            Fork(_, _, _) => F,
            Integer(_) => A,
            Float(_) => F,
            PrimitiveFunction(_) => F,
            PrimitiveMonadicOperator(_) => MOP,
            PrimitiveDyadicOperator(_) => DOP,
            Unfinished(_) => todo!(),
        }
    }
}

fn combine_arrays(x: EvalTree, y: EvalTree) -> EvalTree {
    use EvalTree::*;

    match (x.clone(), y.clone()) {
        (Array(x), Array(y)) => Array(x.into_iter().chain(y.into_iter()).collect()),
        (Array(x), _) => Array(x.into_iter().chain(once(y)).collect()),
        (_, Array(y)) => Array(once(x).chain(y.into_iter()).collect()),
        (_, _) => Array(vec![x, y]),
    }
}

fn combine_functions(x: EvalTree, y: EvalTree) -> EvalTree {
    use EvalTree::*;

    match (x, y) {
        _ => todo!(),
    }
}

pub fn combine(x: EvalTree, y: EvalTree) -> EvalTree {
    use BindingType::*;
    use EvalTree::*;

    let bx = BindingType::from(&x);
    let by = BindingType::from(&y);

    match (bx, by) {
        (A, A) => combine_arrays(x, y),
        (A, F) | (A, H) => BoundLeftArgument(Box::new(x), Box::new(y)),
        (A, MOP) | (F, MOP) | (H, MOP) | (JOT, MOP) => {
            MonadicOperatorApplication(Box::new(x), Box::new(y))
        }
        (A, DOT) => todo!(),
        (A, IDX) => todo!(),
        (F, A) => MonadicFunctionApplication(Box::new(x), Box::new(y)),
        (F, F) => combine_functions(x, y),
        (F, H) => combine_functions(x, y),
        (F, DOT) => todo!(),
        (F, IDX) => todo!(),
        (H, F) => todo!(),
        (H, H) => todo!(),
        (H, AF) => todo!(),
        (AF, A) => {
            if let BoundLeftArgument(l, f) = x {
                DyadicFunctionApplication(l, f, Box::new(y))
            } else {
                unreachable!()
            }
        }
        (AF, F) => todo!(),
        (DOP, A) | (DOP, F) | (DOP, H) | (JOT, A) | (JOT, F) | (JOT, H) | (DOT, F) | (DOT, H) => {
            BoundRightOperand(Box::new(x), Box::new(y))
        }
        (REF, _) => todo!(),
        (IDX, _) => todo!(),
        (_, REF) => todo!(),
        (_, IDX) => todo!(),
        _ => panic!(),
    }
}

pub fn binding_strengths(ts: &[EvalTree]) -> Vec<u8> {
    ts.iter()
        .take(ts.len() - 1)
        .zip(ts.iter().skip(1))
        .map(|(x, y)| binding_strength(x, y))
        .collect()
}

fn binding_strength(x: &EvalTree, y: &EvalTree) -> u8 {
    match (BindingType::from(x), BindingType::from(y)) {
        (BindingType::A, BindingType::A) => 6,
        (BindingType::A, BindingType::F) => 3,
        (BindingType::A, BindingType::H) => 3,
        (BindingType::A, BindingType::MOP) => 4,
        (BindingType::A, BindingType::DOT) => 7,
        (BindingType::A, BindingType::IDX) => 4,
        (BindingType::F, BindingType::A) => 2,
        (BindingType::F, BindingType::F) => 1,
        (BindingType::F, BindingType::H) => 4,
        (BindingType::F, BindingType::MOP) => 4,
        (BindingType::F, BindingType::IDX) => 4,
        (BindingType::H, BindingType::F) => 1,
        (BindingType::H, BindingType::H) => 4,
        (BindingType::H, BindingType::MOP) => 4,
        (BindingType::H, BindingType::IDX) => 4,
        (BindingType::AF, BindingType::A) => 2,
        (BindingType::AF, BindingType::F) => 1,
        (BindingType::MOP, BindingType::H) => 4,
        (BindingType::DOP, BindingType::A) => 5,
        (BindingType::DOP, BindingType::F) => 5,
        (BindingType::DOP, BindingType::H) => 5,
        (BindingType::JOT, BindingType::A) => 5,
        (BindingType::JOT, BindingType::F) => 5,
        (BindingType::JOT, BindingType::H) => 5,
        (BindingType::JOT, BindingType::MOP) => 4,
        (BindingType::DOT, BindingType::A) => 6,
        (BindingType::DOT, BindingType::F) => 5,
        (BindingType::DOT, BindingType::H) => 5,
        (BindingType::DOT, BindingType::DOP) => 6,
        (BindingType::REF, BindingType::A) => 7,
        (BindingType::REF, BindingType::F) => 7,
        (BindingType::REF, BindingType::H) => 7,
        (BindingType::REF, BindingType::MOP) => 7,
        (BindingType::REF, BindingType::DOP) => 7,
        (BindingType::IDX, BindingType::A) => 3,
        (BindingType::IDX, BindingType::F) => 3,
        (BindingType::IDX, BindingType::H) => 3,
        _ => panic!(),
    }
}
