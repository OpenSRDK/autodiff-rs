use crate::{TensorExpression, Value};
use std::collections::HashMap;

impl TensorExpression {
    pub fn evaluate(&self, values: &HashMap<&str, Value>) -> TensorExpression {
        match self {
            TensorExpression::Symbol(symbol) => TensorExpression::evaluate_symbol(values, symbol),
            TensorExpression::Constant(v) => TensorExpression::Constant(v.clone()),
            TensorExpression::Zero => todo!(),
            TensorExpression::Unit => todo!(),
            TensorExpression::Add(l, r) => l.evaluate(values) + r.evaluate(values),
            TensorExpression::Sub(l, r) => l.evaluate(values) - r.evaluate(values),
            TensorExpression::MulScalarLhs(l, r) => l.evaluate(values) * r.evaluate(values),
            TensorExpression::MulScalarRhs(l, r) => l.evaluate(values) * r.evaluate(values),
            TensorExpression::Neg(v) => -v.evaluate(values),
            TensorExpression::InnerProd {
                lhs,
                rhs,
                level_pairs,
            } => lhs
                .evaluate(values)
                .inner_prod(&rhs.evaluate(values), level_pairs.clone()),
            TensorExpression::Det(v) => v.evaluate(values).det(),
        }
    }
}
