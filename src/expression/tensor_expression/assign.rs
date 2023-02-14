use super::operations::dot::DotProduct;
use crate::{ConstantValue, Expression, TensorExpression};
use std::collections::HashMap;

impl TensorExpression {
    pub fn assign(self, variables: &HashMap<&str, ConstantValue>) -> Expression {
        match self {
            TensorExpression::KroneckerDeltas(_) => self.into(),
            TensorExpression::DotProduct {
                terms: v,
                rank_combinations,
            } => v
                .into_iter()
                .map(|v| v.assign(variables))
                .dot_product(&rank_combinations),
        }
    }
}
