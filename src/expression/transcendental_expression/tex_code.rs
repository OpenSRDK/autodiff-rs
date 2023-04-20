use crate::{BracketsLevel, TranscendentalExpression};
use std::collections::HashMap;

impl TranscendentalExpression {
    pub(crate) fn _tex_code(
        &self,
        variables: &HashMap<&str, &str>,
        _brackets_level: BracketsLevel,
    ) -> String {
        match self {
            TranscendentalExpression::Abs(arg) => {
                TranscendentalExpression::tex_code_abs(arg, variables)
            }
            TranscendentalExpression::Pow(base, exponent) => {
                TranscendentalExpression::tex_code_pow(base, exponent, variables)
            }
            TranscendentalExpression::Exp(arg) => {
                TranscendentalExpression::tex_code_exp(arg, variables)
            }
            TranscendentalExpression::Log(base, antilogarithm) => {
                TranscendentalExpression::tex_code_log(base, antilogarithm, variables)
            }
            TranscendentalExpression::Ln(arg) => {
                TranscendentalExpression::tex_code_ln(arg, variables)
            }
            TranscendentalExpression::Sin(arg) => {
                TranscendentalExpression::tex_code_sin(arg, variables)
            }
            TranscendentalExpression::Cos(arg) => {
                TranscendentalExpression::tex_code_cos(arg, variables)
            }
            TranscendentalExpression::Tan(arg) => {
                TranscendentalExpression::tex_code_tan(arg, variables)
            }
        }
    }

    pub fn tex_code(&self, symbols: &HashMap<&str, &str>) -> String {
        self._tex_code(symbols, BracketsLevel::None)
    }
}

#[cfg(test)]
mod tests {
    use crate::new_variable_tensor;
    use std::collections::HashMap;

    #[test]
    fn it_works() {
        let x = new_variable_tensor("x".to_string(), vec![]);
        let mu = new_variable_tensor("mu".to_string(), vec![]);
        let sigma = new_variable_tensor("sigma".to_string(), vec![]);
        let expression = x * mu / sigma;
        let abs = expression.clone().abs();
        let pow = expression.clone().pow(2.0.into());
        let exp = expression.clone().exp();
        let log = expression.clone().log(2.0.into());
        let ln = expression.clone().ln();
        let sin = expression.clone().sin();
        let cos = expression.clone().cos();
        let tan = expression.clone().tan();
        let tex_symbols: Vec<_> = vec![("x", "x"), ("mu", r"\mu"), ("sigma", r"\sigma")]
            .into_iter()
            .collect();
        let tex_symbols: HashMap<_, _> = tex_symbols.into_iter().collect();

        assert_eq!(
            expression.tex_code(&tex_symbols),
            r"{\left({{x} \times {\mu}}\right) / {\sigma}}"
        );
        assert_eq!(
            abs.tex_code(&tex_symbols),
            r"\left|{\left({{x} \times {\mu}}\right) / {\sigma}}\right|"
        );
        assert_eq!(
            pow.tex_code(&tex_symbols),
            r"\left({\left({{x} \times {\mu}}\right) / {\sigma}}\right)^\text{const.}"
        );
        assert_eq!(
            exp.tex_code(&tex_symbols),
            r"\exp{\left({\left({{x} \times {\mu}}\right) / {\sigma}}\right)}"
        );
        assert_eq!(
            log.tex_code(&tex_symbols),
            r"\log_{\left({\left({{x} \times {\mu}}\right) / {\sigma}}\right)}{\text{const.}}"
        );
        assert_eq!(
            ln.tex_code(&tex_symbols),
            r"\ln{\left({\left({{x} \times {\mu}}\right) / {\sigma}}\right)}"
        );
        assert_eq!(
            sin.tex_code(&tex_symbols),
            r"\sin\left({\left({{x} \times {\mu}}\right) / {\sigma}}\right)"
        );
        assert_eq!(
            cos.tex_code(&tex_symbols),
            r"\cos\left({\left({{x} \times {\mu}}\right) / {\sigma}}\right)"
        );
        assert_eq!(
            tan.tex_code(&tex_symbols),
            r"\tan\left({\left({{x} \times {\mu}}\right) / {\sigma}}\right)"
        );
    }
}
