use super::operations::{DirectProduct, DotProduct};
use crate::{ConstantValue, Expression, TensorExpression};
use std::collections::HashMap;

impl TensorExpression {
    pub fn assign(self, variables: &HashMap<&str, ConstantValue>) -> Expression {
        match self {
            TensorExpression::KroneckerDeltas(_) => self.into(),
            TensorExpression::DotProduct {
                terms,
                rank_combinations,
            } => terms
                .into_iter()
                .map(|t| t.assign(variables))
                .dot_product(&rank_combinations),
            TensorExpression::DirectProduct(terms) => terms
                .into_iter()
                .map(|t| t.assign(variables))
                .direct_product(),
        }
    }
}
