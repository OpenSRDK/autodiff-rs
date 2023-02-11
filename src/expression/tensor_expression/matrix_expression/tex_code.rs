use std::collections::HashMap;

use crate::{BracketsLevel, MatrixExpression};

impl MatrixExpression {
    pub(crate) fn _tex_code(
        &self,
        symbols: &HashMap<&str, &str>,
        brackets_level: BracketsLevel,
    ) -> String {
        match self {
            MatrixExpression::Mat(v) => v._tex_code(symbols, brackets_level),
            MatrixExpression::Constant(_) => r"\text{{const.}}".to_owned(),
            MatrixExpression::T(v) => MatrixExpression::tex_code_t(v, symbols),
            MatrixExpression::Inv(v) => MatrixExpression::tex_code_inv(v, symbols),
            MatrixExpression::Det(v) => MatrixExpression::tex_code_det(v, symbols),
        }
    }

    pub fn tex_code(&self, symbols: &HashMap<&str, &str>) -> String {
        self._tex_code(symbols, BracketsLevel::None)
    }
}
