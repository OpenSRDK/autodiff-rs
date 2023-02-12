use crate::{ConstantValue, Expression, MatrixExpression};
use std::collections::HashMap;

impl MatrixExpression {
    pub fn assign(self, values: &HashMap<&str, ConstantValue>) -> Expression {
        match self {
            MatrixExpression::T(v) => v.assign(values).t(),
            MatrixExpression::Inv(v) => v.assign(values).inv(),
            MatrixExpression::Det(v) => v.assign(values).det(),
        }
    }
}
