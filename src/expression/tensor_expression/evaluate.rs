use crate::{TensorExpression, Value};
use std::collections::HashMap;

impl TensorExpression {
    pub fn evaluate(&self, values: &HashMap<&str, Value>) -> TensorExpression {
        match self {
            TensorExpression::Symbol(symbol, rank) => {
                TensorExpression::evaluate_symbol(values, symbol, *rank)
            }
            TensorExpression::Constant(v) => TensorExpression::Constant(v.clone()),
            TensorExpression::Zero => TensorExpression::Zero,
            TensorExpression::Add(l, r) => l.evaluate(values) + r.evaluate(values),
            TensorExpression::Sub(l, r) => l.evaluate(values) - r.evaluate(values),
            TensorExpression::MulScalarLhs(l, r) => l.evaluate(values) * r.evaluate(values),
            TensorExpression::MulScalarRhs(l, r) => l.evaluate(values) * r.evaluate(values),
            TensorExpression::Neg(v) => -v.evaluate(values),
            TensorExpression::KroneckerDeltas(rank_pairs) => {
                TensorExpression::KroneckerDeltas(rank_pairs.clone())
            }
            TensorExpression::InnerProd {
                v,
                rank_combinations,
            } => TensorExpression::InnerProd {
                v: v.iter().map(|v| v.evaluate(values)).collect(),
                rank_combinations: rank_combinations.clone(),
            },
        }
    }
}
