use crate::Expression;

impl Expression {
    pub fn sin(self) -> Self {
        if let Expression::Constant(v) = self {
            return Expression::Constant(v.sin());
        }

        Expression::Sin(self.into())
    }
}
