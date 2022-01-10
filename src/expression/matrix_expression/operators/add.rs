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
