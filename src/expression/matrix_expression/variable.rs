use crate::MatrixExpression;
use std::collections::HashSet;

impl MatrixExpression {
    pub fn variable_ids(&self) -> HashSet<&str> {
        match self {
            MatrixExpression::T(v) => v.variable_ids(),
            MatrixExpression::Inv(v) => v.variable_ids(),
            MatrixExpression::Det(v) => v.variable_ids(),
        }
    }
}
