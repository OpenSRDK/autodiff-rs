use crate::MatrixExpression;
use std::ops::Neg;

impl Neg for MatrixExpression {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if let MatrixExpression::Constant(v) = self {
            return MatrixExpression::Constant(-v);
        }
        if let MatrixExpression::Neg(expression) = self {
            return *expression;
        }

        MatrixExpression::Neg(self.into())
    }
}

impl MatrixExpression {
    pub(crate) fn diff_neg(symbols: &[&str], v: &Box<MatrixExpression>) -> Vec<MatrixExpression> {
        v.differential(symbols).into_iter().map(|e| -e).collect()
    }
}
