use std::collections::HashMap;

use crate::{BracketsLevel, Expression, TranscendentalExpression};

impl Expression {
    pub fn log(self, antilogarithm: Expression) -> Self {
        if let Expression::Constant(base) = &self {
            if let Expression::Constant(antilogarithm) = antilogarithm {
                return antilogarithm.into_scalar().log(base.into_scalar()).into();
            }
        }
        if let Expression::Mul(l, r) = &self {
            return l.as_ref().clone().log(antilogarithm.clone())
                + r.as_ref().clone().log(antilogarithm);
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
    pub(crate) fn tex_code_log(
        base: &Box<Expression>,
        antilogarithm: &Box<Expression>,
        symbols: &HashMap<&str, &str>,
    ) -> String {
        format!(
            "\\log_{{{}}}{{{}}}",
            base._tex_code(symbols, BracketsLevel::ForOperation),
            antilogarithm._tex_code(symbols, BracketsLevel::ForOperation)
        )
    }
}

#[cfg(test)]
mod tests {

    use crate::{new_variable, ConstantValue, Expression};

    #[test]
    fn it_works() {
        let id1 = "x";
        let id2 = "y";

        let va1 = new_variable(id1.to_string());
        let va2 = new_variable(id2.to_string());
        let tex_symbols = vec![("x", "x"), ("y", "y")].into_iter().collect();

        let va_log = va1
            .clone()
            .log(Expression::Constant(ConstantValue::Scalar(10.0)));
        let tex_va_log = va_log.tex_code(&tex_symbols);
        println!("{}", tex_va_log);

        let base = Expression::Constant(ConstantValue::Scalar(10.0));

        let va12_log = base.clone().log(Expression::Mul(
            Box::new(va1.clone()),
            Box::new(va2.clone()),
        ));
        let tex_va12_log = va12_log.tex_code(&tex_symbols);
        println!("{}", tex_va12_log);
    }
}
