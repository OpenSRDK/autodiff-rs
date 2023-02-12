use opensrdk_linear_algebra::{sparse::SparseTensor, Matrix, Tensor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ConstantValue {
    Scalar(f64),
    Tensor(SparseTensor),
    Matrix(Matrix),
}

impl ConstantValue {
    pub fn size(&self) -> Vec<usize> {
        match self {
            ConstantValue::Scalar(_) => vec![],
            ConstantValue::Tensor(v) => {
                (0..v.rank()).into_iter().map(|rank| v.size(rank)).collect()
            }
            ConstantValue::Matrix(v) => vec![v.rows(), v.cols()],
        }
    }

    pub fn elems(&self) -> Vec<f64> {
        match self {
            ConstantValue::Scalar(v) => vec![*v],
            ConstantValue::Tensor(v) => v.elems().into_iter().map(|(_, v)| *v).collect(),
            ConstantValue::Matrix(v) => v.elems().to_vec(),
        }
    }

    pub fn elems_mut(&mut self) -> Vec<&mut f64> {
        match self {
            ConstantValue::Scalar(v) => vec![v],
            ConstantValue::Tensor(v) => v.elems_mut().into_iter().map(|(_, v)| v).collect(),
            ConstantValue::Matrix(v) => v.elems_mut().iter_mut().collect(),
        }
    }

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
