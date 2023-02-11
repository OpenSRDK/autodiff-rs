use crate::{Size, TensorExpression, Value};
use std::{
    collections::{HashMap, HashSet},
    iter::once,
};

pub fn new_symbol_tensor(name: String, sizes: Vec<Size>) -> TensorExpression {
    TensorExpression::Symbol(name, sizes)
}

impl TensorExpression {
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
            TensorExpression::KroneckerDeltas(_) => HashSet::new(),
            TensorExpression::InnerProd {
                terms,
                rank_combinations: _,
            } => terms.iter().map(|v| v.symbols()).flatten().collect(),
            TensorExpression::Matrix(m) => m.symbols(),
        }
    }

    pub(crate) fn diff_symbol(
        symbol: &String,
        sizes: &Vec<Size>,
        symbols: &[&str],
    ) -> Vec<TensorExpression> {
        let rank = sizes.len();
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
