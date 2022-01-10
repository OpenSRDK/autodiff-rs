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
        if let MatrixExpression::Inv(expression) = &self {
            return expression.clone().solve(rhs);
        }
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
