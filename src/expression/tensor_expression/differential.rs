use crate::{Expression, TensorExpression};

impl TensorExpression {
    pub fn differential(&self, symbols: &[&str]) -> Vec<Expression> {
        match self {
            TensorExpression::KroneckerDeltas(_) => {
                vec![0.0.into(); symbols.len()]
            }
            TensorExpression::InnerProd {
                terms,
                rank_combinations,
            } => TensorExpression::diff_inner_prod(terms, rank_combinations, symbols),
        }
    }
}
