use crate::Expression;
use std::ops::Sub;

impl Sub<Expression> for Expression {
    type Output = Self;

    fn sub(self, rhs: Expression) -> Self::Output {
        if let Expression::Constant(vl) = self {
            if let Expression::Constant(vr) = rhs {
                return Self::Constant(vl - vr);
            }
            if vl == 0.0 {
                return rhs;
            }
        }
        if let Expression::Constant(vr) = rhs {
            if vr == 0.0 {
                return self;
            }
        }
        Self::Sub(self.into(), rhs.into())
    }
}
