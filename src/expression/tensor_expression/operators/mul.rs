use crate::{Expression, TensorExpression};
use std::ops::Mul;

impl Mul<TensorExpression> for Expression {
    type Output = TensorExpression;

    fn mul(self, rhs: TensorExpression) -> Self::Output {
        if let Expression::Constant(vl) = self {
            if let TensorExpression::Constant(vr) = rhs {
                return TensorExpression::Constant(vl * vr);
            }
            if vl == 1.0 {
                return rhs;
            }
        }

        TensorExpression::MulScalarLhs(self.into(), rhs.into())
    }
}

impl Mul<Expression> for TensorExpression {
    type Output = Self;

    fn mul(self, rhs: Expression) -> Self::Output {
        if let TensorExpression::Constant(vl) = &self {
            if let Expression::Constant(vr) = rhs {
                return TensorExpression::Constant(vl.clone() * vr);
            }
            // TODO
        }

        TensorExpression::MulScalarRhs(self.into(), rhs.into())
    }
}

impl Mul<TensorExpression> for f64 {
    type Output = TensorExpression;

    fn mul(self, rhs: TensorExpression) -> Self::Output {
        TensorExpression::MulScalarLhs(Expression::Constant(self).into(), rhs.into())
    }
}

impl Mul<f64> for TensorExpression {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        TensorExpression::MulScalarRhs(self.into(), Expression::Constant(rhs).into())
    }
}

impl TensorExpression {
    pub(crate) fn diff_mul_scalar_lhs(
        symbols: &[&str],
        l: &Box<Expression>,
        r: &Box<TensorExpression>,
    ) -> Vec<TensorExpression> {
        l.differential(symbols)
            .into_iter()
            .zip(r.differential(symbols).into_iter())
            .map(|(li, ri)| li * r.as_ref().clone() + l.as_ref().clone() * ri)
            .collect()
    }

    pub(crate) fn diff_mul_scalar_rhs(
        symbols: &[&str],
        l: &Box<TensorExpression>,
        r: &Box<Expression>,
    ) -> Vec<TensorExpression> {
        l.differential(symbols)
            .into_iter()
            .zip(r.differential(symbols).into_iter())
            .map(|(li, ri)| li * r.as_ref().clone() + l.as_ref().clone() * ri)
            .collect()
    }
}
