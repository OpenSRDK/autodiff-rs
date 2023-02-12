use crate::{ConstantValue, Expression, TranscendentalExpression};
use std::collections::HashMap;

impl TranscendentalExpression {
    pub fn assign(self, values: &HashMap<&str, ConstantValue>) -> Expression {
        match self {
            TranscendentalExpression::Abs(arg) => arg.assign(values).abs(),
            TranscendentalExpression::Pow(base, exponent) => {
                base.assign(values).pow(exponent.assign(values))
            }
            TranscendentalExpression::Exp(arg) => arg.assign(values).exp(),
            TranscendentalExpression::Log(base, antilogarithm) => {
                base.assign(values).log(antilogarithm.assign(values))
            }
            TranscendentalExpression::Ln(arg) => arg.assign(values).ln(),
            TranscendentalExpression::Sin(arg) => arg.assign(values).sin(),
            TranscendentalExpression::Cos(arg) => arg.assign(values).cos(),
            TranscendentalExpression::Tan(arg) => arg.assign(values).tan(),
        }
    }
}
