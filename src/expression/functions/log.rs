use crate::Expression;

impl Expression {
    pub fn log(self, antilogarithm: Expression) -> Self {
        if let Expression::Constant(base) = self {
            if let Expression::Constant(antilogarithm) = antilogarithm {
                return Expression::Constant(antilogarithm.log(base));
            }
        }

        Expression::Log(self.into(), antilogarithm.into())
    }
}
