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
            Expression::PartialVariable(_, _) => r"\text{abbreviated.}".to_owned(),
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
