use crate::MatrixExpression;
use std::collections::HashSet;

impl MatrixExpression {
    pub fn variable_ids(&self) -> HashSet<&str> {
        match self {
            MatrixExpression::T(v) => v.variable_ids(),
            MatrixExpression::Inv(v) => v.variable_ids(),
            MatrixExpression::Det(v) => v.variable_ids(),
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use std::collections::HashSet;

//     use crate::new_variable;

//     #[test]
//     fn it_works() {
//         let id = "x";
//         let a = HashSet::from([id; 1]);
//         let ea = new_variable((id).to_string());
//         let ha = ea.variable_ids();

//         assert_eq!(a, ha);
//     }
// }
