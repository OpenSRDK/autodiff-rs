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

#[cfg(test)]
mod tests {
    use opensrdk_linear_algebra::Matrix;

    use crate::Expression;
    #[test]
    fn it_works() {
        fn it_works() {
            let len = 3usize;
            let a = Matrix::from(len, vec![1.0, 3.0, 4.0, 0.0, 1.0, 0.0, 0.0, 0.0, 3.0]).unwrap();
            let ea = Expression::from(a.clone());

            let tr_a = a.clone().tr();
            let tr_ea = ea.clone().tr();

            assert_eq!(Expression::from(tr_a), tr_ea);
        }
    }
}
