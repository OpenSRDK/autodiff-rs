use std::collections::HashMap;

use crate::{BracketsLevel, Expression, TranscendentalExpression};

impl Expression {
    pub fn sin(self) -> Self {
        if let Expression::Constant(mut v) = self {
            v.elems_mut().into_iter().for_each(|v| *v = v.sin());
            return v.into();
        }

        TranscendentalExpression::Sin(self.into()).into()
    }
}

impl TranscendentalExpression {
    pub(crate) fn tex_code_sin(arg: &Box<Expression>, symbols: &HashMap<&str, &str>) -> String {
        format!(
            r"\sin\right({}\left)",
            arg._tex_code(symbols, BracketsLevel::None)
        )
    }
}
