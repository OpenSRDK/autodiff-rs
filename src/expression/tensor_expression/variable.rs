use crate::{Expression, Size, TensorExpression};
use std::collections::HashSet;

pub fn new_variable_tensor(id: String, sizes: Vec<Size>) -> Expression {
    Expression::Variable(id, sizes)
}

impl TensorExpression {
    pub fn variable_ids(&self) -> HashSet<&str> {
        match self {
            TensorExpression::KroneckerDeltas(_) => HashSet::new(),
            TensorExpression::InnerProd {
                terms,
                rank_combinations: _,
            } => terms.iter().map(|v| v.variable_ids()).flatten().collect(),
        }
    }
}
