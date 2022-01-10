use crate::MatrixExpression;

impl MatrixExpression {
    pub fn pow(self, exponent: i32) -> MatrixExpression {
        MatrixExpression::Pow(self.into(), exponent)
    }
}
