use crate::MatrixExpression;
use std::ops::Add;

impl Add<MatrixExpression> for MatrixExpression {
    type Output = Self;

    fn add(self, rhs: MatrixExpression) -> Self::Output {
        if let MatrixExpression::Constant(vl) = &self {
            if let MatrixExpression::Constant(vr) = rhs {
                return Self::Constant(vl + vr);
            }
        }

        MatrixExpression::Add(self.into(), rhs.into())
    }
}

impl MatrixExpression {
    pub(crate) fn diff_add(
        symbols: &[&str],
        l: &Box<MatrixExpression>,
        r: &Box<MatrixExpression>,
    ) -> Vec<MatrixExpression> {
        l.differential(symbols)
            .into_iter()
            .zip(r.differential(symbols).into_iter())
            .map(|(li, ri)| li + ri)
            .collect()
    }
}
