use crate::MatrixExpression;

impl MatrixExpression {
    pub fn t(self) -> MatrixExpression {
        if let MatrixExpression::Constant(v) = self {
            return MatrixExpression::Constant(v.t());
        }

        MatrixExpression::T(self.into())
    }
}
