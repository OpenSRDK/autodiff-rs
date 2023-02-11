use crate::TensorExpression;

impl TensorExpression {
    pub fn differential(&self, symbols: &[&str]) -> Vec<TensorExpression> {
        match self {
            TensorExpression::Symbol(symbol, sizes) => {
                TensorExpression::diff_symbol(symbol, sizes, symbols)
            }
            TensorExpression::Constant(_) | TensorExpression::Zero => {
                vec![TensorExpression::Zero; symbols.len()]
            }

            TensorExpression::Add(l, r) => TensorExpression::diff_add(l, r, symbols),
            TensorExpression::Sub(l, r) => TensorExpression::diff_sub(l, r, symbols),
            TensorExpression::MulScalarLhs(l, r) => {
                TensorExpression::diff_mul_scalar_lhs(l, r, symbols)
            }
            TensorExpression::MulScalarRhs(l, r) => {
                TensorExpression::diff_mul_scalar_rhs(symbols, l, r)
            }
            TensorExpression::Neg(v) => TensorExpression::diff_neg(v, symbols),
            TensorExpression::KroneckerDeltas(_) => {
                vec![TensorExpression::Zero; symbols.len()]
            }
            TensorExpression::InnerProd {
                terms,
                rank_combinations,
            } => TensorExpression::diff_inner_prod(symbols, terms, rank_combinations),
            TensorExpression::Matrix(m) => m.differential(symbols),
        }
    }
}
