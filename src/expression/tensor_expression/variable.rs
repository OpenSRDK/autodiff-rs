use crate::{Expression, Size, TensorExpression};
use std::collections::HashSet;

pub fn new_variable_tensor(id: String, sizes: Vec<Size>) -> Expression {
    Expression::Variable(id, sizes)
}

impl TensorExpression {
    pub fn variable_ids(&self) -> HashSet<&str> {
        match self {
            TensorExpression::KroneckerDeltas(_) => HashSet::new(),
            TensorExpression::DotProduct {
                terms,
                rank_combinations: _,
            } => terms.iter().map(|t| t.variable_ids()).flatten().collect(),
            TensorExpression::DirectProduct(terms) => {
                terms.iter().map(|t| t.variable_ids()).flatten().collect()
            }
        }
    }
}
