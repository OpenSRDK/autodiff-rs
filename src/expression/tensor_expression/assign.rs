use crate::{TensorExpression, Value};
use std::collections::HashMap;

impl TensorExpression {
    pub fn assign(self, values: &HashMap<&str, Value>) -> TensorExpression {
        match self {
            TensorExpression::Symbol(symbol, sizes) => {
                let v = values.get(symbol.as_str());

                match v {
                    Some(v) => TensorExpression::Constant(v.as_tensor_ref().clone()),
                    None => TensorExpression::Symbol(symbol.clone(), sizes.clone()),
                }
            }
            TensorExpression::Constant(_) => self,
            TensorExpression::Zero => TensorExpression::Zero,
            TensorExpression::Add(l, r) => l.assign(values) + r.assign(values),
            TensorExpression::Sub(l, r) => l.assign(values) - r.assign(values),
            TensorExpression::MulScalarLhs(l, r) => l.assign(values) * r.assign(values),
            TensorExpression::MulScalarRhs(l, r) => l.assign(values) * r.assign(values),
            TensorExpression::Neg(v) => -v.assign(values),
            TensorExpression::KroneckerDeltas(rank_pairs) => {
                TensorExpression::KroneckerDeltas(rank_pairs.clone())
            }
            TensorExpression::InnerProd {
                terms: v,
                rank_combinations,
            } => TensorExpression::InnerProd {
                terms: v.into_iter().map(|v| v.assign(values)).collect(),
                rank_combinations: rank_combinations.clone(),
            },
            TensorExpression::Matrix(m) => m.assign(values).into(),
        }
    }
}
