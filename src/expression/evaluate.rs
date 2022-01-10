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
            Expression::Neg(expression) => -expression.evaluate(values),
            Expression::Abs(arg) => arg.evaluate(values).abs(),
            Expression::Pow(base, exponent) => base.evaluate(values).pow(exponent.evaluate(values)),
            Expression::Exp(arg) => arg.evaluate(values).exp(),
            Expression::Log(base, antilogarithm) => {
                base.evaluate(values).log(antilogarithm.evaluate(values))
            }
            Expression::Ln(arg) => arg.evaluate(values).ln(),
            Expression::Sin(arg) => arg.evaluate(values).sin(),
            Expression::Cos(arg) => arg.evaluate(values).cos(),
            Expression::Tan(arg) => arg.evaluate(values).tan(),
            Expression::MatrixScalar(v) => v.evaluate(values).as_scalar(),
        }
    }
}
