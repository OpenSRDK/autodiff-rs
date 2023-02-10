use crate::{TensorExpression, Value};
use std::{
    collections::{HashMap, HashSet},
    iter::once,
};

impl TensorExpression {
    pub fn new_symbol(name: String, rank: usize) -> Self {
        TensorExpression::Symbol(name, rank)
    }

    pub fn symbols(&self) -> HashSet<String> {
        match self {
            TensorExpression::Symbol(symbol, _) => {
                once(symbol.clone()).collect::<HashSet<String>>()
            }
            TensorExpression::Constant(_) | TensorExpression::Zero => HashSet::new(),
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
            TensorExpression::KroneckerDeltas(rank_pairs) => HashSet::new(),
            TensorExpression::InnerProd {
                v,
                rank_combinations,
            } => v.iter().map(|v| v.symbols()).flatten().collect(),
        }
    }

    pub(crate) fn evaluate_symbol(
        values: &HashMap<&str, Value>,
        symbol: &String,
        rank: usize,
    ) -> TensorExpression {
        let v = values.get(symbol.as_str());

        match v {
            Some(v) => TensorExpression::Constant(v.as_tensor_ref().clone()),
            None => TensorExpression::Symbol(symbol.clone(), rank),
        }
    }

    pub(crate) fn diff_symbol(
        symbols: &[&str],
        symbol: &String,
        rank: usize,
    ) -> Vec<TensorExpression> {
        symbols
            .iter()
            .map(|s| {
                if s.eq(symbol) {
                    TensorExpression::KroneckerDeltas((0..rank).map(|r| [r, r + rank]).collect())
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
