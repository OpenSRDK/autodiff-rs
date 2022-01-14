use crate::TranscendentalExpression;
use std::string::ToString;

impl TranscendentalExpression {
    pub fn rust_code(&self) -> String {
        match self {
            TranscendentalExpression::Abs(arg) => TranscendentalExpression::rust_code_abs(arg),
            TranscendentalExpression::Pow(base, exponent) => {
                TranscendentalExpression::rust_code_pow(base, exponent)
            }
            TranscendentalExpression::Exp(arg) => TranscendentalExpression::rust_code_exp(arg),
            TranscendentalExpression::Log(base, antilogarithm) => {
                TranscendentalExpression::rust_code_log(base, antilogarithm)
            }
            TranscendentalExpression::Ln(arg) => TranscendentalExpression::rust_code_ln(arg),
            TranscendentalExpression::Sin(arg) => TranscendentalExpression::rust_code_sin(arg),
            TranscendentalExpression::Cos(arg) => TranscendentalExpression::rust_code_cos(arg),
            TranscendentalExpression::Tan(arg) => TranscendentalExpression::rust_code_tan(arg),
        }
    }
}
