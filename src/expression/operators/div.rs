use crate::Expression;
use std::ops::Div;

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
        symbols: &[&str],
        l: &Box<Expression>,
        r: &Box<Expression>,
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
}
