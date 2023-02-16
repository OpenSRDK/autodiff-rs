use crate::{BracketsLevel, TensorExpression};
use std::collections::HashMap;

impl TensorExpression {
    pub fn _tex_code(
        &self,
        symbols: &HashMap<&str, &str>,
        brackets_level: BracketsLevel,
    ) -> String {
        match self {
            TensorExpression::KroneckerDeltas(rank_pairs) => {
                TensorExpression::tex_code_kronecker_deltas(rank_pairs, brackets_level)
            }
            TensorExpression::DotProduct {
                terms,
                rank_combinations,
            } => TensorExpression::tex_code_dot_product(terms, rank_combinations, symbols),
            TensorExpression::DirectProduct(terms) => {
                TensorExpression::tex_code_direct_product(terms, symbols)
            }
        }
    }

    pub fn tex_code(&self, symbols: &HashMap<&str, &str>) -> String {
        self._tex_code(symbols, BracketsLevel::None)
    }
}
