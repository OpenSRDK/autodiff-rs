use crate::MatrixExpression;

impl MatrixExpression {
    pub fn rust_code(&self) -> String {
        match self {
            MatrixExpression::Mat(v) => format!("{}.to_mat()", v.rust_code()),
            MatrixExpression::Constant(v) => format!("Matrix::from({})", v.rust_code()),
            MatrixExpression::Inv(v) => format!("{}.to_mat().inv()", v.rust_code()),
            MatrixExpression::Det(v) => format!("{}.to_mat().det()", v.rust_code()),
        }
    }
}
