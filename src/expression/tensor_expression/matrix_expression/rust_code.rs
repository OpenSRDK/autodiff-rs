use crate::MatrixExpression;

impl MatrixExpression {
    pub fn rust_code(&self) -> String {
        match self {
            MatrixExpression::Mat(v) => format!("{}.to_mat()", v.rust_code()),
            MatrixExpression::Constant(_) => "const".to_owned(),
            MatrixExpression::T(v) => format!("{}.t()", v.rust_code()),
            MatrixExpression::Inv(v) => format!("{}.inv()", v.rust_code()),
            MatrixExpression::Det(v) => format!("{}.det()", v.rust_code()),
        }
    }
}
