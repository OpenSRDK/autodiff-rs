use crate::TensorExpression;
use std::ops::Sub;

impl Sub<TensorExpression> for TensorExpression {
    type Output = Self;

    fn sub(self, rhs: TensorExpression) -> Self::Output {
        if let TensorExpression::Constant(vl) = &self {
            if let TensorExpression::Constant(vr) = rhs {
                return Self::Constant(vl - vr);
            }
        }
        // Merge Zero
        if let TensorExpression::Zero = &self {
            return -rhs;
        }
        if let TensorExpression::Zero = &rhs {
            return self;
        }

        Self::Sub(self.into(), rhs.into())
    }
}

impl TensorExpression {
    pub(crate) fn diff_sub(
        symbols: &[&str],
        l: &Box<TensorExpression>,
        r: &Box<TensorExpression>,
    ) -> Vec<TensorExpression> {
        l.differential(symbols)
            .into_iter()
            .zip(r.differential(symbols).into_iter())
            .map(|(li, ri)| li - ri)
            .collect()
    }

    pub(crate) fn rust_code_sub(
        l: &Box<TensorExpression>,
        r: &Box<TensorExpression>,
        parentheses: bool,
    ) -> String {
        if parentheses {
            format!("({} - {})", l._rust_code(true), r._rust_code(true))
        } else {
            format!("{} - {}", l._rust_code(true), r._rust_code(true))
        }
    }
}
