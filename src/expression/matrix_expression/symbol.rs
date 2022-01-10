use crate::{MatrixExpression, Symbol};
use rand::prelude::*;
use std::{collections::HashSet, iter::once};

impl Symbol {
    pub fn new_matrix() -> MatrixExpression {
        let id = thread_rng().gen();
        MatrixExpression::Symbol(Self::new(id))
    }
}

impl MatrixExpression {
    pub fn symbols(&self) -> HashSet<Symbol> {
        match self {
            MatrixExpression::Symbol(symbol) => once(symbol.clone()).collect::<HashSet<Symbol>>(),
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
            MatrixExpression::Neg(expression) => expression.symbols(),
            MatrixExpression::Det(expression) => expression.symbols(),
            MatrixExpression::T(expression) => expression.symbols(),
            MatrixExpression::Inv(expression) => expression.symbols(),
            MatrixExpression::Solve(l, r) => l
                .symbols()
                .into_iter()
                .chain(r.symbols().into_iter())
                .collect(),
            MatrixExpression::Pow(base, exponential) => base
                .symbols()
                .into_iter()
                .chain(exponential.symbols().into_iter())
                .collect(),
            MatrixExpression::MatrixExp(arg) => arg.symbols(),
        }
    }
}
