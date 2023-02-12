use crate::{BracketsLevel, TranscendentalExpression};
use std::collections::HashMap;

impl TranscendentalExpression {
    pub(crate) fn _tex_code(
        &self,
        symbols: &HashMap<&str, &str>,
        _brackets_level: BracketsLevel,
    ) -> String {
        match self {
            TranscendentalExpression::Abs(arg) => {
                TranscendentalExpression::tex_code_abs(arg, symbols)
            }
            TranscendentalExpression::Pow(base, exponent) => {
                TranscendentalExpression::tex_code_pow(base, exponent, symbols)
            }
            TranscendentalExpression::Exp(arg) => {
                TranscendentalExpression::tex_code_exp(arg, symbols)
            }
            TranscendentalExpression::Log(base, antilogarithm) => {
                TranscendentalExpression::tex_code_log(base, antilogarithm, symbols)
            }
            TranscendentalExpression::Ln(arg) => {
                TranscendentalExpression::tex_code_ln(arg, symbols)
            }
            TranscendentalExpression::Sin(arg) => {
                TranscendentalExpression::tex_code_sin(arg, symbols)
            }
            TranscendentalExpression::Cos(arg) => {
                TranscendentalExpression::tex_code_cos(arg, symbols)
            }
            TranscendentalExpression::Tan(arg) => {
                TranscendentalExpression::tex_code_tan(arg, symbols)
            }
        }
    }

    pub fn tex_code(&self, symbols: &HashMap<&str, &str>) -> String {
        self._tex_code(symbols, BracketsLevel::None)
    }
}
