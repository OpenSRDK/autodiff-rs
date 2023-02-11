use crate::MatrixExpression;

impl MatrixExpression {
    pub fn tex_code(&self) -> String {
        match self {
            MatrixExpression::Mat(v) => v.tex_code(),
            MatrixExpression::Constant(v) => r"\text{{const.}}".to_owned(),
            MatrixExpression::Inv(v) => format!(r"\left\[{}\right\])^{{-1}}", v.tex_code()),
            MatrixExpression::Det(v) => format!(r"\left\|{}\right\|", v.tex_code()),
        }
    }
}
