use opensrdk_linear_algebra::{sparse::SparseTensor, Matrix, Tensor};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum ConstantValue {
    Scalar(f64),
    Tensor(SparseTensor),
    Matrix(Matrix),
}

impl ConstantValue {
    pub fn sizes(&self) -> Vec<usize> {
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

impl ConstantValue {
    pub fn add(&self, rhs: ConstantValue) -> ConstantValue {
        match (self, rhs) {
            (ConstantValue::Scalar(lhs), ConstantValue::Scalar(rhs)) => {
                ConstantValue::Scalar(lhs + rhs)
            }
            (ConstantValue::Scalar(lhs), ConstantValue::Tensor(rhs)) => {
                ConstantValue::Tensor(lhs + rhs)
            }
            (ConstantValue::Scalar(lhs), ConstantValue::Matrix(rhs)) => {
                ConstantValue::Matrix(lhs + rhs)
            }
            (ConstantValue::Tensor(lhs), ConstantValue::Tensor(rhs)) => {
                ConstantValue::Tensor(lhs + rhs)
            }
            (ConstantValue::Tensor(lhs), ConstantValue::Scalar(rhs)) => {
                ConstantValue::Tensor(lhs.clone() + rhs)
            }
            (ConstantValue::Matrix(lhs), ConstantValue::Matrix(rhs)) => {
                ConstantValue::Matrix(lhs + rhs)
            }
            (ConstantValue::Matrix(lhs), ConstantValue::Scalar(rhs)) => {
                ConstantValue::Matrix(lhs.clone() + rhs)
            }
            _ => panic!(),
        }
    }

    pub fn sub(&self, rhs: ConstantValue) -> ConstantValue {
        match (self, rhs) {
            (ConstantValue::Scalar(lhs), ConstantValue::Scalar(rhs)) => {
                ConstantValue::Scalar(lhs - rhs)
            }
            (ConstantValue::Scalar(lhs), ConstantValue::Tensor(rhs)) => {
                ConstantValue::Tensor(lhs - rhs)
            }
            (ConstantValue::Scalar(lhs), ConstantValue::Matrix(rhs)) => {
                ConstantValue::Matrix(lhs - rhs)
            }
            (ConstantValue::Tensor(lhs), ConstantValue::Tensor(rhs)) => {
                ConstantValue::Tensor(lhs - rhs)
            }
            (ConstantValue::Tensor(lhs), ConstantValue::Scalar(rhs)) => {
                ConstantValue::Tensor(lhs.clone() - rhs)
            }
            (ConstantValue::Matrix(lhs), ConstantValue::Matrix(rhs)) => {
                ConstantValue::Matrix(lhs - rhs)
            }
            (ConstantValue::Matrix(lhs), ConstantValue::Scalar(rhs)) => {
                ConstantValue::Matrix(lhs.clone() - rhs)
            }
            _ => panic!(),
        }
    }

    pub fn mul(&self, rhs: ConstantValue) -> ConstantValue {
        match (self, rhs) {
            (ConstantValue::Scalar(lhs), ConstantValue::Scalar(rhs)) => {
                ConstantValue::Scalar(lhs * rhs)
            }
            (ConstantValue::Scalar(lhs), ConstantValue::Tensor(rhs)) => {
                ConstantValue::Tensor(lhs * rhs)
            }
            (ConstantValue::Scalar(lhs), ConstantValue::Matrix(rhs)) => {
                ConstantValue::Matrix(lhs * rhs)
            }
            (ConstantValue::Tensor(lhs), ConstantValue::Tensor(rhs)) => {
                ConstantValue::Tensor(lhs * rhs)
            }
            (ConstantValue::Tensor(lhs), ConstantValue::Scalar(rhs)) => {
                ConstantValue::Tensor(lhs.clone() * rhs)
            }
            (ConstantValue::Matrix(lhs), ConstantValue::Matrix(rhs)) => {
                ConstantValue::Matrix(lhs * rhs)
            }
            (ConstantValue::Matrix(lhs), ConstantValue::Scalar(rhs)) => {
                ConstantValue::Matrix(lhs.clone() * rhs)
            }
            _ => panic!(),
        }
    }

    pub fn div(self, rhs: &ConstantValue) -> ConstantValue {
        match (self, rhs) {
            (ConstantValue::Scalar(lhs), ConstantValue::Scalar(rhs)) => {
                ConstantValue::Scalar(lhs / rhs)
            }
            (ConstantValue::Scalar(lhs), ConstantValue::Tensor(rhs)) => {
                ConstantValue::Tensor(lhs / rhs.clone())
            }
            (ConstantValue::Scalar(lhs), ConstantValue::Matrix(rhs)) => {
                ConstantValue::Matrix(lhs / rhs.clone())
            }
            (ConstantValue::Tensor(lhs), ConstantValue::Tensor(rhs)) => {
                ConstantValue::Tensor(lhs / rhs)
            }
            (ConstantValue::Tensor(lhs), ConstantValue::Scalar(rhs)) => {
                ConstantValue::Tensor(lhs / rhs)
            }
            (ConstantValue::Matrix(lhs), ConstantValue::Matrix(rhs)) => {
                ConstantValue::Matrix(lhs / rhs)
            }
            (ConstantValue::Matrix(lhs), ConstantValue::Scalar(rhs)) => {
                ConstantValue::Matrix(lhs / rhs)
            }
            _ => panic!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use opensrdk_linear_algebra::{mat, sparse::SparseTensor};

    use crate::ConstantValue;

    #[test]
    fn sizes() {
        let a = ConstantValue::Scalar(1.0);
        let mut bt = SparseTensor::<f64>::new(vec![2, 2]);
        bt[&[0, 0]] = 1.0;
        bt[&[0, 1]] = 2.0;
        bt[&[1, 0]] = 3.0;
        bt[&[1, 1]] = 4.0;

        let b = ConstantValue::Tensor(bt);
        let cm = mat!(
          1.0, 2.0;
          3.0, 4.0
        );

        let c = ConstantValue::Matrix(cm);

        assert_eq!(a.sizes(), vec![]);
        assert_eq!(b.sizes(), vec![2, 2]);
        assert_eq!(c.sizes(), vec![2, 2]);
    }

    fn elems() {
        todo!()
    }
}
