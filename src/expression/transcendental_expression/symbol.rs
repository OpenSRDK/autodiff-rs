use crate::TranscendentalExpression;
use std::collections::HashSet;

impl TranscendentalExpression {
    pub fn symbols(&self) -> HashSet<String> {
        match self {
            TranscendentalExpression::Abs(arg) => arg.symbols(),
            TranscendentalExpression::Pow(base, exponential) => base
                .symbols()
                .into_iter()
                .chain(exponential.symbols().into_iter())
                .collect(),
            TranscendentalExpression::Exp(arg) => arg.symbols(),
            TranscendentalExpression::Log(l, antilogarithm) => l
                .symbols()
                .into_iter()
                .chain(antilogarithm.symbols().into_iter())
                .collect(),
            TranscendentalExpression::Ln(arg) => arg.symbols(),
            TranscendentalExpression::Sin(arg) => arg.symbols(),
            TranscendentalExpression::Cos(arg) => arg.symbols(),
            TranscendentalExpression::Tan(arg) => arg.symbols(),
        }
    }
}
