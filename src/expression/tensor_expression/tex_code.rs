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
                TensorExpression::tex_code_direct_product(terms, symbols, brackets_level)
            }
        }
    }

    pub fn tex_code(&self, symbols: &HashMap<&str, &str>) -> String {
        self._tex_code(symbols, BracketsLevel::None)
    }
}

#[cfg(test)]
mod tests {
    use opensrdk_linear_algebra::generate_rank_combinations;

    use crate::expression::tensor_expression::TensorExpression;
    use crate::expression::Expression;
    use crate::{new_variable, new_variable_tensor, ConstantValue, Size};
    use std::collections::HashMap;

    #[test]
    fn it_works() {
        let id1 = "x";
        let id2 = "y";
        let tex_symbols = vec![("x", "x"), ("y", "y")].into_iter().collect();

        let ka = TensorExpression::KroneckerDeltas(vec![[1, 1], [2, 2]]).tex_code(&tex_symbols);
        assert_eq!(ka, r"{\delta_{[1], [1]}} {\delta_{[2], [2]}}");

        let rank_combination = generate_rank_combinations(&[[2, 2]]);

        let edo = TensorExpression::DotProduct {
            terms: vec![
                new_variable((id1).to_string()),
                new_variable((id2).to_string()),
            ],
            rank_combinations: rank_combination.to_vec(),
        };
        println!("edo: {:?}", edo);
        let doa = edo.tex_code(&tex_symbols);
        println!("doa: {:?}", doa);

        let ed = TensorExpression::DirectProduct(vec![
            new_variable((id1).to_string()),
            new_variable((id2).to_string()),
        ])
        .tex_code(&tex_symbols);
        println!("ed: {:?}", ed);
    }
}
