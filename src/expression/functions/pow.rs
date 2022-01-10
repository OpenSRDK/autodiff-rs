use crate::Expression;

impl Expression {
    pub fn pow(self, exponent: Expression) -> Self {
        if let Expression::Constant(exponent) = exponent {
            if exponent == 0.0 {
                return Expression::Constant(1.0);
            }
            if exponent == 1.0 {
                return self;
            }

            if let Expression::Constant(base) = self {
                return Expression::Constant(base.powf(exponent));
            }
        }

        Expression::Pow(self.into(), exponent.into())
    }
}
