pub mod assign;
pub mod differential;
pub mod operations;
pub mod size;
pub mod tex_code;
pub mod variable;

pub use assign::*;
pub use differential::*;
use serde::{Deserialize, Serialize};
pub use size::*;
pub use tex_code::*;
pub use variable::*;

use crate::Expression;
use std::collections::{HashMap, HashSet};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum TensorExpression {
    KroneckerDeltas(Vec<[usize; 2]>),
    DotProduct {
        terms: Vec<Expression>,
        rank_combinations: Vec<HashMap<usize, String>>,
    },
    DirectProduct(Vec<Expression>),
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

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, ops::Add};

    use opensrdk_linear_algebra::{sparse::SparseTensor, Matrix};

    use crate::{Expression, MatrixExpression};

    #[test]
    fn it_works() {
        let a = 5.0f64;
        let b = vec![a; 8];
        let mut hash = HashMap::new();
        hash.insert(vec![3usize; 8], 2.0);
        hash.insert(vec![1usize; 8], 3.0);
        hash.insert(vec![4usize; 8], 4.0);
        hash.insert(vec![5usize; 8], 2.0);
        let c = SparseTensor::from(vec![6usize; 8], hash).unwrap();

        let ea = Expression::from(a);
        let tea = ea.into_tensor();
        println!("{:?}", tea);
    }
}
