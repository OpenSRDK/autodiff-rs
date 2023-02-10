use crate::{Expression, TensorExpression, Value};
use opensrdk_linear_algebra::Tensor;
use std::collections::HashMap;

impl Expression {
    pub fn assign(&self, values: &HashMap<&str, Value>) -> Expression {
        match self {
            Expression::Symbol(symbol) => {
                let v = values.get(symbol.as_str());

                match v {
                    Some(v) => Expression::Constant(v.as_scalar()),
                    None => Expression::Symbol(symbol.clone()),
                }
            }
            Expression::Constant(v) => Expression::Constant(*v),
            Expression::Add(l, r) => l.assign(values) + r.assign(values),
            Expression::Sub(l, r) => l.assign(values) - r.assign(values),
            Expression::Mul(l, r) => l.assign(values) * r.assign(values),
            Expression::Div(l, r) => l.assign(values) / r.assign(values),
            Expression::Neg(v) => -v.assign(values),
            Expression::Pow(base, exponent) => base.assign(values).powr(*exponent),
            Expression::Transcendental(v) => v.assign(values),
            Expression::DegeneratedTensor(v) => match v.assign(values) {
                TensorExpression::Constant(v) => Expression::Constant(v[&vec![0; v.rank()]]),
                _ => Expression::DegeneratedTensor(v.clone()),
            },
            Expression::DiffResultTensor(v) => {
                Expression::DiffResultTensor(v.assign(values).into())
            }
        }
    }
}
