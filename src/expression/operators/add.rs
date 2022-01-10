use crate::Expression;
use std::ops::Add;

impl Add<Expression> for Expression {
    type Output = Self;

    fn add(self, rhs: Expression) -> Self::Output {
        if let Expression::Constant(vl) = self {
            if let Expression::Constant(vr) = rhs {
                return Expression::Constant(vl + vr);
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

        Expression::Add(self.into(), rhs.into())
    }
}
