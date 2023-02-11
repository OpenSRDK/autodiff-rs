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
        symbols: &HashMap<&str, &str>,
        brackets_level: BracketsLevel,
    ) -> String {
        match self {
            Expression::Symbol(symbol) => format!("{{{}}}", symbols[symbol.as_str()]),
            Expression::Constant(value) => format!("{}", value),
            Expression::Add(l, r) => Expression::tex_code_add(l, r, symbols, brackets_level),
            Expression::Sub(l, r) => Expression::tex_code_sub(l, r, symbols, brackets_level),
            Expression::Mul(l, r) => Expression::tex_code_mul(l, r, symbols, brackets_level),
            Expression::Div(l, r) => Expression::tex_code_div(l, r, symbols, brackets_level),
            Expression::Neg(v) => Expression::tex_code_neg(v, symbols),
            Expression::Pow(base, exponent) => Expression::tex_code_pow(base, exponent, symbols),
            Expression::Transcendental(v) => v._tex_code(symbols, brackets_level),
            Expression::DegeneratedTensor(v) => v._tex_code(symbols, brackets_level),
            Expression::DiffResultTensor(v) => v._tex_code(symbols, brackets_level),
        }
    }

    pub fn tex_code(&self, symbols: &HashMap<&str, &str>) -> String {
        self._tex_code(symbols, BracketsLevel::None)
    }
}
