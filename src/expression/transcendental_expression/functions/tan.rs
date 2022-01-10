use crate::{Expression, TranscendentalExpression};

impl Expression {
    pub fn tan(self) -> Self {
        if let Expression::Constant(v) = self {
            return Expression::Constant(v.tan());
        }

        TranscendentalExpression::Tan(self.into()).into()
    }
}
