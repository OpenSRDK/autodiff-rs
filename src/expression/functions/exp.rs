use crate::Expression;

impl Expression {
    pub fn exp(self) -> Self {
        if let Expression::Constant(v) = self {
            return Expression::Constant(v.exp());
        }

        Expression::Exp(self.into())
    }
}
