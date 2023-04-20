use crate::{Size, TensorExpression};

impl TensorExpression {
    pub fn sizes(&self) -> Vec<Size> {
        match self {
            TensorExpression::KroneckerDeltas(_) => vec![],
            TensorExpression::DotProduct {
                terms,
                rank_combinations,
            } => TensorExpression::size_dot_product(terms, rank_combinations),
            TensorExpression::DirectProduct(terms) => TensorExpression::size_direct_product(terms),
        }
    }
}

#[cfg(test)]
mod tests {
    use opensrdk_linear_algebra::generate_rank_combinations;

    use crate::expression::tensor_expression::TensorExpression;
    use crate::{new_variable_tensor, Size};

    #[test]
    fn it_works() {
        let id = "x";
        let ex = new_variable_tensor((id).to_string(), vec![Size::Many, Size::Many]);

        let k = TensorExpression::KroneckerDeltas(vec![[1, 1], [2, 2]]);
        let k_size = k.sizes();
        assert_eq!(k_size, vec![]);

        let rank_combination = generate_rank_combinations(&[[2, 2]]);

        let dop = TensorExpression::DotProduct {
            terms: vec![ex.clone(), ex.clone()],
            rank_combinations: rank_combination.to_vec(),
        };
        let dp_size = dop.sizes();
        println!("{:?}", dp_size);

        let dip = TensorExpression::DirectProduct(vec![ex.clone(), ex.clone()]);
        let dip_size = dip.sizes();
        println!("{:?}", dip_size);
    }
}
