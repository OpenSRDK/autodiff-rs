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
                println!("v_det: {:?}", v_det);
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

#[cfg(test)]

mod tests {
    use std::collections::HashMap;

    use crate::{new_variable_tensor, MatrixExpression};
    #[test]
    fn it_works() {
        use crate::Size;

        let x = new_variable_tensor("x".to_string(), vec![Size::Many, Size::Many]);
        let mu = new_variable_tensor("mu".to_string(), vec![Size::Many, Size::Many]);
        let sigma = new_variable_tensor("sigma".to_string(), vec![Size::Many, Size::Many]);
        let expression = x * mu / sigma;
        let det = expression.clone().det();
        // let diff_x = MatrixExpression::diff_det(&det, &["x"]);
        // println!("diff_x: {:?}", diff_x);
        let tex_symbols: Vec<_> = vec![("x", "x"), ("mu", r"\mu"), ("sigma", r"\sigma")]
            .into_iter()
            .collect();
        let tex_symbols: HashMap<_, _> = tex_symbols.into_iter().collect();

        assert_eq!(
            expression.tex_code(&tex_symbols),
            r"{\left({{x} \times {\mu}}\right) / {\sigma}}"
        );
        // assert_eq!(
        //     diff_x.tex_code(&tex_symbols),
        //     r"{\left({\mu}}\right) / {\sigma}}"
        // );
        // assert_eq!(diff_mu.tex_code(&tex_symbols), r"\frac{x}{\Sigma}");
        // assert_eq!(
        //     diff_sigma.tex_code(&tex_symbols),
        //     r"\frac{-x \mu}{\Sigma^2}"
        // );
    }
}
