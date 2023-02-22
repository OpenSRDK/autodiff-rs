use crate::{BracketsLevel, ConstantValue, Expression};
use std::{collections::HashMap, ops::Div};

impl Div<Expression> for Expression {
    type Output = Self;

    fn div(self, rhs: Expression) -> Self::Output {
        if !self.is_same_size(&rhs) {
            panic!("Cannot add expressions of different sizes");
        }
        if let Expression::Constant(vr) = &rhs {
            if vr == &ConstantValue::Scalar(1.0) {
                return self;
            }
            if let Expression::Constant(vl) = self {
                return vl.div(vr).into();
            }
        }

        Expression::Div(self.into(), rhs.into())
    }
}

impl Div<f64> for Expression {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self / Expression::Constant(ConstantValue::Scalar(rhs))
    }
}

impl Div<Expression> for f64 {
    type Output = Expression;

    fn div(self, rhs: Expression) -> Self::Output {
        Expression::Constant(ConstantValue::Scalar(self)) / rhs
    }
}

impl Expression {
    pub(crate) fn diff_div(
        l: &Box<Expression>,
        r: &Box<Expression>,
        variable_ids: &[&str],
    ) -> Vec<Expression> {
        l.differential(variable_ids)
            .into_iter()
            .zip(r.differential(variable_ids).into_iter())
            .map(|(li, ri)| {
                (li * r.as_ref().clone() - l.as_ref().clone() * ri)
                    / r.as_ref().clone().pow(2.0.into())
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
