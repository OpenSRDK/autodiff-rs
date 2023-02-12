use crate::{BracketsLevel, TranscendentalExpression};
use std::collections::HashMap;

impl TranscendentalExpression {
    pub(crate) fn _tex_code(
        &self,
        variables: &HashMap<&str, &str>,
        _brackets_level: BracketsLevel,
    ) -> String {
        match self {
            TranscendentalExpression::Abs(arg) => {
                TranscendentalExpression::tex_code_abs(arg, variables)
            }
            TranscendentalExpression::Pow(base, exponent) => {
                TranscendentalExpression::tex_code_pow(base, exponent, variables)
            }
            TranscendentalExpression::Exp(arg) => {
                TranscendentalExpression::tex_code_exp(arg, variables)
            }
            TranscendentalExpression::Log(base, antilogarithm) => {
                TranscendentalExpression::tex_code_log(base, antilogarithm, variables)
            }
            TranscendentalExpression::Ln(arg) => {
                TranscendentalExpression::tex_code_ln(arg, variables)
            }
            TranscendentalExpression::Sin(arg) => {
                TranscendentalExpression::tex_code_sin(arg, variables)
            }
            TranscendentalExpression::Cos(arg) => {
                TranscendentalExpression::tex_code_cos(arg, variables)
            }
            TranscendentalExpression::Tan(arg) => {
                TranscendentalExpression::tex_code_tan(arg, variables)
            }
        }
    }

    pub fn tex_code(&self, symbols: &HashMap<&str, &str>) -> String {
        self._tex_code(symbols, BracketsLevel::None)
    }
}
