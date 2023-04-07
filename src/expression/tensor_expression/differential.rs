use crate::{Expression, TensorExpression};

impl TensorExpression {
    pub fn differential(&self, variable_ids: &[&str]) -> Vec<Expression> {
        match self {
            TensorExpression::KroneckerDeltas(_) => {
                vec![0.0.into(); variable_ids.len()]
            }
            TensorExpression::DotProduct {
                terms,
                rank_combinations,
            } => TensorExpression::diff_dot_product(terms, rank_combinations, variable_ids),
            TensorExpression::DirectProduct(terms) => {
                TensorExpression::diff_direct_product(terms, variable_ids)
            }
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
        let k_diff = k.differential(&[id]);
        assert_eq!(k_diff, vec![0.0.into(); 1]);

        let rank_combination = generate_rank_combinations(&[[2, 2]]);

        let dop = TensorExpression::DotProduct {
            terms: vec![ex.clone(), ex.clone()],
            rank_combinations: rank_combination.to_vec(),
        };
        let dp_diff = dop.differential(&[id]);
        println!("{:?}", dp_diff);

        let dip = TensorExpression::DirectProduct(vec![ex.clone(), ex.clone()]);
        let dip_diff = dip.differential(&[id]);
        println!("{:?}", dip_diff);
    }
}
