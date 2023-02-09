use crate::TensorExpression;

impl TensorExpression {
    pub(crate) fn _rust_code(&self, parentheses: bool) -> String {
        match self {
            TensorExpression::Symbol(symbol, _) => TensorExpression::rust_code_symbol(symbol),
            TensorExpression::Constant(v) => todo!(),
            TensorExpression::Zero => todo!(),
            TensorExpression::Add(l, r) => todo!(),
            TensorExpression::Sub(l, r) => todo!(),
            TensorExpression::MulScalarLhs(l, r) => todo!(),
            TensorExpression::MulScalarRhs(l, r) => todo!(),
            TensorExpression::Neg(v) => todo!(),
            TensorExpression::KroneckerDeltas(level_pairs) => todo!(),
            TensorExpression::InnerProd {
                lhs,
                rhs,
                level_pairs,
            } => TensorExpression::rust_code_inner_prod(lhs, rhs, level_pairs, parentheses),
            TensorExpression::Det(v) => todo!(),
        }
    }

    pub fn rust_code(&self) -> String {
        Self::_rust_code(&self, false)
    }
}
