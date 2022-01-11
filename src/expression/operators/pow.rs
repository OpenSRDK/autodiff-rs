use num_rational::Ratio;
use num_traits::ToPrimitive;

use crate::Expression;

impl Expression {
    pub fn powr(self, exponent: Ratio<u32>) -> Self {
        let exponent_float = exponent.to_f64().unwrap_or_default();
        if let Expression::Constant(base) = self {
            return Expression::Constant(base.powf(exponent_float));
        }

        if exponent_float == 0.0 {
            return Expression::Constant(1.0);
        }
        if exponent_float == 1.0 {
            return self;
        }

        Expression::Pow(self.into(), exponent)
    }
}
