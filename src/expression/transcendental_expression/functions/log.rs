use crate::{Expression, TranscendentalExpression};

impl Expression {
    pub fn log(self, antilogarithm: Expression) -> Self {
        if let Expression::Constant(base) = self {
            if let Expression::Constant(antilogarithm) = antilogarithm {
                return Expression::Constant(antilogarithm.log(base));
            }
        }

        TranscendentalExpression::Log(self.into(), antilogarithm.into()).into()
    }
}

impl TranscendentalExpression {
    pub(crate) fn rust_code_log(base: &Box<Expression>, antilogarithm: &Box<Expression>) -> String {
        format!("{}.log({})", antilogarithm.rust_code(), base.rust_code())
    }
}
