use crate::Expression;
use std::ops::Neg;

impl Neg for Expression {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if let Expression::Constant(v) = self {
            return Expression::Constant(-v);
        }
        if let Expression::Neg(v) = self {
            return *v;
        }

        Expression::Neg(self.into())
    }
}

impl Expression {
    pub(crate) fn diff_neg(symbols: &[&str], v: &Box<Expression>) -> Vec<Expression> {
        v.differential(symbols).into_iter().map(|e| -e).collect()
    }

    pub(crate) fn rust_code_neg(v: &Box<Expression>) -> String {
        format!("-{}", v._rust_code(true))
    }
}
