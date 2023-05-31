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
use std::{
    collections::{HashMap, HashSet},
    ops::Mul,
};

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

impl TensorExpression {
    pub fn elems(self) -> Vec<Expression> {
        match self {
            TensorExpression::KroneckerDeltas(v) => v
                .iter()
                .map(|i| Expression::from(vec![i.clone()[0] as f64, i.clone()[1] as f64]))
                .collect::<Vec<Expression>>(),
            TensorExpression::DotProduct {
                terms,
                rank_combinations,
            } => terms,
            TensorExpression::DirectProduct(v) => v,
        }
    }
}

impl TensorExpression {
    pub fn to_vec(self) -> Vec<Expression> {
        match self {
            TensorExpression::KroneckerDeltas(v) => todo!(),
            TensorExpression::DotProduct {
                terms,
                rank_combinations,
            } => terms
                .iter()
                .zip(rank_combinations.iter())
                .map(|i| {
                    i.1.clone().sort().dedup().map(|j| {
                        i.iter()
                            .filter(|k| k.1.clone() == j)
                            .fold(Expression::from(1f64), |mut l, m| {
                                l.mul(m.0.clone()).collect::<Vec<Expression>>()
                            })
                    })
                })
                .collect::<Vec<Expression>>(),
            // rank_combinations
            //     .into_iter()
            //     .map(|h| h.keys().cloned())
            //     .collect::<Vec<String>>()
            //     .sort()
            //     .dedup()
            //     .iter()
            //     .map(|i: HashMap<usize, String>| {
            //         terms
            //             .iter()
            //             .zip(rank_combinations.iter())
            //             .filter(|j| j.1.clone() == i)
            //             .fold(Expression::from(1f64), |mut k, l| k.mul(l.0.clone()))
            //     })
            //     .collect::<Vec<Expression>>(),
            //ここにiter()つけて、同じrank_combinationsを持つものを
            TensorExpression::DirectProduct(v) => todo!(),
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

        let ec = Expression::from(c);
        // TODO: into_tensor is for internal use. don't use here.
        // TODO: At first it is needed to extract ConstantValue from ec, and then convert it to SparseTensor.
        // let tec = ec.into_tensor();
        // println!("{:?}", tec);
    }
}
