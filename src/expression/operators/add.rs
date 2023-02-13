use crate::{BracketsLevel, ConstantValue, Expression};
use std::{collections::HashMap, ops::Add};

impl Add<Expression> for Expression {
    type Output = Self;

    fn add(self, rhs: Expression) -> Self::Output {
        if !self.is_same_size(&rhs) {
            println!("{:#?}", self);
            println!("{:#?}", rhs);
            panic!("Cannot add expressions of different sizes");
        }
        if let Expression::Constant(vl) = &self {
            if let Expression::Constant(mut vr) = rhs {
                vl.elems()
                    .into_iter()
                    .zip(vr.elems_mut().into_iter())
                    .for_each(|(vl, vr)| *vr = vl + *vr);
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

        Expression::Add(self.into(), rhs.into())
    }
}

impl Add<f64> for Expression {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        self + Expression::Constant(ConstantValue::Scalar(rhs))
    }
}

impl Add<Expression> for f64 {
    type Output = Expression;

    fn add(self, rhs: Expression) -> Self::Output {
        Expression::Constant(ConstantValue::Scalar(self)) + rhs
    }
}

impl Expression {
    pub(crate) fn diff_add(
        l: &Box<Expression>,
        r: &Box<Expression>,
        variable_ids: &[&str],
    ) -> Vec<Expression> {
        l.differential(variable_ids)
            .into_iter()
            .zip(r.differential(variable_ids).into_iter())
            .map(|(li, ri)| li + ri)
            .collect()
    }

    pub(crate) fn tex_code_add(
        l: &Box<Expression>,
        r: &Box<Expression>,
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
