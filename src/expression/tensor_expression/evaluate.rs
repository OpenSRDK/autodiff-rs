use crate::{TensorExpression, Value};
use std::collections::HashMap;

impl TensorExpression {
    pub fn evaluate(&self, values: &HashMap<&str, Value>) -> TensorExpression {
        match self {
            TensorExpression::Symbol(symbol, levels) => {
                TensorExpression::evaluate_symbol(values, symbol, *levels)
            }
            TensorExpression::Constant(v) => TensorExpression::Constant(v.clone()),
            TensorExpression::Zero => TensorExpression::Zero,
            TensorExpression::Add(l, r) => l.evaluate(values) + r.evaluate(values),
            TensorExpression::Sub(l, r) => l.evaluate(values) - r.evaluate(values),
            TensorExpression::MulScalarLhs(l, r) => l.evaluate(values) * r.evaluate(values),
            TensorExpression::MulScalarRhs(l, r) => l.evaluate(values) * r.evaluate(values),
            TensorExpression::Neg(v) => -v.evaluate(values),
            TensorExpression::KroneckerDeltas(level_pairs) => {
                TensorExpression::KroneckerDeltas(level_pairs.clone())
            }
            TensorExpression::InnerProd {
                lhs,
                rhs,
                level_pairs,
            } => lhs
                .evaluate(values)
                .inner_prod(rhs.evaluate(values), level_pairs),
            TensorExpression::Det(v) => v.evaluate(values).det(),
        }
    }
}
