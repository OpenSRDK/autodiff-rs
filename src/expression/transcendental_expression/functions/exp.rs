use std::collections::HashMap;

use crate::{Expression, ExpressionArray, TranscendentalExpression};

impl Expression {
    pub fn exp(self) -> Self {
        if let Expression::PartialVariable(v) = &self {
            return Expression::PartialVariable(ExpressionArray::from_factory(
                v.sizes().to_vec(),
                |indices| TranscendentalExpression::Exp(v[indices].clone().into()).into(),
            ));
        }
        if let Expression::Constant(mut v) = self {
            v.elems_mut().into_iter().for_each(|v| *v = v.exp());
            return v.into();
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
