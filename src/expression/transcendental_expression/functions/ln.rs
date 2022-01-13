use crate::{Expression, TranscendentalExpression};

impl Expression {
    pub fn ln(self) -> Self {
        if let Expression::Constant(v) = self {
            return Expression::Constant(v.ln());
        }

        TranscendentalExpression::Ln(self.into()).into()
    }
}

impl TranscendentalExpression {
    pub(crate) fn rust_code_ln(arg: &Box<Expression>) -> String {
        format!("{}.ln()", arg.rust_code())
    }
}
