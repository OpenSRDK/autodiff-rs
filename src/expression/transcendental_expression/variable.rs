use crate::TranscendentalExpression;
use std::collections::HashSet;

impl TranscendentalExpression {
    pub fn variable_ids(&self) -> HashSet<&str> {
        match self {
            TranscendentalExpression::Abs(arg) => arg.variable_ids(),
            TranscendentalExpression::Pow(base, exponential) => base
                .variable_ids()
                .into_iter()
                .chain(exponential.variable_ids().into_iter())
                .collect(),
            TranscendentalExpression::Exp(arg) => arg.variable_ids(),
            TranscendentalExpression::Log(l, antilogarithm) => l
                .variable_ids()
                .into_iter()
                .chain(antilogarithm.variable_ids().into_iter())
                .collect(),
            TranscendentalExpression::Ln(arg) => arg.variable_ids(),
            TranscendentalExpression::Sin(arg) => arg.variable_ids(),
            TranscendentalExpression::Cos(arg) => arg.variable_ids(),
            TranscendentalExpression::Tan(arg) => arg.variable_ids(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::new_variable;
    use std::collections::HashSet;

    #[test]
    fn it_works() {
        let id = "theta";
        let va = new_variable(id.to_string());

        let abs = va.clone().abs();
        let abs_ids = abs.variable_ids();
        let pow = va.clone().pow(2.0.into());
        let pow_ids = pow.variable_ids();
        let exp = va.clone().exp();
        let exp_ids = exp.variable_ids();
        let log = va.clone().log(2.0.into());
        let log_ids = log.variable_ids();
        let ln = va.clone().ln();
        let ln_ids = ln.variable_ids();
        let sin = va.clone().sin();
        let sin_ids = sin.variable_ids();
        let cos = va.clone().cos();
        let cos_ids = cos.variable_ids();
        let tan = va.clone().tan();
        let tan_ids = tan.variable_ids();

        let mut expected_ids = HashSet::new();
        expected_ids.insert(id);

        assert_eq!(abs_ids, expected_ids);
        assert_eq!(pow_ids, expected_ids);
        assert_eq!(exp_ids, expected_ids);
        assert_eq!(log_ids, expected_ids);
        assert_eq!(ln_ids, expected_ids);
        assert_eq!(sin_ids, expected_ids);
        assert_eq!(cos_ids, expected_ids);
        assert_eq!(tan_ids, expected_ids);
    }
}
