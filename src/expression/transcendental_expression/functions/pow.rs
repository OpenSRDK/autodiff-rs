use std::collections::HashMap;

use crate::{BracketsLevel, Expression, TranscendentalExpression};

impl Expression {
    pub fn pow(self, exponent: Expression) -> Self {
        if let Expression::Constant(exponent) = exponent {
            if exponent == 0.0 {
                return Expression::Constant(1.0);
            }
            if exponent == 1.0 {
                return self;
            }

            if let Expression::Constant(base) = self {
                return Expression::Constant(base.powf(exponent));
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
