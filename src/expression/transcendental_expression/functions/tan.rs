use std::collections::HashMap;

use crate::{BracketsLevel, Expression, TranscendentalExpression};

impl Expression {
    pub fn tan(self) -> Self {
        if let Expression::Constant(mut v) = self {
            v.elems_mut().into_iter().for_each(|v| *v = v.tan());
            return v.into();
        }

        TranscendentalExpression::Tan(self.into()).into()
    }
}

impl TranscendentalExpression {
    pub(crate) fn tex_code_tan(arg: &Box<Expression>, symbols: &HashMap<&str, &str>) -> String {
        format!(
            r"\tan\right({}\left)",
            arg._tex_code(symbols, BracketsLevel::None)
        )
    }
}
