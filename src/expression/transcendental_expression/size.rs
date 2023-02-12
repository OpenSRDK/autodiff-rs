use crate::{Size, TranscendentalExpression};

impl TranscendentalExpression {
    pub fn sizes(&self) -> Vec<Size> {
        match self {
            TranscendentalExpression::Abs(arg) => arg.sizes(),
            TranscendentalExpression::Pow(base, exponent) => {
                [base.sizes(), exponent.sizes()].concat()
            }
            TranscendentalExpression::Exp(arg) => arg.sizes(),
            TranscendentalExpression::Log(base, antilogarithm) => {
                [base.sizes(), antilogarithm.sizes()].concat()
            }
            TranscendentalExpression::Ln(arg) => arg.sizes(),
            TranscendentalExpression::Sin(arg) => arg.sizes(),
            TranscendentalExpression::Cos(arg) => arg.sizes(),
            TranscendentalExpression::Tan(arg) => arg.sizes(),
        }
    }
}
