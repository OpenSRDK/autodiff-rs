use crate::MatrixExpression;

impl MatrixExpression {
    pub fn solve(self, rhs: MatrixExpression) -> MatrixExpression {
        MatrixExpression::Solve(self.into(), rhs.into())
    }
}
