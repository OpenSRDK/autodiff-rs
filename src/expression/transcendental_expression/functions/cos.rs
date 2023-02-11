use std::collections::HashMap;

use crate::{BracketsLevel, Expression, TranscendentalExpression};

impl Expression {
    pub fn cos(self) -> Self {
        if let Expression::Constant(v) = self {
            return Expression::Constant(v.cos());
        }

        TranscendentalExpression::Cos(self.into()).into()
    }
}

impl TranscendentalExpression {
    pub(crate) fn tex_code_cos(arg: &Box<Expression>, symbols: &HashMap<&str, &str>) -> String {
        format!(r"\cos({})", arg._tex_code(symbols, BracketsLevel::None))
    }
}
