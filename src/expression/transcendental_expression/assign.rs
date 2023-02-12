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
