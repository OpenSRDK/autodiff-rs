use crate::Expression;

impl Expression {
    pub fn ln(self) -> Self {
        if let Expression::Constant(v) = self {
            return Expression::Constant(v.ln());
        }

        Expression::Ln(self.into())
    }
}
