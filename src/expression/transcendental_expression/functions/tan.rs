use std::collections::HashMap;

use crate::{BracketsLevel, Expression, TranscendentalExpression};

impl Expression {
    pub fn tan(self) -> Self {
        if let Expression::Constant(v) = self {
            return Expression::Constant(v.tan());
        }

        TranscendentalExpression::Tan(self.into()).into()
    }
}

impl TranscendentalExpression {
    pub(crate) fn tex_code_tan(arg: &Box<Expression>, symbols: &HashMap<&str, &str>) -> String {
        format!(r"\tan({})", arg._tex_code(symbols, BracketsLevel::None))
    }
}
