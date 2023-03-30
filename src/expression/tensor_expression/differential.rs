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

// #[cfg(test)]
// mod tests {

//     use crate::TensorExpression;

//     #[test]
//     fn differential() {
//         let x = TensorExpression::KroneckerDeltas(vec![[1, 1], [2, 2]]);
//         let y = TensorExpression::KroneckerDeltas(vec![[1, 1], [2, 2]]);
//         let z = TensorExpression::KroneckerDeltas(vec![[1, 1], [2, 2]]);
//         let x = x.dot(y, &[[0, 0], [1, 1]]).dot(z, &[[0, 0], [1, 1]]);

//         let diff = x.differential(&["x"]);
//         println!("diff:{:?}", diff);
//         assert_eq!(diff.len(), 1);
//     }
// }
