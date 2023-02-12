use crate::{ConstantValue, Expression};
use std::collections::HashMap;

impl Expression {
    pub fn assign(self, values: &HashMap<&str, ConstantValue>) -> Expression {
        match self {
            Expression::Symbol(symbol, sizes) => {
                let v = values.get(symbol.as_str());

                match v {
                    Some(v) => v.clone().into(),
                    None => Expression::Symbol(symbol.clone(), sizes.clone()),
                }
            }
            Expression::Constant(_) => self,
            Expression::Tensor(v) => v.assign(values),
            Expression::Add(l, r) => l.assign(values) + r.assign(values),
            Expression::Sub(l, r) => l.assign(values) - r.assign(values),
            Expression::Mul(l, r) => l.assign(values) * r.assign(values),
            Expression::Div(l, r) => l.assign(values) / r.assign(values),
            Expression::Neg(v) => -v.assign(values),
            Expression::Transcendental(v) => v.assign(values),
            Expression::Matrix(v) => v.assign(values),
            Expression::Index(v, index) => todo!(),
            Expression::IndexedTensor(v) => todo!(),
        }
    }
}
