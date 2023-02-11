use crate::{BracketsLevel, Expression};
use std::collections::HashMap;

impl Expression {
    pub fn powr(self, exponent: f64) -> Self {
        if let Expression::Constant(base) = self {
            return Expression::Constant(base.powf(exponent));
        }

        if exponent == 0.0 {
            return Expression::Constant(1.0);
        }
        if exponent == 1.0 {
            return self;
        }

        Expression::Pow(self.into(), exponent)
    }
}

impl Expression {
    pub(crate) fn diff_pow(
        base: &Box<Expression>,
        exponent: &f64,
        symbols: &[&str],
    ) -> Vec<Expression> {
        base.differential(symbols)
            .into_iter()
            .map(|b| {
                Expression::Constant(*exponent) * base.as_ref().clone().powr(exponent - 1.0) * b
            })
            .collect()
    }

    pub(crate) fn tex_code_pow(
        base: &Box<Expression>,
        exponent: &f64,
        symbols: &HashMap<&str, &str>,
    ) -> String {
        format!(
            "{{{}^{}}}",
            base._tex_code(symbols, BracketsLevel::ForOperation),
            exponent.to_string()
        )
    }
}
