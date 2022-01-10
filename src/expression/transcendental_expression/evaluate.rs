use crate::{Expression, TranscendentalExpression, Value};
use std::collections::HashMap;

impl TranscendentalExpression {
    pub fn evaluate(&self, values: &HashMap<&str, Value>) -> Expression {
        match self {
            TranscendentalExpression::Abs(arg) => arg.evaluate(values).abs(),
            TranscendentalExpression::Pow(base, exponent) => base
                .evaluate(values)
                .pow_transcendental(exponent.evaluate(values)),
            TranscendentalExpression::Exp(arg) => arg.evaluate(values).exp(),
            TranscendentalExpression::Log(base, antilogarithm) => {
                base.evaluate(values).log(antilogarithm.evaluate(values))
            }
            TranscendentalExpression::Ln(arg) => arg.evaluate(values).ln(),
            TranscendentalExpression::Sin(arg) => arg.evaluate(values).sin(),
            TranscendentalExpression::Cos(arg) => arg.evaluate(values).cos(),
            TranscendentalExpression::Tan(arg) => arg.evaluate(values).tan(),
        }
    }
}
