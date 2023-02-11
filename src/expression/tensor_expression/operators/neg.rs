use crate::{BracketsLevel, TensorExpression};
use std::{collections::HashMap, ops::Neg};

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
    pub(crate) fn diff_neg(v: &Box<TensorExpression>, symbols: &[&str]) -> Vec<TensorExpression> {
        v.differential(symbols).into_iter().map(|e| -e).collect()
    }

    pub(crate) fn tex_code_neg(v: &Box<TensorExpression>, symbols: &HashMap<&str, &str>) -> String {
        format!("{{-{}}}", v._tex_code(symbols, BracketsLevel::ForOperation))
    }
}
