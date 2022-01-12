use crate::{Expression, MatrixExpression};
use std::ops::Mul;

impl Mul<MatrixExpression> for MatrixExpression {
    type Output = Self;

    fn mul(self, rhs: MatrixExpression) -> Self::Output {
        if let MatrixExpression::Constant(vl) = &self {
            if let MatrixExpression::Constant(vr) = rhs {
                return MatrixExpression::Constant(vl * vr);
            }
        }
        // Merge as pow

        MatrixExpression::Mul(self.into(), rhs.into())
    }
}

impl Mul<MatrixExpression> for Expression {
    type Output = MatrixExpression;

    fn mul(self, rhs: MatrixExpression) -> Self::Output {
        if let Expression::Constant(vl) = self {
            if let MatrixExpression::Constant(vr) = rhs {
                return MatrixExpression::Constant(vl * vr);
            }
            if vl == 1.0 {
                return rhs;
            }
        }

        MatrixExpression::MulScalar(self.into(), rhs.into())
    }
}

impl Mul<Expression> for MatrixExpression {
    type Output = Self;

    fn mul(self, rhs: Expression) -> Self::Output {
        if let MatrixExpression::Constant(vl) = &self {
            if let Expression::Constant(vr) = rhs {
                return MatrixExpression::Constant(vl.clone() * vr);
            }
            // TODO
        }

        MatrixExpression::MulScalar(rhs.into(), self.into())
    }
}

impl Mul<MatrixExpression> for f64 {
    type Output = MatrixExpression;

    fn mul(self, rhs: MatrixExpression) -> Self::Output {
        MatrixExpression::MulScalar(Expression::Constant(self).into(), rhs.into())
    }
}

impl Mul<f64> for MatrixExpression {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        MatrixExpression::MulScalar(Expression::Constant(rhs).into(), self.into())
    }
}

impl MatrixExpression {
    pub(crate) fn diff_mul(
        symbols: &[&str],
        l: &Box<MatrixExpression>,
        r: &Box<MatrixExpression>,
    ) -> Vec<MatrixExpression> {
        l.differential(symbols)
            .into_iter()
            .zip(r.differential(symbols).into_iter())
            .map(|(li, ri)| li * r.as_ref().clone() + l.as_ref().clone() * ri)
            .collect()
    }

    pub(crate) fn diff_mul_scalar(
        symbols: &[&str],
        l: &Box<Expression>,
        r: &Box<MatrixExpression>,
    ) -> Vec<MatrixExpression> {
        l.differential(symbols)
            .into_iter()
            .zip(r.differential(symbols).into_iter())
            .map(|(li, ri)| li * r.as_ref().clone() + l.as_ref().clone() * ri)
            .collect()
    }
}
