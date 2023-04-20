use crate::{ConstantValue, Expression, TranscendentalExpression};
use std::collections::HashMap;

impl TranscendentalExpression {
    pub fn assign(self, variables: &HashMap<&str, ConstantValue>) -> Expression {
        match self {
            TranscendentalExpression::Abs(arg) => arg.assign(variables).abs(),
            TranscendentalExpression::Pow(base, exponent) => {
                base.assign(variables).pow(exponent.assign(variables))
            }
            TranscendentalExpression::Exp(arg) => arg.assign(variables).exp(),
            TranscendentalExpression::Log(base, antilogarithm) => {
                base.assign(variables).log(antilogarithm.assign(variables))
            }
            TranscendentalExpression::Ln(arg) => arg.assign(variables).ln(),
            TranscendentalExpression::Sin(arg) => arg.assign(variables).sin(),
            TranscendentalExpression::Cos(arg) => arg.assign(variables).cos(),
            TranscendentalExpression::Tan(arg) => arg.assign(variables).tan(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{new_variable, ConstantValue, Expression};

    #[test]
    fn it_works() {
        let id = "mu";
        let va = new_variable(id.to_string());
        let abs = va.clone().abs();
        let pow = va.clone().pow(2.0.into());
        let exp = va.clone().exp();
        let log = va.clone().log(100.0.into());
        let ln = va.clone().ln();
        let sin = va.clone().sin();
        let cos = va.clone().cos();
        let tan = va.clone().tan();

        let variables = vec![(id, ConstantValue::Scalar(10.0))]
            .into_iter()
            .collect();

        assert_eq!(
            Expression::Constant(ConstantValue::Scalar(10.0)),
            abs.assign(&variables)
        );
        assert_eq!(
            Expression::Constant(ConstantValue::Scalar(100.0)),
            pow.assign(&variables)
        );
        assert_eq!(
            Expression::Constant(ConstantValue::Scalar(22026.465794806718)),
            exp.assign(&variables)
        );
        assert_eq!(
            Expression::Constant(ConstantValue::Scalar(2.0)),
            log.assign(&variables)
        );
        assert_eq!(
            Expression::Constant(ConstantValue::Scalar(2.302585092994046)),
            ln.assign(&variables)
        );
        assert_eq!(
            Expression::Constant(ConstantValue::Scalar(-0.5440211108893698)),
            sin.assign(&variables)
        );
        assert_eq!(
            Expression::Constant(ConstantValue::Scalar(-0.8390715290764524)),
            cos.assign(&variables)
        );
        assert_eq!(
            Expression::Constant(ConstantValue::Scalar(0.6483608274590866)),
            tan.assign(&variables)
        );
    }
}
