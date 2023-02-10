use crate::TranscendentalExpression;

impl TranscendentalExpression {
    pub fn tex_code(&self) -> String {
        match self {
            TranscendentalExpression::Abs(arg) => format!("\\left|{}\\right|", arg.tex_code()),
            TranscendentalExpression::Pow(base, exponent) => {
                format!("{}^{{{}}}", base.tex_code(), exponent.tex_code())
            }
            TranscendentalExpression::Exp(arg) => format!("\\exp({})", arg.tex_code()),
            TranscendentalExpression::Log(base, antilogarithm) => format!(
                "\\log_{{{}}}({})",
                base.tex_code(),
                antilogarithm.tex_code()
            ),
            TranscendentalExpression::Ln(arg) => format!("\\ln({})", arg.tex_code()),
            TranscendentalExpression::Sin(arg) => format!("\\sin({})", arg.tex_code()),
            TranscendentalExpression::Cos(arg) => format!("\\cos({})", arg.tex_code()),
            TranscendentalExpression::Tan(arg) => format!("\\tan({})", arg.tex_code()),
        }
    }
}
