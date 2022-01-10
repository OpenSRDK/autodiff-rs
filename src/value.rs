use opensrdk_linear_algebra::Matrix;

#[derive(Clone)]
pub enum Value {
    Scalar(f64),
    Matrix(Matrix),
}

impl Value {
    pub fn as_scalar(&self) -> f64 {
        if let Value::Scalar(v) = self {
            *v
        } else {
            panic!()
        }
    }

    pub fn as_matrix(self) -> Matrix {
        if let Value::Matrix(v) = self {
            v
        } else {
            panic!()
        }
    }

    pub fn as_matrix_ref(&self) -> &Matrix {
        if let Value::Matrix(v) = self {
            v
        } else {
            panic!()
        }
    }
}
