use crate::{Expression, Value};
use std::collections::HashMap;

impl Expression {
    pub fn evaluate(&self, values: &HashMap<&str, Value>) -> Expression {
        match self {
            Expression::Symbol(symbol) => {
                let v = values.get(symbol.as_str());

                match v {
                    Some(v) => Expression::Constant(v.as_scalar()),
                    None => Expression::Symbol(symbol.clone()),
                }
            }
            Expression::Constant(v) => Expression::Constant(*v),
            Expression::Add(l, r) => l.evaluate(values) + r.evaluate(values),
            Expression::Sub(l, r) => l.evaluate(values) - r.evaluate(values),
            Expression::Mul(l, r) => l.evaluate(values) * r.evaluate(values),
            Expression::Div(l, r) => l.evaluate(values) / r.evaluate(values),
            Expression::Neg(v) => -v.evaluate(values),
            Expression::Pow(base, exponent) => base.evaluate(values).pow(*exponent),
            Expression::Transcendental(v) => v.evaluate(values),
            Expression::MatrixScalar(v) => v.evaluate(values).as_scalar(),
        }
    }
}
