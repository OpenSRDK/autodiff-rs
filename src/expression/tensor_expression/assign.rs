use crate::{TensorExpression, Value};
use std::collections::HashMap;

impl TensorExpression {
    pub fn assign(&self, values: &HashMap<&str, Value>) -> TensorExpression {
        match self {
            TensorExpression::Symbol(symbol, sizes) => {
                TensorExpression::assign_symbol(values, symbol, sizes)
            }
            TensorExpression::Constant(v) => TensorExpression::Constant(v.clone()),
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
                terms: v.iter().map(|v| v.assign(values)).collect(),
                rank_combinations: rank_combinations.clone(),
            },
        }
    }
}
