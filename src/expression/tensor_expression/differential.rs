use crate::{Expression, TensorExpression};

impl TensorExpression {
    pub fn differential(&self, variable_ids: &[&str]) -> Vec<Expression> {
        match self {
            TensorExpression::KroneckerDeltas(_) => {
                vec![0.0.into(); variable_ids.len()]
            }
            TensorExpression::InnerProd {
                terms,
                rank_combinations,
            } => TensorExpression::diff_inner_prod(terms, rank_combinations, variable_ids),
        }
    }
}
