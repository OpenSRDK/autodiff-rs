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
