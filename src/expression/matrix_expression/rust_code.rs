use crate::MatrixExpression;

impl MatrixExpression {
    pub(crate) fn _rust_code(&self, parentheses: bool) -> String {
        match self {
            MatrixExpression::Symbol(symbol) => MatrixExpression::rust_code_symbol(symbol),
            MatrixExpression::Constant(v) => todo!(),
            MatrixExpression::Zero => todo!(),
            MatrixExpression::Unit => todo!(),
            MatrixExpression::Add(l, r) => todo!(),
            MatrixExpression::Sub(l, r) => todo!(),
            MatrixExpression::Mul(l, r) => todo!(),
            MatrixExpression::MulScalar(l, r) => todo!(),
            MatrixExpression::Neg(v) => todo!(),
            MatrixExpression::Pow(base, exponent) => todo!(),
            MatrixExpression::T(v) => todo!(),
            MatrixExpression::Det(v) => todo!(),
        }
    }

    pub fn rust_code(&self) -> String {
        Self::_rust_code(&self, false)
    }
}
