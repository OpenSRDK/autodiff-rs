use crate::{Expression, TensorExpression};

impl TensorExpression {
    pub fn differential(&self, variable_ids: &[&str]) -> Vec<Expression> {
        match self {
            TensorExpression::KroneckerDeltas(_) => {
                vec![0.0.into(); variable_ids.len()]
            }
            TensorExpression::DotProduct {
                terms,
                rank_combinations,
            } => TensorExpression::diff_dot_product(terms, rank_combinations, variable_ids),
            TensorExpression::DirectProduct(terms) => {
                TensorExpression::diff_direct_product(terms, variable_ids)
            }
        }
    }
}
