use crate::{Expression, Size, TensorExpression};
use std::collections::HashSet;

pub fn new_symbol_tensor(name: String, sizes: Vec<Size>) -> Expression {
    Expression::Symbol(name, sizes)
}

impl TensorExpression {
    pub fn symbols(&self) -> HashSet<&str> {
        match self {
            TensorExpression::KroneckerDeltas(_) => HashSet::new(),
            TensorExpression::InnerProd {
                terms,
                rank_combinations: _,
            } => terms.iter().map(|v| v.symbols()).flatten().collect(),
        }
    }
}
