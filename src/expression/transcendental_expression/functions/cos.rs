use std::collections::HashMap;

use crate::{BracketsLevel, Expression, TranscendentalExpression};

impl Expression {
    pub fn cos(self) -> Self {
        if let Expression::Constant(mut v) = self {
            v.elems_mut().into_iter().for_each(|v| *v = v.cos());
            return v.into();
        }

        TranscendentalExpression::Cos(self.into()).into()
    }
}

impl TranscendentalExpression {
    pub(crate) fn tex_code_cos(arg: &Box<Expression>, symbols: &HashMap<&str, &str>) -> String {
        format!(
            r"\cos\left({}\right)",
            arg._tex_code(symbols, BracketsLevel::None)
        )
    }
}
