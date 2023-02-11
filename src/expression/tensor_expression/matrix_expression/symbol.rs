use crate::MatrixExpression;
use std::collections::HashSet;

impl MatrixExpression {
    pub fn symbols(&self) -> HashSet<String> {
        match self {
            MatrixExpression::Mat(v) => v.symbols(),
            MatrixExpression::Constant(v) => HashSet::new(),
            MatrixExpression::Inv(v) => v.symbols(),
            MatrixExpression::Det(v) => v.symbols(),
        }
    }
}
