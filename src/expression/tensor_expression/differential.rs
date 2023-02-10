use crate::TensorExpression;

impl TensorExpression {
    pub fn differential(&self, symbols: &[&str]) -> Vec<TensorExpression> {
        match self {
            TensorExpression::Symbol(symbol, rank) => {
                TensorExpression::diff_symbol(symbols, symbol, *rank)
            }
            TensorExpression::Constant(_) | TensorExpression::Zero => {
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
            TensorExpression::KroneckerDeltas(_) => {
                vec![TensorExpression::Zero; symbols.len()]
            }
            TensorExpression::InnerProd {
                v,
                rank_combinations,
            } => TensorExpression::diff_inner_prod(symbols, v, rank_combinations),
        }
    }
}
