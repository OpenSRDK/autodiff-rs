use crate::{Expression, MatrixExpression};

impl MatrixExpression {
    pub fn differential(&self, symbols: &[&str]) -> Vec<Expression> {
        match self {
            MatrixExpression::T(v) => MatrixExpression::diff_t(v, symbols),
            MatrixExpression::Inv(v) => MatrixExpression::diff_inv(v, symbols),
            MatrixExpression::Det(v) => MatrixExpression::diff_det(v, symbols),
        }
    }
}
