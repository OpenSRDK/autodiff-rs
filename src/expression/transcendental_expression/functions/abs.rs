use std::collections::HashMap;

use crate::{BracketsLevel, Expression, TranscendentalExpression};

impl Expression {
    pub fn abs(self) -> Self {
        if let Expression::Constant(mut v) = self {
            v.elems_mut().into_iter().for_each(|v| *v = v.abs());
            return v.into();
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
