use crate::TensorExpression;

impl TensorExpression {
    pub(crate) fn _rust_code(&self, parentheses: bool) -> String {
        match self {
            TensorExpression::Symbol(symbol, _) => TensorExpression::rust_code_symbol(symbol),
            TensorExpression::Constant(v) => todo!(),
            TensorExpression::Zero => "0.0".to_owned(),
            TensorExpression::Add(l, r) => TensorExpression::rust_code_add(l, r, parentheses),
            TensorExpression::Sub(l, r) => TensorExpression::rust_code_sub(l, r, parentheses),
            TensorExpression::MulScalarLhs(l, r) => {
                TensorExpression::rust_code_mul_scalar_lhs(l, r, parentheses)
            }
            TensorExpression::MulScalarRhs(l, r) => {
                TensorExpression::rust_code_mul_scalar_rhs(l, r, parentheses)
            }
            TensorExpression::Neg(v) => TensorExpression::rust_code_neg(v),
            TensorExpression::KroneckerDeltas(rank_pairs) => {
                TensorExpression::rust_code_kronecker_deltas(rank_pairs, parentheses)
            }
            TensorExpression::InnerProd {
                v,
                rank_combinations,
            } => TensorExpression::rust_code_inner_prod(v, rank_combinations, parentheses),
        }
    }

    pub fn rust_code(&self) -> String {
        Self::_rust_code(&self, false)
    }
}
