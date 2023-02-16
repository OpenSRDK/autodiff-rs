use crate::{AbstractSize, ConstantValue, Expression, ExpressionArray};
use std::collections::HashMap;

impl Expression {
    pub fn assign(self, variables: &HashMap<&str, ConstantValue>) -> Expression {
        match self {
            Expression::Variable(id, sizes) => {
                let v = variables.get(id.as_str());

                match v {
                    Some(v) => {
                        if sizes != v.sizes().into_abstract_size() {
                            panic!("Variable {} has sizes {:?} but is assigned a value with sizes {:?}", id, sizes, v.sizes());
                        }
                        v.clone().into()
                    }
                    None => Expression::Variable(id.clone(), sizes.clone()),
                }
            }
            Expression::Constant(_) => self,
            Expression::PartialVariable(v) => Expression::PartialVariable(
                ExpressionArray::from_factory(v.sizes().to_vec(), |indices| {
                    v[indices].clone().assign(variables)
                }),
            ),
            Expression::Add(l, r) => l.assign(variables) + r.assign(variables),
            Expression::Sub(l, r) => l.assign(variables) - r.assign(variables),
            Expression::Mul(l, r) => l.assign(variables) * r.assign(variables),
            Expression::Div(l, r) => l.assign(variables) / r.assign(variables),
            Expression::Neg(v) => -v.assign(variables),
            Expression::Transcendental(v) => v.assign(variables),
            Expression::Tensor(v) => v.assign(variables),
            Expression::Matrix(v) => v.assign(variables),
        }
    }
}
