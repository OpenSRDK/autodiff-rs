use crate::{Expression, TranscendentalExpression};

impl Expression {
    pub fn exp(self) -> Self {
        if let Expression::Constant(v) = self {
            return Expression::Constant(v.exp());
        }

        TranscendentalExpression::Exp(self.into()).into()
    }
}
