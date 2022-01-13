use crate::{MatrixExpression, Value};
use std::{
    collections::{HashMap, HashSet},
    iter::once,
};

impl MatrixExpression {
    pub fn new_symbol(name: String) -> Self {
        MatrixExpression::Symbol(name)
    }

    pub fn symbols(&self) -> HashSet<String> {
        match self {
            MatrixExpression::Symbol(symbol) => once(symbol.clone()).collect::<HashSet<String>>(),
            MatrixExpression::Constant(_) => HashSet::new(),
            MatrixExpression::Zero => todo!(),
            MatrixExpression::Unit => todo!(),
            MatrixExpression::Add(l, r) => l
                .symbols()
                .into_iter()
                .chain(r.symbols().into_iter())
                .collect(),
            MatrixExpression::Sub(l, r) => l
                .symbols()
                .into_iter()
                .chain(r.symbols().into_iter())
                .collect(),
            MatrixExpression::Mul(l, r) => l
                .symbols()
                .into_iter()
                .chain(r.symbols().into_iter())
                .collect(),
            MatrixExpression::MulScalar(l, r) => l
                .symbols()
                .into_iter()
                .chain(r.symbols().into_iter())
                .collect(),
            MatrixExpression::Neg(v) => v.symbols(),
            MatrixExpression::Pow(base, _) => base.symbols(),
            MatrixExpression::T(v) => v.symbols(),
            MatrixExpression::Det(v) => v.symbols(),
            MatrixExpression::MatrixExp(v) => v.symbols(),
        }
    }

    pub(crate) fn evaluate_symbol(
        values: &HashMap<&str, Value>,
        symbol: &String,
    ) -> MatrixExpression {
        let v = values.get(symbol.as_str());

        match v {
            Some(v) => MatrixExpression::Constant(v.as_matrix_ref().clone()),
            None => MatrixExpression::Symbol(symbol.clone()),
        }
    }

    pub(crate) fn diff_symbol(symbols: &[&str], symbol: &String) -> Vec<MatrixExpression> {
        symbols
            .iter()
            .map(|s| {
                if s.eq(symbol) {
                    MatrixExpression::Unit
                } else {
                    MatrixExpression::Zero
                }
            })
            .collect()
    }

    pub(crate) fn rust_code_symbol(symbol: &String) -> String {
        format!("{}.clone()", symbol)
    }
}
