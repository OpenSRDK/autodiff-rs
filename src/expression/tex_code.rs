use crate::Expression;
use std::collections::HashMap;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BracketsLevel {
    None,
    ForMul,
    ForDiv,
    ForOperation,
}

impl Expression {
    pub(crate) fn _tex_code(
        &self,
        variables: &HashMap<&str, &str>,
        brackets_level: BracketsLevel,
    ) -> String {
        match self {
            Expression::Variable(id, _) => format!("{{{}}}", variables[id.as_str()]),
            Expression::Constant(_) => r"\text{const.}".to_owned(),
            Expression::PartialVariable(_) => r"\text{abbreviated.}".to_owned(),
            Expression::Add(l, r) => Expression::tex_code_add(l, r, variables, brackets_level),
            Expression::Sub(l, r) => Expression::tex_code_sub(l, r, variables, brackets_level),
            Expression::Mul(l, r) => Expression::tex_code_mul(l, r, variables, brackets_level),
            Expression::Div(l, r) => Expression::tex_code_div(l, r, variables, brackets_level),
            Expression::Neg(v) => Expression::tex_code_neg(v, variables),
            Expression::Transcendental(v) => v._tex_code(variables, brackets_level),
            Expression::Tensor(v) => v._tex_code(variables, brackets_level),
            Expression::Matrix(v) => v._tex_code(variables, brackets_level),
        }
    }

    pub fn tex_code(&self, symbols: &HashMap<&str, &str>) -> String {
        self._tex_code(symbols, BracketsLevel::None)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use opensrdk_linear_algebra::sparse::SparseTensor;

    use crate::{new_variable, Expression};

    #[test]
    fn it_works1() {
        let a = 5.0f64;
        let b = vec![a; 8];

        let ea = Expression::from(a);
        let eb = Expression::from(b);

        let tex_symbols = vec![].into_iter().collect();

        let tex_a = ea.tex_code(&tex_symbols);
        let tex_b = eb.tex_code(&tex_symbols);

        assert_eq!("\\text{const.}", tex_a);
        assert_eq!("\\text{const.}", tex_b);
    }

    #[test]
    fn it_works2() {
        let id = "x";
        let a = HashSet::from([id; 1]);
        let ea = new_variable((id).to_string());

        let tex_symbols = vec![("x", "y")].into_iter().collect();

        let tex_a = ea.tex_code(&tex_symbols);

        assert_eq!("{y}", tex_a);
    }
}
