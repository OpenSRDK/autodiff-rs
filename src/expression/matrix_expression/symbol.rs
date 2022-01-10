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
