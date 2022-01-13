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
        symbols: &[&str],
        base: &Box<Expression>,
        exponent: &Ratio<u32>,
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
        let exponent_float = exponent.to_f64().unwrap_or_default();

        format!("{}.powf({})", base.as_ref().rust_code(), exponent_float)
    }
}
