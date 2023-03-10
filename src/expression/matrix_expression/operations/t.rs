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
        let delta_01: Expression = TensorExpression::KroneckerDeltas(vec![[0, 1]]).into();
        let tensor = delta_01
            .clone()
            .dot(v.clone(), &[[0, 1]])
            .dot(delta_01, &[[0, 1]]);

        tensor.differential(symbols)
    }

    pub(crate) fn tex_code_t(v: &Expression, symbols: &HashMap<&str, &str>) -> String {
        format!(
            r"{}^\top",
            v._tex_code(symbols, BracketsLevel::ForOperation)
        )
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{new_variable_tensor, Expression, MatrixExpression, Size};

    #[test]
    fn it_works() {
        let id = "x";
        let a = HashSet::from([id; 1]);
        let ea = new_variable_tensor((id).to_string(), vec![Size::Many, Size::Many]);

        let ea_t = ea.clone().t();

        let id2 = "g";
        let diff_ea_t = MatrixExpression::diff_t(&ea, &[id]);
        let tex_symbols = vec![("x", "y")].into_iter().collect();
        println!("{:?}", diff_ea_t);
        let tex_ea_t = ea_t.tex_code(&tex_symbols);
        let tex_diff_ea_t = diff_ea_t[0].tex_code(&tex_symbols);
        println!("{:?}", tex_ea_t);
        println!("{:?}", tex_diff_ea_t);
    }
}
