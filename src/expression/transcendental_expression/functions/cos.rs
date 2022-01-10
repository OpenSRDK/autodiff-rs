use crate::{Expression, TranscendentalExpression};

impl Expression {
    pub fn cos(self) -> Self {
        if let Expression::Constant(v) = self {
            return Expression::Constant(v.cos());
        }

        TranscendentalExpression::Cos(self.into()).into()
    }
}
