use std::collections::HashMap;

use opensrdk_linear_algebra::Matrix;

use crate::{BracketsLevel, ConstantValue, Expression, MatrixExpression};

impl Expression {
    pub fn det(self) -> Expression {
        if let Expression::Constant(v) = self {
            let det = |v: Matrix| v.getrf().unwrap().0.trdet().into();
            return match v {
                ConstantValue::Scalar(v) => v.abs().into(),
                ConstantValue::Tensor(v) => det(v.reduce_1dimension_rank().to_mat()),
                ConstantValue::Matrix(v) => return det(v),
            };
        }

        MatrixExpression::Det(self.into()).into()
    }
}

impl MatrixExpression {
    pub(crate) fn diff_det(v: &Expression, symbols: &[&str]) -> Vec<Expression> {
        v.differential(symbols)
            .into_iter()
            .map(|d_v_d_symbol| {
                let v_det = v.clone().det();
                let v_inv_t = v.clone().inv().t();
                let d_v_det_d_v = v_det * v_inv_t;

                d_v_det_d_v.dot(d_v_d_symbol, &[[0, 0], [1, 1]])
            })
            .collect()
    }

    pub(crate) fn tex_code_det(v: &Expression, symbols: &HashMap<&str, &str>) -> String {
        format!(
            r"\left\|{}\right\|",
            v._tex_code(symbols, BracketsLevel::None)
        )
    }
}
