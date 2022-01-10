use crate::{Expression, TranscendentalExpression};

impl Expression {
    pub fn abs(self) -> Self {
        if let Expression::Constant(v) = self {
            return Expression::Constant(v.abs());
        }

        TranscendentalExpression::Abs(self.into()).into()
    }
}
