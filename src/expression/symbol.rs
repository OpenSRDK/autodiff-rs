use crate::Expression;
use std::{collections::HashSet, iter::once};

impl Expression {
    pub fn new_symbol(name: String) -> Self {
        Expression::Symbol(name)
    }

    pub fn symbols(&self) -> HashSet<String> {
        match self {
            Expression::Symbol(symbol) => once(symbol.clone()).collect::<HashSet<String>>(),
            Expression::Constant(_) => HashSet::new(),
            Expression::Add(l, r) => l
                .symbols()
                .into_iter()
                .chain(r.symbols().into_iter())
                .collect(),
            Expression::Sub(l, r) => l
                .symbols()
                .into_iter()
                .chain(r.symbols().into_iter())
                .collect(),
            Expression::Mul(l, r) => l
                .symbols()
                .into_iter()
                .chain(r.symbols().into_iter())
                .collect(),
            Expression::Div(l, r) => l
                .symbols()
                .into_iter()
                .chain(r.symbols().into_iter())
                .collect(),
            Expression::Neg(expression) => expression.symbols(),
            Expression::Abs(arg) => arg.symbols(),
            Expression::Pow(base, exponential) => base
                .symbols()
                .into_iter()
                .chain(exponential.symbols().into_iter())
                .collect(),
            Expression::Exp(arg) => arg.symbols(),
            Expression::Log(l, antilogarithm) => l
                .symbols()
                .into_iter()
                .chain(antilogarithm.symbols().into_iter())
                .collect(),
            Expression::Ln(arg) => arg.symbols(),
            Expression::Sin(arg) => arg.symbols(),
            Expression::Cos(arg) => arg.symbols(),
            Expression::Tan(arg) => arg.symbols(),
            Expression::MatrixScalar(arg) => arg.symbols(),
        }
    }

    pub(crate) fn diff_symbol(symbols: &[&str], symbol: &String) -> Vec<Expression> {
        symbols
            .iter()
            .map(|s| {
                if s.eq(symbol) {
                    Expression::Constant(1.0)
                } else {
                    Expression::Constant(0.0)
                }
            })
            .collect()
    }
}
