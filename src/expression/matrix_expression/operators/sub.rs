use crate::MatrixExpression;
use std::ops::Sub;

impl Sub<MatrixExpression> for MatrixExpression {
    type Output = Self;

    fn sub(self, rhs: MatrixExpression) -> Self::Output {
        if let MatrixExpression::Constant(vl) = &self {
            if let MatrixExpression::Constant(vr) = rhs {
                return Self::Constant(vl - vr);
            }
        }

        Self::Sub(self.into(), rhs.into())
    }
}

impl MatrixExpression {
    pub(crate) fn diff_sub(
        symbols: &[&str],
        l: &Box<MatrixExpression>,
        r: &Box<MatrixExpression>,
    ) -> Vec<MatrixExpression> {
        l.differential(symbols)
            .into_iter()
            .zip(r.differential(symbols).into_iter())
            .map(|(li, ri)| li - ri)
            .collect()
    }
}
