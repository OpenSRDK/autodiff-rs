use crate::MatrixExpression;

impl MatrixExpression {
    pub fn differential(&self, symbols: &[&str]) -> Vec<MatrixExpression> {
        match self {
            MatrixExpression::Symbol(symbol) => MatrixExpression::diff_symbol(symbols, symbol),
            MatrixExpression::Constant(_) | MatrixExpression::Zero | MatrixExpression::Unit => {
                vec![MatrixExpression::Zero; symbols.len()]
            }
            MatrixExpression::Add(l, r) => MatrixExpression::diff_add(symbols, l, r),
            MatrixExpression::Sub(l, r) => MatrixExpression::diff_sub(symbols, l, r),
            MatrixExpression::Mul(l, r) => MatrixExpression::diff_mul(symbols, l, r),
            MatrixExpression::MulScalar(l, r) => MatrixExpression::diff_mul_scalar(symbols, l, r),
            MatrixExpression::Neg(v) => MatrixExpression::diff_neg(symbols, v),
            MatrixExpression::Pow(base, exponent) => {
                MatrixExpression::diff_pow(symbols, base, exponent)
            }
            MatrixExpression::T(v) => MatrixExpression::diff_t(symbols, v),
            MatrixExpression::Det(_) => todo!(),
        }
    }
}
