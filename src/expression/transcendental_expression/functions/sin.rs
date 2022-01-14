use crate::{Expression, TranscendentalExpression};

impl Expression {
    pub fn sin(self) -> Self {
        if let Expression::Constant(v) = self {
            return Expression::Constant(v.sin());
        }

        TranscendentalExpression::Sin(self.into()).into()
    }
}

impl TranscendentalExpression {
    pub(crate) fn rust_code_sin(arg: &Box<Expression>) -> String {
        format!("{}.sin()", arg._rust_code(true))
    }
}
