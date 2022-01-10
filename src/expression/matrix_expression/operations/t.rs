use crate::MatrixExpression;

impl MatrixExpression {
    pub fn t(self) -> MatrixExpression {
        MatrixExpression::T(self.into())
    }
}
