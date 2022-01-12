use crate::MatrixExpression;

impl MatrixExpression {
    pub fn pow(self, exponent: i32) -> MatrixExpression {
        MatrixExpression::Pow(self.into(), exponent)
    }
}

impl MatrixExpression {
    pub(crate) fn diff_pow(
        symbols: &[&str],
        base: &Box<MatrixExpression>,
        exponent: &i32,
    ) -> Vec<MatrixExpression> {
        base.differential(symbols)
            .into_iter()
            .map(|b| *exponent as f64 * base.as_ref().clone().pow(*exponent - 1) * b)
            .collect()
    }
}
