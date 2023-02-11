use crate::{BracketsLevel, TensorExpression};
use std::collections::HashMap;

impl TensorExpression {
    pub fn _tex_code(
        &self,
        symbols: &HashMap<&str, &str>,
        brackets_level: BracketsLevel,
    ) -> String {
        match self {
            TensorExpression::Symbol(symbol, _) => format!("{{{}}}", symbols[symbol.as_str()]),
            TensorExpression::Constant(_) => r"\text{const.}".to_owned(),
            TensorExpression::Zero => r"\mathbf{0}".to_owned(),
            TensorExpression::Add(l, r) => {
                TensorExpression::tex_code_add(l, r, symbols, brackets_level)
            }
            TensorExpression::Sub(l, r) => {
                TensorExpression::tex_code_sub(l, r, symbols, brackets_level)
            }
            TensorExpression::MulScalarLhs(l, r) => {
                TensorExpression::tex_code_mul_scalar_lhs(l, r, symbols, brackets_level)
            }
            TensorExpression::MulScalarRhs(l, r) => {
                TensorExpression::tex_code_mul_scalar_rhs(l, r, symbols, brackets_level)
            }
            TensorExpression::Neg(v) => TensorExpression::tex_code_neg(v, symbols),
            TensorExpression::KroneckerDeltas(rank_pairs) => {
                TensorExpression::tex_code_kronecker_deltas(rank_pairs, brackets_level)
            }
            TensorExpression::InnerProd {
                terms,
                rank_combinations,
            } => TensorExpression::tex_code_inner_prod(terms, rank_combinations, symbols),
            TensorExpression::Matrix(m) => m._tex_code(symbols, brackets_level),
        }
    }

    pub fn tex_code(&self, symbols: &HashMap<&str, &str>) -> String {
        self._tex_code(symbols, BracketsLevel::None)
    }
}
