use crate::TensorExpression;
use std::ops::Neg;

impl Neg for TensorExpression {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if let TensorExpression::Constant(v) = self {
            return TensorExpression::Constant(-v);
        }
        if let TensorExpression::Zero = self {
            return TensorExpression::Zero;
        }
        if let TensorExpression::Neg(expression) = self {
            return *expression;
        }

        TensorExpression::Neg(self.into())
    }
}

impl TensorExpression {
    pub(crate) fn diff_neg(symbols: &[&str], v: &Box<TensorExpression>) -> Vec<TensorExpression> {
        v.differential(symbols).into_iter().map(|e| -e).collect()
    }

    pub(crate) fn rust_code_neg(v: &Box<TensorExpression>) -> String {
        format!("-{}", v._rust_code(true))
    }
}
