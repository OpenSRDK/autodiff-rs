use opensrdk_linear_algebra::sparse::SparseTensor;

#[derive(Clone)]
pub enum Value {
    Scalar(f64),
    Tensor(SparseTensor),
}

impl Value {
    pub fn as_scalar(&self) -> f64 {
        if let Value::Scalar(v) = self {
            *v
        } else {
            panic!()
        }
    }

    pub fn as_tensor(self) -> SparseTensor {
        if let Value::Tensor(v) = self {
            v
        } else {
            panic!()
        }
    }

    pub fn as_tensor_ref(&self) -> &SparseTensor {
        if let Value::Tensor(v) = self {
            v
        } else {
            panic!()
        }
    }
}
