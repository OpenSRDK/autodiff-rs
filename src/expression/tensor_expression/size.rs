use crate::{Size, TensorExpression};

impl TensorExpression {
    pub fn sizes(&self) -> Vec<Size> {
        match self {
            TensorExpression::KroneckerDeltas(_) => vec![],
            TensorExpression::InnerProd {
                terms,
                rank_combinations,
            } => TensorExpression::size_inner_prod(terms, rank_combinations),
        }
    }
}
