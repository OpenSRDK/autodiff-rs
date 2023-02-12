use crate::{Expression, MatrixExpression};

impl MatrixExpression {
    pub fn differential(&self, variable_ids: &[&str]) -> Vec<Expression> {
        match self {
            MatrixExpression::T(v) => MatrixExpression::diff_t(v, variable_ids),
            MatrixExpression::Inv(v) => MatrixExpression::diff_inv(v, variable_ids),
            MatrixExpression::Det(v) => MatrixExpression::diff_det(v, variable_ids),
        }
    }
}
