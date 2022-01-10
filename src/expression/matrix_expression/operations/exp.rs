use crate::MatrixExpression;

impl MatrixExpression {
    pub fn exp(self) -> MatrixExpression {
        MatrixExpression::MatrixExp(self.into())
    }
}
