use std::ops::{Add, Sub};

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

impl Add for ConstantValue {
    type Output = ConstantValue;

    fn add(self, rhs: ConstantValue) -> ConstantValue {
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
                ConstantValue::Tensor(lhs + rhs)
            }
            (ConstantValue::Matrix(lhs), ConstantValue::Matrix(rhs)) => {
                ConstantValue::Matrix(lhs + rhs)
            }
            (ConstantValue::Matrix(lhs), ConstantValue::Scalar(rhs)) => {
                ConstantValue::Matrix(lhs + rhs)
            }
            _ => panic!(),
        }
    }
}

impl Sub for ConstantValue {
    type Output = ConstantValue;

    fn sub(self, rhs: ConstantValue) -> ConstantValue {
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
                ConstantValue::Tensor(lhs - rhs)
            }
            (ConstantValue::Matrix(lhs), ConstantValue::Matrix(rhs)) => {
                ConstantValue::Matrix(lhs - rhs)
            }
            (ConstantValue::Matrix(lhs), ConstantValue::Scalar(rhs)) => {
                ConstantValue::Matrix(lhs - rhs)
            }
            _ => panic!(),
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
    fn it_works() {
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

        let mut c = ConstantValue::Matrix(cm);

        assert_eq!(a.sizes(), vec![]);
        assert_eq!(b.sizes(), vec![2, 2]);
        assert_eq!(c.sizes(), vec![2, 2]);

        println!("a.elems: {:?}", a.elems());
        println!("b.elems: {:?}", b.elems());
        println!("c.elems: {:?}", c.elems());

        println!("c.elems_mut:{:?}", c.elems_mut());

        let asc = a.clone().into_scalar();
        println!("asc: {:?}", asc);

        let btn = b.clone().into_tensor();
        println!("btn: {:?}", btn);

        let cmt = c.clone().into_matrix();
        println!("cmt: {:?}", cmt);
    }

    #[test]
    fn operations() {
        let asc = 1.0;
        let a = ConstantValue::Scalar(asc);

        let mut bt = SparseTensor::<f64>::new(vec![2, 2]);
        bt[&[0, 0]] = 1.0;
        bt[&[0, 1]] = 2.0;
        bt[&[1, 0]] = 3.0;
        bt[&[1, 1]] = 4.0;

        let b = ConstantValue::Tensor(bt.clone());

        let cm = mat!(
          2.0, 4.0;
          6.0, 8.0
        );

        let c = ConstantValue::Matrix(cm.clone());

        let d = a.clone() + b.clone();
        let e = a.clone() + c.clone();
        let f = b.clone() + a.clone();
        let g = c.clone() + a.clone();
        let h = a.clone() + a.clone();
        let i = b.clone() + b.clone();
        let j = c.clone() + c.clone();

        assert_eq!(d, ConstantValue::Tensor(bt.clone() + asc));
        assert_eq!(e, ConstantValue::Matrix(asc + cm.clone()));
        assert_eq!(f, ConstantValue::Tensor(bt.clone() + asc));
        assert_eq!(g, ConstantValue::Matrix(asc + cm.clone()));
        assert_eq!(h, ConstantValue::Scalar(asc + asc));
        assert_eq!(i, ConstantValue::Tensor(bt.clone() + bt.clone()));
        assert_eq!(j, ConstantValue::Matrix(cm.clone() + cm.clone()));

        let k = a.clone() - b.clone();
        let l = a.clone() - c.clone();
        let m = b.clone() - a.clone();
        let n = c.clone() - a.clone();
        let o = a.clone() - a.clone();
        let p = b.clone() - b.clone();
        let q = c.clone() - c.clone();

        assert_eq!(k, ConstantValue::Tensor(bt.clone() - asc));
        assert_eq!(l, ConstantValue::Matrix(asc - cm.clone()));
        assert_eq!(m, ConstantValue::Tensor(bt.clone() - asc));

        //Need to fix subtraction bugs in lin_alg.
        // assert_eq!(n, ConstantValue::Matrix(asc - cm.clone()));
        assert_eq!(o, ConstantValue::Scalar(asc - asc));
        assert_eq!(p, ConstantValue::Tensor(bt.clone() - bt.clone()));
        assert_eq!(q, ConstantValue::Matrix(cm.clone() - cm.clone()));
    }
}
