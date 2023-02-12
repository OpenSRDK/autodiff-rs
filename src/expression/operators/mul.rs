use crate::{BracketsLevel, ConstantValue, Expression, TranscendentalExpression};
use std::{collections::HashMap, ops::Mul};

impl Mul<Expression> for Expression {
    type Output = Self;

    fn mul(self, rhs: Expression) -> Self::Output {
        if !self.is_same_size(&rhs) {
            panic!("Cannot add expressions of different sizes");
        }
        // Merge constant
        if let Expression::Constant(vl) = &self {
            if let Expression::Constant(mut vr) = rhs {
                vl.elems()
                    .into_iter()
                    .zip(vr.elems_mut().into_iter())
                    .for_each(|(vl, vr)| *vr = vl * *vr);
                return vr.into();
            }
            if vl == &ConstantValue::Scalar(0.0) {
                return 0.0.into();
            }
            if vl == &ConstantValue::Scalar(1.0) {
                return rhs;
            }
        }
        if let Expression::Constant(vr) = &rhs {
            if vr == &ConstantValue::Scalar(0.0) {
                return 0.0.into();
            }
            if vr == &ConstantValue::Scalar(1.0) {
                return self;
            }
        }
        // Merge pow
        if let Expression::Transcendental(vl) = &self {
            if let TranscendentalExpression::Pow(vl, el) = vl.as_ref() {
                if let Expression::Transcendental(vr) = &rhs {
                    if let TranscendentalExpression::Pow(vr, er) = vr.as_ref() {
                        if vl.as_ref() == vr.as_ref() {
                            return vl.clone().pow(*el.clone() + *er.clone());
                        }
                    }
                }
                if vl.as_ref() == &rhs {
                    let one: Expression = 1.0.into();
                    return vl.clone().pow(*el.clone() + one);
                }
            }
        }
        if let Expression::Transcendental(vr) = &rhs {
            if let TranscendentalExpression::Pow(vr, er) = vr.as_ref() {
                if vr.as_ref() == &self {
                    let one: Expression = 1.0.into();
                    return vr.clone().pow(*er.clone() + one);
                }
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
