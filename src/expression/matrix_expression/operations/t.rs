use crate::MatrixExpression;

impl MatrixExpression {
    pub fn t(self) -> MatrixExpression {
        if let MatrixExpression::Constant(v) = self {
            return MatrixExpression::Constant(v.t());
        }

        MatrixExpression::T(self.into())
    }
}

impl MatrixExpression {
    pub(crate) fn diff_t(symbols: &[&str], v: &Box<MatrixExpression>) -> Vec<MatrixExpression> {
        v.differential(symbols).into_iter().map(|e| e.t()).collect()
    }
}
