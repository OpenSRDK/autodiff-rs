use crate::TensorExpression;
use std::ops::Add;

impl Add<TensorExpression> for TensorExpression {
    type Output = Self;

    fn add(self, rhs: TensorExpression) -> Self::Output {
        if let TensorExpression::Constant(vl) = &self {
            if let TensorExpression::Constant(vr) = rhs {
                return Self::Constant(vl + vr);
            }
        }
        // Merge Zero
        if let TensorExpression::Zero = &self {
            return rhs;
        }
        if let TensorExpression::Zero = &rhs {
            return self;
        }

        TensorExpression::Add(self.into(), rhs.into())
    }
}

impl TensorExpression {
    pub(crate) fn diff_add(
        symbols: &[&str],
        l: &Box<TensorExpression>,
        r: &Box<TensorExpression>,
    ) -> Vec<TensorExpression> {
        l.differential(symbols)
            .into_iter()
            .zip(r.differential(symbols).into_iter())
            .map(|(li, ri)| li + ri)
            .collect()
    }

    pub(crate) fn rust_code_add(
        l: &Box<TensorExpression>,
        r: &Box<TensorExpression>,
        parentheses: bool,
    ) -> String {
        if parentheses {
            format!("({} + {})", l._rust_code(true), r._rust_code(true))
        } else {
            format!("{} + {}", l._rust_code(true), r._rust_code(true))
        }
    }
}
