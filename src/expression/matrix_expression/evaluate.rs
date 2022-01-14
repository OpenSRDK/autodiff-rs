use crate::{MatrixExpression, Value};
use std::collections::HashMap;

impl MatrixExpression {
    pub fn evaluate(&self, values: &HashMap<&str, Value>) -> MatrixExpression {
        match self {
            MatrixExpression::Symbol(symbol) => MatrixExpression::evaluate_symbol(values, symbol),
            MatrixExpression::Constant(v) => MatrixExpression::Constant(v.clone()),
            MatrixExpression::Zero => todo!(),
            MatrixExpression::Unit => todo!(),
            MatrixExpression::Add(l, r) => l.evaluate(values) + r.evaluate(values),
            MatrixExpression::Sub(l, r) => l.evaluate(values) - r.evaluate(values),
            MatrixExpression::Mul(l, r) => l.evaluate(values) * r.evaluate(values),
            MatrixExpression::MulScalar(l, r) => l.evaluate(values) * r.evaluate(values),
            MatrixExpression::Neg(v) => -v.evaluate(values),
            MatrixExpression::Pow(base, exponent) => base.evaluate(values).pow(*exponent),
            MatrixExpression::T(v) => v.evaluate(values).t(),
            MatrixExpression::Det(v) => v.evaluate(values).det(),
        }
    }
}
