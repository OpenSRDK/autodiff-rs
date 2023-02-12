use crate::{Expression, Size, TensorExpression};
use std::collections::HashSet;

pub fn new_symbol_tensor(name: String, sizes: Vec<Size>) -> Expression {
    Expression::Symbol(name, sizes)
}

impl TensorExpression {
    pub fn symbols(&self) -> HashSet<String> {
        match self {
            TensorExpression::KroneckerDeltas(_) => HashSet::new(),
            TensorExpression::InnerProd {
                terms,
                rank_combinations: _,
            } => terms.iter().map(|v| v.symbols()).flatten().collect(),
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
