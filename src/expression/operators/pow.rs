use crate::Expression;
use num_rational::Ratio;
use num_traits::cast::ToPrimitive;

impl Expression {
    pub fn powr(self, exponent: Ratio<u32>) -> Self {
        let exponent_float = exponent.to_f64().unwrap_or_default();
        if let Expression::Constant(base) = self {
            return Expression::Constant(base.powf(exponent_float));
        }

        if exponent_float == 0.0 {
            return Expression::Constant(1.0);
        }
        if exponent_float == 1.0 {
            return self;
        }

        Expression::Pow(self.into(), exponent)
    }
}

impl Expression {
    pub(crate) fn diff_powr(
        base: &Box<Expression>,
        exponent: &Ratio<u32>,
        symbols: &[&str],
    ) -> Vec<Expression> {
        base.differential(symbols)
            .into_iter()
            .map(|b| {
                Expression::Constant(exponent.to_f64().unwrap_or_default())
                    * base.as_ref().clone().powr(exponent - 1)
                    * b
            })
            .collect()
    }

    pub(crate) fn rust_code_powr(base: &Box<Expression>, exponent: &Ratio<u32>) -> String {
        let exponent = exponent.to_f64().unwrap_or_default().to_string();
        format!(
            "{}.{}({})",
            base.as_ref()._rust_code(true),
            if exponent.contains('.') {
                "powf"
            } else {
                "powi"
            },
            exponent
        )
    }
}
