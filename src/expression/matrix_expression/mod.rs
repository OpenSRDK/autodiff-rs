pub mod assign;
pub mod differential;
pub mod operations;
pub mod size;
pub mod tex_code;
pub mod variable;

pub use assign::*;
pub use differential::*;
pub use operations::*;
use serde::{Deserialize, Serialize};
pub use size::*;
pub use tex_code::*;
pub use variable::*;

use crate::Expression;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MatrixExpression {
    T(Box<Expression>),
    Inv(Box<Expression>),
    Det(Box<Expression>),
}

impl Expression {
    pub fn matrix(self) -> Option<MatrixExpression> {
        match self {
            Expression::Matrix(t) => Some(*t),
            _ => None,
        }
    }

    pub fn into_matrix(self) -> MatrixExpression {
        match self {
            Expression::Matrix(t) => *t,
            _ => panic!("The expression is not a matrix expression."),
        }
    }
}

// impl TensorExpression {
//     pub fn to_mat(self) -> MatrixExpression {
//         let sizes = self.sizes();
//         if sizes.len() != 2 {
//             panic!("The rank of the argument must be 2.");
//         }

//         if let TensorExpression::Constant(v) = self {
//             return MatrixExpression::Constant(v.to_mat()).into();
//         }

//         MatrixExpression::Mat(self.into())
//     }
// }

impl From<MatrixExpression> for Expression {
    fn from(m: MatrixExpression) -> Self {
        Expression::Matrix(m.into())
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, ops::Add};

    use opensrdk_linear_algebra::Matrix;

    use crate::{Expression, MatrixExpression};

    #[test]
    fn it_works() {
        let len = 7usize;
        let a = Matrix::from(len, vec![1.0; len * len]).unwrap();

        let mea_mat_t = MatrixExpression::T(Box::new(Expression::from(a.clone())));
        let ea_mat_t = Expression::from(mea_mat_t.clone());
        let mea_mat_t_mat = Box::new(ea_mat_t.clone()).into_matrix();

        assert_eq!(mea_mat_t, mea_mat_t_mat);

        let mea_mat_inv = MatrixExpression::Inv(Box::new(Expression::from(a.clone())));
        let ea_mat_inv = Expression::from(mea_mat_inv.clone());
        let mea_mat_inv_mat = Box::new(ea_mat_inv.clone()).into_matrix();

        assert_eq!(mea_mat_inv, mea_mat_inv_mat);

        let mea_mat_det = MatrixExpression::Det(Box::new(Expression::from(a.clone())));
        let ea_mat_det = Expression::from(mea_mat_det.clone());
        let mea_mat_det_mat = Box::new(ea_mat_det.clone()).into_matrix();

        assert_eq!(mea_mat_det, mea_mat_det_mat);
    }
}
