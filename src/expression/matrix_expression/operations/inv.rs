use crate::MatrixExpression;

impl MatrixExpression {
    pub fn inv(self) -> MatrixExpression {
        MatrixExpression::Inv(self.into())
    }
}
