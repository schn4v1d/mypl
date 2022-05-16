use std::iter::once;

use super::EvalTree;

enum BindingType {
    A,
    F,
    H,
    Af,
    Mop,
    Dop,
    Jot,
    Dot,
    Ref,
    Idx,
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
            BoundLeftArgument(_, _) => Af,
            MonadicOperatorApplication(_, _) => F,
            BoundRightOperand(_, _) => Mop,
            DyadicOperatorApplication(_, _, _) => F,
            Atop(_, _) => F,
            Fork(_, _, _) => F,
            Integer(_) => A,
            Float(_) => F,
            PrimitiveFunction(_) => F,
            PrimitiveMonadicOperator(_) => Mop,
            PrimitiveDyadicOperator(_) => Dop,
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

    // match (x, y) {
    //     _ => todo!(),
    // }

    todo!()
}

pub fn combine(x: EvalTree, y: EvalTree) -> EvalTree {
    use BindingType::*;
    use EvalTree::*;

    let bx = BindingType::from(&x);
    let by = BindingType::from(&y);

    match (bx, by) {
        (A, A) => combine_arrays(x, y),
        (A, F) | (A, H) => BoundLeftArgument(Box::new(x), Box::new(y)),
        (A, Mop) | (F, Mop) | (H, Mop) | (Jot, Mop) => {
            MonadicOperatorApplication(Box::new(x), Box::new(y))
        }
        (A, Dot) => todo!(),
        (A, Idx) => todo!(),
        (F, A) => MonadicFunctionApplication(Box::new(x), Box::new(y)),
        (F, F) => combine_functions(x, y),
        (F, H) => combine_functions(x, y),
        (F, Dot) => todo!(),
        (F, Idx) => todo!(),
        (H, F) => todo!(),
        (H, H) => todo!(),
        (H, Af) => todo!(),
        (Af, A) => {
            if let BoundLeftArgument(l, f) = x {
                DyadicFunctionApplication(l, f, Box::new(y))
            } else {
                unreachable!()
            }
        }
        (Af, F) => todo!(),
        (Dop, A) | (Dop, F) | (Dop, H) | (Jot, A) | (Jot, F) | (Jot, H) | (Dot, F) | (Dot, H) => {
            BoundRightOperand(Box::new(x), Box::new(y))
        }
        (Ref, _) => todo!(),
        (Idx, _) => todo!(),
        (_, Ref) => todo!(),
        (_, Idx) => todo!(),
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
        (BindingType::A, BindingType::Mop) => 4,
        (BindingType::A, BindingType::Dot) => 7,
        (BindingType::A, BindingType::Idx) => 4,
        (BindingType::F, BindingType::A) => 2,
        (BindingType::F, BindingType::F) => 1,
        (BindingType::F, BindingType::H) => 4,
        (BindingType::F, BindingType::Mop) => 4,
        (BindingType::F, BindingType::Idx) => 4,
        (BindingType::H, BindingType::F) => 1,
        (BindingType::H, BindingType::H) => 4,
        (BindingType::H, BindingType::Mop) => 4,
        (BindingType::H, BindingType::Idx) => 4,
        (BindingType::Af, BindingType::A) => 2,
        (BindingType::Af, BindingType::F) => 1,
        (BindingType::Mop, BindingType::H) => 4,
        (BindingType::Dop, BindingType::A) => 5,
        (BindingType::Dop, BindingType::F) => 5,
        (BindingType::Dop, BindingType::H) => 5,
        (BindingType::Jot, BindingType::A) => 5,
        (BindingType::Jot, BindingType::F) => 5,
        (BindingType::Jot, BindingType::H) => 5,
        (BindingType::Jot, BindingType::Mop) => 4,
        (BindingType::Dot, BindingType::A) => 6,
        (BindingType::Dot, BindingType::F) => 5,
        (BindingType::Dot, BindingType::H) => 5,
        (BindingType::Dot, BindingType::Dop) => 6,
        (BindingType::Ref, BindingType::A) => 7,
        (BindingType::Ref, BindingType::F) => 7,
        (BindingType::Ref, BindingType::H) => 7,
        (BindingType::Ref, BindingType::Mop) => 7,
        (BindingType::Ref, BindingType::Dop) => 7,
        (BindingType::Idx, BindingType::A) => 3,
        (BindingType::Idx, BindingType::F) => 3,
        (BindingType::Idx, BindingType::H) => 3,
        _ => panic!(),
    }
}
