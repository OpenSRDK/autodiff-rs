pub mod assign;
pub mod differential;
pub mod operations;
pub mod size;
pub mod symbol;
pub mod tex_code;

pub use assign::*;
pub use differential::*;
pub use operations::*;
use serde::{Deserialize, Serialize};
pub use size::*;
pub use symbol::*;
pub use tex_code::*;

use crate::TensorExpression;
use opensrdk_linear_algebra::Matrix;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum MatrixExpression {
    Mat(Box<TensorExpression>),
    Constant(Matrix),
    T(Box<MatrixExpression>),
    Inv(Box<MatrixExpression>),
    Det(Box<MatrixExpression>),
}

impl MatrixExpression {
    pub fn as_tensor(self) -> TensorExpression {
        TensorExpression::Matrix(self.into())
    }
}

impl TensorExpression {
    pub fn to_mat(self) -> MatrixExpression {
        let sizes = self.sizes();
        if sizes.len() != 2 {
            panic!("The rank of the argument must be 2.");
        }

        if let TensorExpression::Constant(v) = self {
            return MatrixExpression::Constant(v.to_mat()).into();
        }

        MatrixExpression::Mat(self.into())
    }
}
