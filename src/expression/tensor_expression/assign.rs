use super::operations::prod::InnerProd;
use crate::{ConstantValue, Expression, TensorExpression};
use std::collections::HashMap;

impl TensorExpression {
    pub fn assign(self, values: &HashMap<&str, ConstantValue>) -> Expression {
        match self {
            TensorExpression::KroneckerDeltas(_) => self.into(),
            TensorExpression::InnerProd {
                terms: v,
                rank_combinations,
            } => v
                .into_iter()
                .map(|v| v.assign(values))
                .inner_prod(&rank_combinations),
        }
    }
}