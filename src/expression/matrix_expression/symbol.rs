use crate::MatrixExpression;
use std::{collections::HashSet, iter::once};

impl MatrixExpression {
    pub fn new_symbol(name: String) -> Self {
        MatrixExpression::Symbol(name)
    }

    pub fn symbols(&self) -> HashSet<String> {
        match self {
            MatrixExpression::Symbol(symbol) => once(symbol.clone()).collect::<HashSet<String>>(),
            MatrixExpression::Constant(_) => HashSet::new(),
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
}
