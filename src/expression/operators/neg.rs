use crate::{BracketsLevel, Expression};
use std::{collections::HashMap, ops::Neg};

impl Neg for Expression {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if let Expression::Constant(v) = self {
            return (-v).into();
        }
        if let Expression::Neg(v) = self {
            return *v;
        }

        Expression::Neg(self.into())
    }
}

impl Expression {
    pub(crate) fn diff_neg(v: &Box<Expression>, symbols: &[&str]) -> Vec<Expression> {
        v.differential(symbols).into_iter().map(|e| -e).collect()
    }

    pub(crate) fn tex_code_neg(v: &Box<Expression>, symbols: &HashMap<&str, &str>) -> String {
        format!("{{-{}}}", v._tex_code(symbols, BracketsLevel::ForOperation))
    }
}
