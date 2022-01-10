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
            MatrixExpression::Add(l, r) => l.evaluate(values) + r.evaluate(values),
            MatrixExpression::Sub(l, r) => l.evaluate(values) - r.evaluate(values),
            MatrixExpression::Mul(l, r) => l.evaluate(values) * r.evaluate(values),
            MatrixExpression::MulScalar(l, r) => l.evaluate(values) * r.evaluate(values),
            MatrixExpression::Neg(expression) => -expression.evaluate(values),
            MatrixExpression::Det(expression) => expression.evaluate(values).det(),
            MatrixExpression::T(expression) => expression.evaluate(values).t(),
            MatrixExpression::Inv(expression) => expression.evaluate(values).inv(),
            MatrixExpression::Solve(l, r) => l.evaluate(values).solve(r.evaluate(values)),
            MatrixExpression::Pow(base, exponent) => todo!(),
            MatrixExpression::MatrixExp(arg) => todo!(),
        }
    }
}
