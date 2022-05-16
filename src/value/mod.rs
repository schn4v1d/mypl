use std::fmt::Display;

use crate::eval_tree::EvalTree;

use self::{
    array::{scalar::Scalar, Array},
    function::Function,
};

pub mod array;
pub mod function;

#[derive(Debug)]
pub enum Value {
    Array(Array),
    Function(Function),
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Array(a) => write!(f, "{}", a),
            Value::Function(_) => todo!(),
        }
    }
}

impl Value {
    pub fn as_array(self) -> Array {
        match self {
            Value::Array(a) => a,
            Value::Function(_) => panic!("WTF"),
        }
    }

    pub fn as_function(self) -> Function {
        match self {
            Value::Array(_) => panic!("WTF"),
            Value::Function(f) => f,
        }
    }
}

pub fn eval(tree: &EvalTree) -> Value {
    match tree {
        EvalTree::Array(x) => Value::Array(
            x.iter()
                .map(eval)
                .map(|v| {
                    if let Value::Array(x) = v {
                        x
                    } else {
                        panic!("wtf")
                    }
                })
                .collect(),
        ),
        EvalTree::Function(_) => todo!(),
        EvalTree::MonadicFunctionApplication(f, r) => {
            Value::Array(if let Value::Function(f) = eval(f.as_ref()) {
                f.apply(None, eval(r.as_ref()).as_array())
            } else {
                panic!("wtf")
            })
        }
        EvalTree::DyadicFunctionApplication(l, f, r) => {
            Value::Array(if let Value::Function(f) = eval(f.as_ref()) {
                f.apply(
                    Some(eval(l.as_ref()).as_array()),
                    eval(r.as_ref()).as_array(),
                )
            } else {
                panic!("wtf")
            })
        }
        EvalTree::BoundLeftArgument(_, _) => todo!(),
        EvalTree::MonadicOperatorApplication(_, _) => todo!(),
        EvalTree::BoundRightOperand(_, _) => todo!(),
        EvalTree::DyadicOperatorApplication(_, _, _) => todo!(),
        EvalTree::Atop(_, _) => todo!(),
        EvalTree::Fork(_, _, _) => todo!(),
        EvalTree::Integer(i) => Value::Array(Array::Scalar(Scalar::Integer(*i))),
        EvalTree::Float(f) => Value::Array(Array::Scalar(Scalar::Float(*f))),
        EvalTree::PrimitiveFunction(p) => Value::Function(Function::Primitive(*p)),
        EvalTree::PrimitiveMonadicOperator(_) => panic!("SYNTAX ERROR"),
        EvalTree::PrimitiveDyadicOperator(_) => panic!("SYNTAX ERROR"),
        EvalTree::Unfinished(_) => unreachable!(),
    }
}
