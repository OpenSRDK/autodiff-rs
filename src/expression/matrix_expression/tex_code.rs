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

    use crate::{new_variable, Expression, MatrixExpression};

    #[test]
    fn it_works1() {
        let len = 7usize;
        let a = Matrix::from(len, vec![1.0; len * len]).unwrap();
        let ea = Expression::from(a);
        let mea_t = MatrixExpression::T(Box::new(ea.clone()));

        let tex_symbols = vec![].into_iter().collect();

        let tex_a = mea_t.tex_code(&tex_symbols);

        assert_eq!("\\text{const.}^\\top", tex_a);
    }

    #[test]
    fn it_works2() {
        let id = "x";
        let ea = new_variable((id).to_string());
        let mea_t = MatrixExpression::T(Box::new(ea.clone()));

        let tex_symbols = vec![("x", "y")].into_iter().collect();

        let tex_a = mea_t.tex_code(&tex_symbols);

        assert_eq!("{y}^\\top", tex_a);
    }
}
