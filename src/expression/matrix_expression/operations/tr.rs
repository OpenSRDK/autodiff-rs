use crate::{tensor_expression::operations::dot::DotProduct, Expression};
use opensrdk_linear_algebra::generate_rank_combination_id;
use std::iter::once;

impl Expression {
    pub fn tr(self) -> Expression {
        let id = generate_rank_combination_id();
        let tensor =
            once(self).dot_product(&[vec![(0, id.clone()), (1, id)].into_iter().collect()]);

        tensor
    }
}
