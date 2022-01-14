use crate::Expression;
use std::ops::Sub;

impl Sub<Expression> for Expression {
    type Output = Self;

    fn sub(self, rhs: Expression) -> Self::Output {
        if let Expression::Constant(vl) = self {
            if let Expression::Constant(vr) = rhs {
                return Self::Constant(vl - vr);
            }
            if vl == 0.0 {
                return rhs;
            }
        }
        if let Expression::Constant(vr) = rhs {
            if vr == 0.0 {
                return self;
            }
        }
        Self::Sub(self.into(), rhs.into())
    }
}

impl Sub<f64> for Expression {
    type Output = Self;

    fn sub(self, rhs: f64) -> Self::Output {
        self - Expression::Constant(rhs)
    }
}

impl Sub<Expression> for f64 {
    type Output = Expression;

    fn sub(self, rhs: Expression) -> Self::Output {
        Expression::Constant(self) - rhs
    }
}

impl Expression {
    pub(crate) fn diff_sub(
        symbols: &[&str],
        l: &Box<Expression>,
        r: &Box<Expression>,
    ) -> Vec<Expression> {
        l.differential(symbols)
            .into_iter()
            .zip(r.differential(symbols).into_iter())
            .map(|(li, ri)| li - ri)
            .collect()
    }

    pub(crate) fn rust_code_sub(
        l: &Box<Expression>,
        r: &Box<Expression>,
        parentheses: bool,
    ) -> String {
        if parentheses {
            format!("({} - {})", l._rust_code(true), r._rust_code(true))
        } else {
            format!("{} - {}", l._rust_code(true), r._rust_code(true))
        }
    }
}
