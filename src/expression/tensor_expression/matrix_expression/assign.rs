use crate::{MatrixExpression, Value};
use std::collections::HashMap;

impl MatrixExpression {
    pub fn assign(self, values: &HashMap<&str, Value>) -> MatrixExpression {
        match self {
            MatrixExpression::Mat(v) => v.assign(values).to_mat(),
            MatrixExpression::Constant(v) => self,
            MatrixExpression::Inv(v) => v.assign(values).to_mat().inv(),
            MatrixExpression::Det(v) => v.assign(values).to_mat().det(),
        }
    }
}
