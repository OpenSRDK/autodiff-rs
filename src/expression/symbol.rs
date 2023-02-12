use crate::{Expression, Size, TensorExpression};
use std::{collections::HashSet, iter::once};

pub fn new_symbol(name: String) -> Expression {
    Expression::Symbol(name, vec![])
}

impl Expression {
    pub fn symbols(&self) -> HashSet<String> {
        match self {
            Expression::Symbol(symbol, _) => once(symbol.clone()).collect::<HashSet<String>>(),
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
            Expression::Transcendental(v) => v.symbols(),
            Expression::Tensor(v) => v.symbols(),
            Expression::Matrix(v) => v.symbols(),
            Expression::Index(v, index) => todo!(),
            Expression::IndexedTensor(v) => todo!(),
        }
    }

    pub(crate) fn diff_symbol(
        symbol: &String,
        sizes: &Vec<Size>,
        symbols: &[&str],
    ) -> Vec<Expression> {
        let rank = sizes.len();
        symbols
            .iter()
            .map(|s| {
                if s.eq(symbol) {
                    TensorExpression::KroneckerDeltas((0..rank).map(|r| [r, r + rank]).collect())
                        .into()
                } else {
                    0.0.into()
                }
            })
            .collect()
    }
}
