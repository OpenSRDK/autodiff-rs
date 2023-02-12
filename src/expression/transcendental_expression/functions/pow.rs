use std::collections::HashMap;

use crate::{BracketsLevel, ConstantValue, Expression, TranscendentalExpression};

impl Expression {
    pub fn pow(self, exponent: Expression) -> Self {
        if let Expression::Constant(exponent) = exponent {
            if exponent == ConstantValue::Scalar(0.0) {
                return 1.0.into();
            }
            if exponent == ConstantValue::Scalar(1.0) {
                return self;
            }

            if let Expression::Constant(base) = self {
                return Expression::Constant(base.pow(exponent));
            }
        }

        TranscendentalExpression::Pow(self.into(), exponent.into()).into()
    }
}

impl TranscendentalExpression {
    pub(crate) fn tex_code_pow(
        base: &Box<Expression>,
        exponent: &Box<Expression>,
        symbols: &HashMap<&str, &str>,
    ) -> String {
        format!(
            "{}^{}",
            base._tex_code(symbols, BracketsLevel::ForOperation),
            exponent._tex_code(symbols, BracketsLevel::ForOperation)
        )
    }
}
