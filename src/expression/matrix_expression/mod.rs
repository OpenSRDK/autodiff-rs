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
        let len = 3usize;
        let a = Matrix::from(len, vec![1.0, 3.0, 4.0, 0.0, 1.0, 0.0, 0.0, 0.0, 3.0]).unwrap();
        let ea = Expression::from(a.clone());

        let a_t = a.clone().t();
        let ea_t = ea.clone().t();

        assert_eq!(Expression::from(a_t), ea_t);

        let a_inv = a.clone().getrf().unwrap().getri().unwrap();
        let ea_inv = ea.clone().inv();

        assert_eq!(Expression::from(a_inv), ea_inv);

        let a_det = a.clone().getrf().unwrap().0.trdet();
        let ea_det = ea.clone().det();

        println!("{:?}", ea_det);
        assert_eq!(Expression::from(a_det), ea_det);
    }
}
