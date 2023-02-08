use crate::TensorExpression;
use std::ops::Add;

impl Add<TensorExpression> for TensorExpression {
    type Output = Self;

    fn add(self, rhs: TensorExpression) -> Self::Output {
        if let TensorExpression::Constant(vl) = &self {
            if let TensorExpression::Constant(vr) = rhs {
                return Self::Constant(vl + vr);
            }
        }

        TensorExpression::Add(self.into(), rhs.into())
    }
}

impl TensorExpression {
    pub(crate) fn diff_add(
        symbols: &[&str],
        l: &Box<TensorExpression>,
        r: &Box<TensorExpression>,
    ) -> Vec<TensorExpression> {
        l.differential(symbols)
            .into_iter()
            .zip(r.differential(symbols).into_iter())
            .map(|(li, ri)| li + ri)
            .collect()
    }
}
