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
