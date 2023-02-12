use crate::{BracketsLevel, ConstantValue, Expression};
use std::{collections::HashMap, ops::Mul};

impl Mul<Expression> for Expression {
    type Output = Self;

    fn mul(self, rhs: Expression) -> Self::Output {
        // Merge constant
        if let Expression::Constant(vl) = self {
            if let Expression::Constant(vr) = rhs {
                return (vl * vr).into();
            }
            if vl == ConstantValue::Scalar(0.0) {
                return 0.0.into();
            }
            if vl == ConstantValue::Scalar(1.0) {
                return rhs;
            }
        }
        if let Expression::Constant(vr) = rhs {
            if vr == ConstantValue::Scalar(0.0) {
                return 0.0.into();
            }
            if vr == ConstantValue::Scalar(1.0) {
                return self;
            }
        }
        // Merge pow
        if let Expression::Pow(vl, el) = &self {
            if let Expression::Pow(vr, er) = &rhs {
                if vl.as_ref().eq(vr) {
                    return Expression::Pow(vl.clone(), el + er);
                }
            }
            if vl.as_ref().eq(&rhs) {
                return Expression::Pow(vl.clone(), el + 1.0);
            }
        }
        if let Expression::Pow(vr, er) = &rhs {
            if let Expression::Pow(vl, el) = &self {
                if vr.as_ref().eq(vl) {
                    return Expression::Pow(vr.clone(), el + er);
                }
            }
            if vr.as_ref().eq(&self) {
                return Expression::Pow(vr.clone(), er + 1.0);
            }
        }

        Expression::Mul(self.into(), rhs.into())
    }
}

impl Mul<Expression> for f64 {
    type Output = Expression;

    fn mul(self, rhs: Expression) -> Self::Output {
        Expression::Constant(ConstantValue::Scalar(self)) * rhs
    }
}

impl Mul<f64> for Expression {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        self * Expression::Constant(ConstantValue::Scalar(rhs))
    }
}

impl Expression {
    pub(crate) fn diff_mul(
        l: &Box<Expression>,
        r: &Box<Expression>,
        symbols: &[&str],
    ) -> Vec<Expression> {
        l.differential(symbols)
            .into_iter()
            .zip(r.differential(symbols).into_iter())
            .map(|(li, ri)| li * r.as_ref().clone() + l.as_ref().clone() * ri)
            .collect()
    }

    pub(crate) fn tex_code_mul(
        l: &Box<Expression>,
        r: &Box<Expression>,
        symbols: &HashMap<&str, &str>,
        brackets_level: BracketsLevel,
    ) -> String {
        let inner = format!(
            r"{{{} \times {}}}",
            l._tex_code(symbols, BracketsLevel::ForMul),
            r._tex_code(symbols, BracketsLevel::ForMul)
        );

        match brackets_level {
            BracketsLevel::None | BracketsLevel::ForMul => inner,
            BracketsLevel::ForDiv | BracketsLevel::ForOperation => {
                format!(r"\left({}\right)", inner)
            }
        }
    }
}
