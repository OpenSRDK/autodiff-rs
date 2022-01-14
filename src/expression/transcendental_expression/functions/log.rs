use crate::{Expression, TranscendentalExpression};
use num_traits::ToPrimitive;

impl Expression {
    pub fn log(self, antilogarithm: Expression) -> Self {
        if let Expression::Constant(base) = self {
            if let Expression::Constant(antilogarithm) = antilogarithm {
                return Expression::Constant(antilogarithm.log(base));
            }
        }
        if let Expression::Mul(l, r) = &self {
            return l.as_ref().clone().log(antilogarithm.clone())
                + r.as_ref().clone().log(antilogarithm);
        }
        if let Expression::Pow(base, exponent) = &self {
            return exponent.to_f64().unwrap_or_default()
                * base.as_ref().clone().log(antilogarithm);
        }
        if let Expression::Transcendental(v) = &self {
            match v.as_ref() {
                TranscendentalExpression::Pow(base, exponent) => {
                    return exponent.as_ref().clone() * base.as_ref().clone().log(antilogarithm);
                }
                _ => {}
            }
        }

        TranscendentalExpression::Log(self.into(), antilogarithm.into()).into()
    }
}

impl TranscendentalExpression {
    pub(crate) fn rust_code_log(base: &Box<Expression>, antilogarithm: &Box<Expression>) -> String {
        format!(
            "{}.log({})",
            antilogarithm._rust_code(true),
            base._rust_code(false)
        )
    }
}
