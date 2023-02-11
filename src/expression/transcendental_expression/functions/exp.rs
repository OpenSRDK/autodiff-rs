use std::collections::HashMap;

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
    pub(crate) fn tex_code_exp(arg: &Box<Expression>, symbols: &HashMap<&str, &str>) -> String {
        format!(
            r"\exp{{{}}}",
            arg._tex_code(symbols, crate::BracketsLevel::ForOperation)
        )
    }
}
