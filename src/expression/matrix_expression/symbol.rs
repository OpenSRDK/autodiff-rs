use crate::MatrixExpression;
use std::collections::HashSet;

impl MatrixExpression {
    pub fn symbols(&self) -> HashSet<&str> {
        match self {
            MatrixExpression::T(v) => v.symbols(),
            MatrixExpression::Inv(v) => v.symbols(),
            MatrixExpression::Det(v) => v.symbols(),
        }
    }
}
