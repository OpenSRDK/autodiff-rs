use crate::{Expression, Symbol};
use rand::prelude::*;
use std::{collections::HashSet, iter::once};

impl Symbol {
    pub fn new_scalar() -> Expression {
        let id = thread_rng().gen();
        Expression::Symbol(Self::new(id))
    }
}

impl Expression {
    pub fn symbols(&self) -> HashSet<Symbol> {
        match self {
            Expression::Symbol(symbol) => once(symbol.clone()).collect::<HashSet<Symbol>>(),
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
}
