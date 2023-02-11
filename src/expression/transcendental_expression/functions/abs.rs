use std::collections::HashMap;

use crate::{BracketsLevel, Expression, TranscendentalExpression};

impl Expression {
    pub fn abs(self) -> Self {
        if let Expression::Constant(v) = self {
            return Expression::Constant(v.abs());
        }

        TranscendentalExpression::Abs(self.into()).into()
    }
}

impl TranscendentalExpression {
    pub(crate) fn tex_code_abs(arg: &Box<Expression>, symbols: &HashMap<&str, &str>) -> String {
        format!(
            r"\left|{}\right|",
            arg._tex_code(symbols, BracketsLevel::None)
        )
    }
}
