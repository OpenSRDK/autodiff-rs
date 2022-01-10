use crate::Expression;

impl Expression {
    pub fn cos(self) -> Self {
        if let Expression::Constant(v) = self {
            return Expression::Constant(v.cos());
        }

        Expression::Cos(self.into())
    }
}
