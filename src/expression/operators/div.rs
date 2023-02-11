use crate::{BracketsLevel, Expression};
use std::{collections::HashMap, ops::Div};

impl Div<Expression> for Expression {
    type Output = Self;

    fn div(self, rhs: Expression) -> Self::Output {
        if let Expression::Constant(vr) = rhs {
            if let Expression::Constant(vl) = self {
                return Expression::Constant(vl / vr);
            }
            if vr == 1.0 {
                return self;
            }
        }

        Expression::Div(self.into(), rhs.into())
    }
}

impl Div<f64> for Expression {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self / Expression::Constant(rhs)
    }
}

impl Div<Expression> for f64 {
    type Output = Expression;

    fn div(self, rhs: Expression) -> Self::Output {
        Expression::Constant(self) / rhs
    }
}

impl Expression {
    pub(crate) fn diff_div(
        l: &Box<Expression>,
        r: &Box<Expression>,
        symbols: &[&str],
    ) -> Vec<Expression> {
        l.differential(symbols)
            .into_iter()
            .zip(r.differential(symbols).into_iter())
            .map(|(li, ri)| {
                (li * r.as_ref().clone() - l.as_ref().clone() * ri)
                    / r.as_ref().clone().powr(2.into())
            })
            .collect()
    }

    pub(crate) fn tex_code_div(
        l: &Box<Expression>,
        r: &Box<Expression>,
        symbols: &HashMap<&str, &str>,
        brackets_level: BracketsLevel,
    ) -> String {
        let inner = format!(
            "{{{} / {}}}",
            l._tex_code(symbols, BracketsLevel::ForDiv),
            r._tex_code(symbols, BracketsLevel::ForDiv)
        );

        match brackets_level {
            BracketsLevel::None | BracketsLevel::ForMul => inner,
            BracketsLevel::ForDiv | BracketsLevel::ForOperation => {
                format!(r"\left({}\right)", inner)
            }
        }
    }
}
