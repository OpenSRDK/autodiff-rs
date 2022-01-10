use crate::Expression;

impl Expression {
    pub fn tan(self) -> Self {
        if let Expression::Constant(v) = self {
            return Expression::Constant(v.tan());
        }

        Expression::Tan(self.into())
    }
}
