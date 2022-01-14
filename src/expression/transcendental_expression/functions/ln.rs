use crate::{Expression, TranscendentalExpression};
use num_traits::ToPrimitive;

impl Expression {
    pub fn ln(self) -> Self {
        if let Expression::Constant(v) = self {
            return Expression::Constant(v.ln());
        }
        if let Expression::Mul(l, r) = &self {
            return l.as_ref().clone().ln() + r.as_ref().clone().ln();
        }
        if let Expression::Pow(base, exponent) = &self {
            return exponent.to_f64().unwrap_or_default() * base.as_ref().clone().ln();
        }
        if let Expression::Transcendental(v) = &self {
            match v.as_ref() {
                TranscendentalExpression::Pow(base, exponent) => {
                    return exponent.as_ref().clone() * base.as_ref().clone().ln();
                }
                TranscendentalExpression::Exp(arg) => {
                    return arg.as_ref().clone().into();
                }
                _ => {}
            }
        }

        TranscendentalExpression::Ln(self.into()).into()
    }
}

impl TranscendentalExpression {
    pub(crate) fn rust_code_ln(arg: &Box<Expression>) -> String {
        format!("{}.ln()", arg._rust_code(true))
    }
}
