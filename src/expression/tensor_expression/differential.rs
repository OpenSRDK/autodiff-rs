use crate::TensorExpression;

impl TensorExpression {
    pub fn differential(&self, symbols: &[&str]) -> Vec<TensorExpression> {
        match self {
            TensorExpression::Symbol(symbol) => TensorExpression::diff_symbol(symbols, symbol),
            TensorExpression::Constant(_) | TensorExpression::Zero | TensorExpression::Unit => {
                vec![TensorExpression::Zero; symbols.len()]
            }
            TensorExpression::Add(l, r) => TensorExpression::diff_add(symbols, l, r),
            TensorExpression::Sub(l, r) => TensorExpression::diff_sub(symbols, l, r),
            TensorExpression::MulScalarLhs(l, r) => {
                TensorExpression::diff_mul_scalar_lhs(symbols, l, r)
            }
            TensorExpression::MulScalarRhs(l, r) => {
                TensorExpression::diff_mul_scalar_rhs(symbols, l, r)
            }
            TensorExpression::Neg(v) => TensorExpression::diff_neg(symbols, v),
            TensorExpression::InnerProd {
                lhs,
                rhs,
                level_pairs,
            } => {
                todo!()
            }
            TensorExpression::Det(_) => todo!(),
        }
    }
}
