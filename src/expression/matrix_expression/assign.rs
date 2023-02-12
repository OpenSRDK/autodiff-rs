use crate::{ConstantValue, Expression, MatrixExpression};
use std::collections::HashMap;

impl MatrixExpression {
    pub fn assign(self, variables: &HashMap<&str, ConstantValue>) -> Expression {
        match self {
            MatrixExpression::T(v) => v.assign(variables).t(),
            MatrixExpression::Inv(v) => v.assign(variables).inv(),
            MatrixExpression::Det(v) => v.assign(variables).det(),
        }
    }
}
