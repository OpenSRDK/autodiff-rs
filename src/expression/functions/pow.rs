use crate::Expression;

impl Expression {
    pub fn pow(self, exponent: Expression) -> Self {
        if let Expression::Constant(v) = self {}

        Expression::Pow(self.into(), exponent.into())
    }
}
