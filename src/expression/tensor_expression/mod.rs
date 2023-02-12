pub mod assign;
pub mod differential;
pub mod operations;
pub mod symbol;
pub mod tex_code;

pub use assign::*;
pub use differential::*;
use serde::{Deserialize, Serialize};
pub use symbol::*;
pub use tex_code::*;

use crate::Expression;
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TensorExpression {
    KroneckerDeltas(Vec<[usize; 2]>),
    InnerProd {
        terms: Vec<Expression>,
        rank_combinations: Vec<HashMap<usize, String>>,
    },
}

impl Expression {
    pub fn tensor(self) -> Option<TensorExpression> {
        match self {
            Expression::Tensor(t) => Some(*t),
            _ => None,
        }
    }

    pub fn into_tensor(self) -> TensorExpression {
        match self {
            Expression::Tensor(t) => *t,
            _ => panic!("The expression is not a tensor expression."),
        }
    }
}

impl From<TensorExpression> for Expression {
    fn from(t: TensorExpression) -> Self {
        Expression::Tensor(t.into())
    }
}
