use crate::{BracketsLevel, ConstantValue, Expression};
use std::{collections::HashMap, ops::Sub};

impl Sub<Expression> for Expression {
    type Output = Self;

    fn sub(self, rhs: Expression) -> Self::Output {
        if !self.is_same_size(&rhs) {
            panic!("Cannot add expressions of different sizes");
        }
        if let Expression::Constant(vl) = &self {
            if let Expression::Constant(mut vr) = rhs {
                vl.elems()
                    .into_iter()
                    .zip(vr.elems_mut().into_iter())
                    .for_each(|(vl, vr)| *vr = vl - *vr);
                return vr.into();
            }
            if vl == &ConstantValue::Scalar(0.0) {
                return rhs;
            }
        }
        if let Expression::Constant(vr) = &rhs {
            if vr == &ConstantValue::Scalar(0.0) {
                return self;
            }
        }
        Self::Sub(self.into(), rhs.into())
    }
}

impl Sub<f64> for Expression {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        self - Expression::Constant(ConstantValue::Scalar(rhs))
    }
}

impl Sub<Expression> for f64 {
    type Output = Expression;

    fn sub(self, rhs: Expression) -> Self::Output {
        Expression::Constant(ConstantValue::Scalar(self)) - rhs
    }
}

impl Expression {
    pub(crate) fn diff_sub(
        l: &Box<Expression>,
        r: &Box<Expression>,
        symbols: &[&str],
    ) -> Vec<Expression> {
        l.differential(symbols)
            .into_iter()
            .zip(r.differential(symbols).into_iter())
            .map(|(li, ri)| li - ri)
            .collect()
    }

    pub(crate) fn tex_code_sub(
        l: &Box<Expression>,
        r: &Box<Expression>,
        symbols: &HashMap<&str, &str>,
        brackets_level: BracketsLevel,
    ) -> String {
        let inner = format!(
            "{{{} - {}}}",
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
