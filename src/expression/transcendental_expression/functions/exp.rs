use crate::{Expression, TranscendentalExpression};

impl Expression {
    pub fn exp(self) -> Self {
        if let Expression::Constant(v) = self {
            return Expression::Constant(v.exp());
        }

        TranscendentalExpression::Exp(self.into()).into()
    }
}

impl TranscendentalExpression {
    pub(crate) fn rust_code_exp(arg: &Box<Expression>) -> String {
        format!("{}.exp()", arg.rust_code())
    }
}
