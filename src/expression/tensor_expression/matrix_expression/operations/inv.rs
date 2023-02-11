use std::collections::HashMap;

use crate::{BracketsLevel, MatrixExpression, TensorExpression};

impl MatrixExpression {
    pub fn inv(self) -> MatrixExpression {
        match self {
            MatrixExpression::Mat(_) => MatrixExpression::Inv(self.into()),
            MatrixExpression::Constant(v) => {
                MatrixExpression::Constant(v.getrf().unwrap().getri().unwrap())
            }
            MatrixExpression::T(_) => MatrixExpression::Inv(self.into()),
            MatrixExpression::Inv(v) => *v,
            MatrixExpression::Det(_) => panic!(),
        }
    }
}

impl MatrixExpression {
    pub(crate) fn diff_inv(v: &MatrixExpression, symbols: &[&str]) -> Vec<TensorExpression> {
        v.differential(symbols)
            .into_iter()
            .map(|d_v_d_symbol| {
                let v_inv = v.clone().inv().as_tensor();
                let d_v_inv_d_v = -v_inv.clone().inner_prod(v_inv, &[[1, 0]]);

                d_v_inv_d_v.inner_prod(d_v_d_symbol, &[[0, 0], [1, 1]])
            })
            .collect()
    }

    pub(crate) fn tex_code_inv(v: &MatrixExpression, symbols: &HashMap<&str, &str>) -> String {
        format!(
            r"{{{}^{{-1}}}}",
            v._tex_code(symbols, BracketsLevel::ForOperation)
        )
    }
}
