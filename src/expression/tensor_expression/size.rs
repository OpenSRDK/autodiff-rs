use crate::{Size, TensorExpression};

impl TensorExpression {
    pub fn sizes(&self) -> Vec<Size> {
        match self {
            TensorExpression::KroneckerDeltas(_) => vec![],
            TensorExpression::DotProduct {
                terms,
                rank_combinations,
            } => TensorExpression::size_dot_product(terms, rank_combinations),
            TensorExpression::DirectProduct(terms) => TensorExpression::size_direct_product(terms),
        }
    }
}
