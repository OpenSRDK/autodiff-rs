use opensrdk_linear_algebra::{sparse::SparseTensor, Matrix};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ConstantValue {
    Scalar(f64),
    Tensor(SparseTensor),
    Matrix(Matrix),
}

impl ConstantValue {
    pub fn into_scalar(&self) -> f64 {
        if let ConstantValue::Scalar(v) = self {
            *v
        } else {
            panic!()
        }
    }

    pub fn into_tensor(self) -> SparseTensor {
        if let ConstantValue::Tensor(v) = self {
            v
        } else {
            panic!()
        }
    }

    pub fn into_tensor_ref(&self) -> &SparseTensor {
        if let ConstantValue::Tensor(v) = self {
            v
        } else {
            panic!()
        }
    }

    pub fn into_matrix(self) -> Matrix {
        if let ConstantValue::Matrix(v) = self {
            v
        } else {
            panic!()
        }
    }

    pub fn into_matrix_ref(&self) -> &Matrix {
        if let ConstantValue::Matrix(v) = self {
            v
        } else {
            panic!()
        }
    }
}
