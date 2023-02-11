use opensrdk_linear_algebra::Vector;

use crate::{Expression, MatrixExpression, TensorExpression, Value};
use std::collections::HashMap;

impl MatrixExpression {
    pub fn assign(self, values: &HashMap<&str, Value>) -> MatrixExpression {
        match self {
            MatrixExpression::Mat(v) => v.assign(values).to_mat(),
            MatrixExpression::Constant(_) => self,
            MatrixExpression::T(v) => v.assign(values).t(),
            MatrixExpression::Inv(v) => v.assign(values).inv(),
            MatrixExpression::Det(v) => {
                let det = v.assign(values).det();
                if let &Expression::Constant(v) = &det {
                    return MatrixExpression::Constant(vec![v].col_mat());
                } else if let Expression::DegeneratedTensor(v) = det {
                    if let TensorExpression::Matrix(m) = *v {
                        // This must be MatrixExpression::Det
                        return *m;
                    }
                }

                panic!()
            }
        }
    }
}
