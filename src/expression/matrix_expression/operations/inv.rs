use std::collections::HashMap;

use opensrdk_linear_algebra::Matrix;

use crate::{BracketsLevel, ConstantValue, Expression, MatrixExpression};

impl Expression {
    pub fn inv(self) -> Expression {
        if let Expression::Constant(v) = self {
            let inv = |v: Matrix| v.getrf().unwrap().getri().unwrap().into();
            return match v {
                ConstantValue::Scalar(v) => v.abs().into(),
                ConstantValue::Tensor(v) => inv(v.reduce_1dimension_rank().to_mat()),
                ConstantValue::Matrix(v) => return inv(v),
            };
        }

        MatrixExpression::Inv(self.into()).into()
    }
}

impl MatrixExpression {
    pub(crate) fn diff_inv(v: &Expression, symbols: &[&str]) -> Vec<Expression> {
        v.differential(symbols)
            .into_iter()
            .map(|d_v_d_symbol| {
                let v_inv = v.clone().inv();
                let d_v_inv_d_v = -v_inv.clone().dot(v_inv, &[[1, 0]]);

                d_v_inv_d_v.dot(d_v_d_symbol, &[[0, 0], [1, 1]])
            })
            .collect()
    }

    pub(crate) fn tex_code_inv(v: &Expression, symbols: &HashMap<&str, &str>) -> String {
        format!(
            r"{{{}^{{-1}}}}",
            v._tex_code(symbols, BracketsLevel::ForOperation)
        )
    }
}
