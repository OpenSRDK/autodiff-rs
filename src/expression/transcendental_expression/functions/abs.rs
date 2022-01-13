use crate::{Expression, TranscendentalExpression};

impl Expression {
    pub fn abs(self) -> Self {
        if let Expression::Constant(v) = self {
            return Expression::Constant(v.abs());
        }

        TranscendentalExpression::Abs(self.into()).into()
    }
}

impl TranscendentalExpression {
    pub(crate) fn rust_code_abs(arg: &Box<Expression>) -> String {
        format!("{}.abs()", arg.rust_code())
    }
}
