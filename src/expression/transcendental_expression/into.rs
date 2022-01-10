use crate::{Expression, TranscendentalExpression};

impl From<TranscendentalExpression> for Expression {
    fn from(t: TranscendentalExpression) -> Self {
        Expression::Transcendental(t.into())
    }
}
