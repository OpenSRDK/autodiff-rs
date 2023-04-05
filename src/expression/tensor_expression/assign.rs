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

#[cfg(test)]
mod tests {

    use opensrdk_linear_algebra::generate_rank_combinations;

    use crate::expression::tensor_expression::TensorExpression;
    use crate::expression::Expression;
    use crate::{new_variable, new_variable_tensor, ConstantValue, Size};
    use std::collections::HashMap;

    #[test]
    fn it_works() {
        let id1 = "x";
        let id2 = "y";

        let mut variables = HashMap::new();
        variables.insert("x", ConstantValue::Scalar(1.0));
        variables.insert("y", ConstantValue::Scalar(2.0));

        let ka = TensorExpression::KroneckerDeltas(vec![[1, 1], [2, 2]]).assign(&variables);
        assert_eq!(
            ka,
            TensorExpression::KroneckerDeltas(vec![[1, 1], [2, 2]]).into()
        );

        let edo = TensorExpression::DotProduct {
            terms: vec![
                new_variable((id1).to_string()),
                new_variable((id2).to_string()),
            ],
            rank_combinations: generate_rank_combinations(&[[2, 2]]).to_vec(),
        };
        let doa = edo.assign(&variables);
        println!("doa: {:?}", doa);

        let ed = TensorExpression::DirectProduct(vec![
            new_variable((id1).to_string()),
            new_variable((id2).to_string()),
        ]);
        let da = ed.assign(&variables);
        assert_eq!(
            da,
            TensorExpression::DirectProduct(vec![
                Expression::Constant(ConstantValue::Scalar(1.0)),
                Expression::Constant(ConstantValue::Scalar(2.0)),
            ])
            .into()
        )
    }
}
