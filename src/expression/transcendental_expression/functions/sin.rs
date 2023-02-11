use std::collections::HashMap;

use crate::{BracketsLevel, Expression, TranscendentalExpression};

impl Expression {
    pub fn sin(self) -> Self {
        if let Expression::Constant(v) = self {
            return Expression::Constant(v.sin());
        }

        TranscendentalExpression::Sin(self.into()).into()
    }
}

impl TranscendentalExpression {
    pub(crate) fn tex_code_sin(arg: &Box<Expression>, symbols: &HashMap<&str, &str>) -> String {
        format!(r"\sin({})", arg._tex_code(symbols, BracketsLevel::None))
    }
}
