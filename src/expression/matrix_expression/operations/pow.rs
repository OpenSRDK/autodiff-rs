use crate::{Expression, MatrixExpression};

impl MatrixExpression {
    pub fn pow(self, exponent: Expression) -> MatrixExpression {
        MatrixExpression::Pow(self.into(), exponent.into())
    }
}
