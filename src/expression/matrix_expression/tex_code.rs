use std::collections::HashMap;

use crate::{BracketsLevel, MatrixExpression};

impl MatrixExpression {
    pub(crate) fn _tex_code(
        &self,
        variables: &HashMap<&str, &str>,
        _brackets_level: BracketsLevel,
    ) -> String {
        match self {
            MatrixExpression::T(v) => MatrixExpression::tex_code_t(v, variables),
            MatrixExpression::Inv(v) => MatrixExpression::tex_code_inv(v, variables),
            MatrixExpression::Det(v) => MatrixExpression::tex_code_det(v, variables),
        }
    }

    pub fn tex_code(&self, symbols: &HashMap<&str, &str>) -> String {
        self._tex_code(symbols, BracketsLevel::None)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use opensrdk_linear_algebra::{sparse::SparseTensor, Matrix};

    use crate::{new_variable, new_variable_tensor, Expression, MatrixExpression, Size};

    #[test]
    fn it_works() {
        let id = "x";
        let ea = new_variable_tensor((id).to_string(), vec![Size::Many, Size::Many]);
        let tex_symbols = vec![("x", "y")].into_iter().collect();

        let ea_t = ea.clone().t();
        let tex_a_t = ea_t.tex_code(&tex_symbols);
        assert_eq!("{y}^\\top", tex_a_t);

        let ea_inv = ea.clone().inv();
        let tex_a_inv = ea_inv.tex_code(&tex_symbols);
        assert_eq!("{{y}^{-1}}", tex_a_inv);

        let ea_det = ea.clone().det();
        let tex_a_det = ea_det.tex_code(&tex_symbols);
        assert_eq!("\\left\\|{y}\\right\\|", tex_a_det);
    }
}
