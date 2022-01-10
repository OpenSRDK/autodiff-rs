use crate::{Expression, MatrixExpression};

impl MatrixExpression {
    pub fn as_scalar(self) -> Expression {
        Expression::MatrixScalar(self.into())
    }
}
