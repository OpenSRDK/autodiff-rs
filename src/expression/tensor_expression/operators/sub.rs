use crate::TensorExpression;
use std::ops::Sub;

impl Sub<TensorExpression> for TensorExpression {
    type Output = Self;

    fn sub(self, rhs: TensorExpression) -> Self::Output {
        if let TensorExpression::Constant(vl) = &self {
            if let TensorExpression::Constant(vr) = rhs {
                return Self::Constant(vl - vr);
            }
        }

        Self::Sub(self.into(), rhs.into())
    }
}

impl TensorExpression {
    pub(crate) fn diff_sub(
        symbols: &[&str],
        l: &Box<TensorExpression>,
        r: &Box<TensorExpression>,
    ) -> Vec<TensorExpression> {
        l.differential(symbols)
            .into_iter()
            .zip(r.differential(symbols).into_iter())
            .map(|(li, ri)| li - ri)
            .collect()
    }
}
