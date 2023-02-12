use super::operations::prod::InnerProd;
use crate::{ConstantValue, Expression, TensorExpression};
use std::collections::HashMap;

impl TensorExpression {
    pub fn assign(self, variables: &HashMap<&str, ConstantValue>) -> Expression {
        match self {
            TensorExpression::KroneckerDeltas(_) => self.into(),
            TensorExpression::InnerProd {
                terms: v,
                rank_combinations,
            } => v
                .into_iter()
                .map(|v| v.assign(variables))
                .inner_prod(&rank_combinations),
        }
    }
}
