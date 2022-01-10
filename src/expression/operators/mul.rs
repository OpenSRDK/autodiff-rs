use crate::Expression;
use std::ops::Mul;

impl Mul<Expression> for Expression {
    type Output = Self;

    fn mul(self, rhs: Expression) -> Self::Output {
        if let Expression::Constant(vl) = self {
            if let Expression::Constant(vr) = rhs {
                return Expression::Constant(vl * vr);
            }
            if vl == 0.0 {
                return Expression::Constant(0.0);
            }
            if vl == 1.0 {
                return rhs;
            }
        }
        if let Expression::Constant(vr) = rhs {
            if vr == 0.0 {
                return Expression::Constant(0.0);
            }
            if vr == 1.0 {
                return self;
            }
        }

        Expression::Mul(self.into(), rhs.into())
    }
}

impl Mul<f64> for Expression {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        self * Expression::Constant(rhs)
    }
}

impl Mul<Expression> for f64 {
    type Output = Expression;

    fn mul(self, rhs: Expression) -> Self::Output {
        Expression::Constant(self) * rhs
    }
}
