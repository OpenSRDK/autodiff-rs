use crate::{Expression, TranscendentalExpression};

impl Expression {
    pub fn tan(self) -> Self {
        if let Expression::Constant(v) = self {
            return Expression::Constant(v.tan());
        }

        TranscendentalExpression::Tan(self.into()).into()
    }
}

impl TranscendentalExpression {
    pub(crate) fn rust_code_tan(arg: &Box<Expression>) -> String {
        format!("{}.tan()", arg.rust_code())
    }
}
