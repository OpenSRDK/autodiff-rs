use crate::{BracketsLevel, TensorExpression};
use std::{collections::HashMap, ops::Sub};

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
        l: &Box<TensorExpression>,
        r: &Box<TensorExpression>,
        symbols: &[&str],
    ) -> Vec<TensorExpression> {
        l.differential(symbols)
            .into_iter()
            .zip(r.differential(symbols).into_iter())
            .map(|(li, ri)| li - ri)
            .collect()
    }

    pub(crate) fn tex_code_sub(
        l: &Box<TensorExpression>,
        r: &Box<TensorExpression>,
        symbols: &HashMap<&str, &str>,
        brackets_level: BracketsLevel,
    ) -> String {
        let inner = format!(
            "{{{} + {}}}",
            l._tex_code(symbols, BracketsLevel::None),
            r._tex_code(symbols, BracketsLevel::None)
        );

        match brackets_level {
            BracketsLevel::None => inner,
            BracketsLevel::ForMul | BracketsLevel::ForDiv | BracketsLevel::ForOperation => {
                format!(r"\left({}\right)", inner)
            }
        }
    }
}
