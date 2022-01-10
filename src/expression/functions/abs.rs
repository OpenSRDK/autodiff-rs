use crate::Expression;

impl Expression {
    pub fn abs(self) -> Self {
        if let Expression::Constant(v) = self {
            return Expression::Constant(v.abs());
        }

        Expression::Abs(self.into())
    }
}
