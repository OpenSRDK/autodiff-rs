use crate::Expression;
use std::{collections::HashSet, iter::once};

pub fn new_symbol(name: String) -> Expression {
    Expression::Symbol(name)
}

impl Expression {
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
            Expression::Neg(v) => v.symbols(),
            Expression::Pow(base, _) => base.symbols(),
            Expression::Transcendental(v) => v.symbols(),
            Expression::DegeneratedTensor(v) => v.symbols(),
            Expression::DiffResultTensor(v) => v.symbols(),
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

    pub(crate) fn rust_code_symbol(symbol: &String) -> String {
        symbol.clone()
    }
}
