use std::iter::once;

use crate::{tensor_expression::operations::prod::InnerProd, Expression, MatrixExpression};
use opensrdk_linear_algebra::generate_rank_combination_id;

impl MatrixExpression {
    pub fn tr(self) -> Expression {
        let id = generate_rank_combination_id();
        let tensor = once(self.as_tensor())
            .inner_prod(&[vec![(0, id.clone()), (1, id)].into_iter().collect()]);

        tensor.as_scalar()
    }
}
