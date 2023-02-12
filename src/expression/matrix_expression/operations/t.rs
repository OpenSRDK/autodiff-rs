use std::collections::HashMap;

use opensrdk_linear_algebra::Matrix;

use crate::{BracketsLevel, ConstantValue, Expression, MatrixExpression, TensorExpression};

impl Expression {
    pub fn t(self) -> Expression {
        if let Expression::Constant(v) = &self {
            let t = |v: &Matrix| v.t().into();
            return match v {
                ConstantValue::Scalar(v) => v.abs().into(),
                ConstantValue::Tensor(v) => t(&v.reduce_1dimension_rank().to_mat()),
                ConstantValue::Matrix(v) => return t(v),
            };
        }

        MatrixExpression::T(self.into()).into()
    }
}

impl MatrixExpression {
    pub(crate) fn diff_t(v: &Expression, symbols: &[&str]) -> Vec<Expression> {
        let tensor = TensorExpression::KroneckerDeltas(vec![[0, 1]])
            .inner_prod(TensorExpression::Matrix(v.clone().into()), &[[0, 1]])
            .inner_prod(TensorExpression::KroneckerDeltas(vec![[0, 1]]), &[[0, 1]]);

        tensor.differential(symbols)
    }

    pub(crate) fn tex_code_t(v: &Expression, symbols: &HashMap<&str, &str>) -> String {
        format!(
            r"{}^\top",
            v._tex_code(symbols, BracketsLevel::ForOperation)
        )
    }
}
