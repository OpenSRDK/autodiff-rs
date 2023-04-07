use std::collections::HashMap;

use crate::{BracketsLevel, ConstantValue, Expression, TranscendentalExpression};

impl Expression {
    pub fn pow(self, exponent: Expression) -> Self {
        if let Expression::Constant(exponent) = &exponent {
            if exponent == &ConstantValue::Scalar(0.0) {
                return 1.0.into();
            }
            if exponent == &ConstantValue::Scalar(1.0) {
                return self;
            }

            if let Expression::Constant(base) = self {
                return base.into_scalar().powf(exponent.into_scalar()).into();
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

        // let va_pow0 = va1
        //     .clone()
        //     .pow(Expression::Constant(ConstantValue::Scalar(0.0)));
        // let tex_pow0 = va_pow0.tex_code(&tex_symbols);
        // assert_eq!(r"1", tex_pow0);

        let va_pow1 = va1
            .clone()
            .pow(Expression::Constant(ConstantValue::Scalar(1.0)));
        let tex_pow1 = va_pow1.tex_code(&tex_symbols);
        assert_eq!(r"{x}", tex_pow1);

        let va_pow = va1.clone().pow(va2.clone());
        let tex_a_pow = va_pow.tex_code(&tex_symbols);

        assert_eq!(r"{x}^{y}", tex_a_pow);
    }
}
