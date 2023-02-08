use crate::{TensorExpression, Value};
use std::{
    collections::{HashMap, HashSet},
    iter::once,
};

impl TensorExpression {
    pub fn new_symbol(name: String) -> Self {
        TensorExpression::Symbol(name)
    }

    pub fn symbols(&self) -> HashSet<String> {
        match self {
            TensorExpression::Symbol(symbol) => once(symbol.clone()).collect::<HashSet<String>>(),
            TensorExpression::Constant(_) | TensorExpression::Zero | TensorExpression::Unit => {
                HashSet::new()
            }
            TensorExpression::Add(l, r) => l
                .symbols()
                .into_iter()
                .chain(r.symbols().into_iter())
                .collect(),
            TensorExpression::Sub(l, r) => l
                .symbols()
                .into_iter()
                .chain(r.symbols().into_iter())
                .collect(),
            TensorExpression::MulScalarLhs(l, r) => l
                .symbols()
                .into_iter()
                .chain(r.symbols().into_iter())
                .collect(),
            TensorExpression::MulScalarRhs(l, r) => l
                .symbols()
                .into_iter()
                .chain(r.symbols().into_iter())
                .collect(),
            TensorExpression::Neg(v) => v.symbols(),
            TensorExpression::InnerProd {
                lhs,
                rhs,
                level_pairs,
            } => lhs
                .symbols()
                .into_iter()
                .chain(rhs.symbols().into_iter())
                .collect(),
            TensorExpression::Det(v) => v.symbols(),
        }
    }

    pub(crate) fn evaluate_symbol(
        values: &HashMap<&str, Value>,
        symbol: &String,
    ) -> TensorExpression {
        let v = values.get(symbol.as_str());

        match v {
            Some(v) => TensorExpression::Constant(v.as_tensor_ref().clone()),
            None => TensorExpression::Symbol(symbol.clone()),
        }
    }

    pub(crate) fn diff_symbol(symbols: &[&str], symbol: &String) -> Vec<TensorExpression> {
        symbols
            .iter()
            .map(|s| {
                if s.eq(symbol) {
                    TensorExpression::Unit
                } else {
                    TensorExpression::Zero
                }
            })
            .collect()
    }

    pub(crate) fn rust_code_symbol(symbol: &String) -> String {
        format!("{}.clone()", symbol)
    }
}
