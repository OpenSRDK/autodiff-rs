use crate::Expression;
use std::ops::Mul;

impl Mul<Expression> for Expression {
    type Output = Self;

    fn mul(self, rhs: Expression) -> Self::Output {
        // Merge constant
        if let Expression::Constant(vl) = self {
            if let Expression::Constant(vr) = rhs {
                return Expression::Constant(vl * vr);
            }
            if vl == 0.0 {
                return Expression::Constant(0.0);
            }
            if vl == 1.0 {
                return rhs;
            }
        }
        if let Expression::Constant(vr) = rhs {
            if vr == 0.0 {
                return Expression::Constant(0.0);
            }
            if vr == 1.0 {
                return self;
            }
        }
        // Merge as pow
        if let Expression::Pow(vl, el) = &self {
            if let Expression::Pow(vr, er) = &rhs {
                if vl.as_ref().eq(vr) {
                    return Expression::Pow(vl.clone(), el + er);
                }
            }
            if vl.as_ref().eq(&rhs) {
                return Expression::Pow(vl.clone(), el + 1);
            }
        }
        if let Expression::Pow(vr, er) = &rhs {
            if let Expression::Pow(vl, el) = &self {
                if vr.as_ref().eq(vl) {
                    return Expression::Pow(vr.clone(), el + er);
                }
            }
            if vr.as_ref().eq(&self) {
                return Expression::Pow(vr.clone(), er + 1);
            }
        }

        Expression::Mul(self.into(), rhs.into())
    }
}

impl Mul<Expression> for f64 {
    type Output = Expression;

    fn mul(self, rhs: Expression) -> Self::Output {
        Expression::Constant(self) * rhs
    }
}

impl Mul<f64> for Expression {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        self * Expression::Constant(rhs)
    }
}

impl Expression {
    pub(crate) fn diff_mul(
        symbols: &[&str],
        l: &Box<Expression>,
        r: &Box<Expression>,
    ) -> Vec<Expression> {
        l.differential(symbols)
            .into_iter()
            .zip(r.differential(symbols).into_iter())
            .map(|(li, ri)| li * r.as_ref().clone() + l.as_ref().clone() * ri)
            .collect()
    }
}
