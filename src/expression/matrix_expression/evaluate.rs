use crate::{MatrixExpression, Value};
use std::collections::HashMap;

impl MatrixExpression {
    pub fn evaluate(&self, values: &HashMap<&str, Value>) -> MatrixExpression {
        match self {
            MatrixExpression::Symbol(symbol) => {
                let v = values.get(symbol.as_str());

                match v {
                    Some(v) => MatrixExpression::Constant(v.as_matrix_ref().clone()),
                    None => MatrixExpression::Symbol(symbol.clone()),
                }
            }
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
            MatrixExpression::MatrixExp(v) => todo!(),
        }
    }
}
