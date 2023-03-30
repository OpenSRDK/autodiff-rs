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

// #[cfg(test)]
// mod tests {
//     use opensrdk_linear_algebra::Matrix;

//     use crate::expression::tensor_expression::TensorExpression;
//     use crate::expression::Expression;
//     use crate::{new_variable_tensor, ConstantValue, Size};
//     use std::collections::HashMap;

//     #[test]
//     fn it_works() {
//         let id = "x";
//         let ea = new_variable_tensor((id).to_string(), vec![Size::Many, Size::Many]);

//         let ea_t = ea.clone().t();
//         let mut hash1 = HashMap::new();

//         let len = 7usize;
//         let a = Matrix::from(len, vec![1.0; len * len]).unwrap();

//         hash1.insert(id, ConstantValue::Matrix(a.clone()));

//         let result = ea_t.assign(&hash1);

//         assert_eq!(result, Expression::from(ConstantValue::Matrix(a.t())));

//         let mut variables = HashMap::new();
//         variables.insert("x", ConstantValue::Scalar(1.0));
//         variables.insert("y", ConstantValue::Scalar(2.0));

//         let expr = TensorExpression::KroneckerDeltas(vec![[1, 1], [2, 2]]).assign(&variables);
//         assert_eq!(expr, Expression::Constant(ConstantValue::Scalar(2.0)));
//     }
// }
