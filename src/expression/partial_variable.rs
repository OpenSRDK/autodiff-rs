use crate::{Expression, ExpressionArray};
use std::collections::HashMap;

pub fn new_partial_variable(v: ExpressionArray) -> Expression {
    Expression::PartialVariable(v)
}

impl Expression {
    pub(crate) fn diff_partial_variable(
        v: &ExpressionArray,
        variable_ids: &[&str],
    ) -> Vec<Expression> {
        variable_ids
            .iter()
            .map(|&variable_id| {
                new_partial_variable(ExpressionArray::from_factory(
                    v.sizes().to_vec(),
                    |indices| v[indices].differential(&[variable_id])[0].clone(),
                ))
            })
            .collect()
    }
}
