use crate::Expression;
use std::ops::Mul;

impl Mul<Expression> for Expression {
    type Output = Self;

    fn mul(self, rhs: Expression) -> Self::Output {
        if let Expression::Constant(vl) = self {
            if let Expression::Constant(vr) = rhs {
                return Expression::Constant(vl * vr);
            }
            if vl == 1.0 {
                return rhs;
            }
        }
        if let Expression::Constant(vr) = rhs {
            if vr == 1.0 {
                return self;
            }
        }

        Expression::Mul(self.into(), rhs.into())
    }
}
