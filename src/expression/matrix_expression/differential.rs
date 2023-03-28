use crate::{Expression, MatrixExpression};

impl MatrixExpression {
    pub fn differential(&self, variable_ids: &[&str]) -> Vec<Expression> {
        match self {
            MatrixExpression::T(v) => MatrixExpression::diff_t(v, variable_ids),
            MatrixExpression::Inv(v) => MatrixExpression::diff_inv(v, variable_ids),
            MatrixExpression::Det(v) => MatrixExpression::diff_det(v, variable_ids),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::new_variable_tensor;
    #[test]
    fn it_works() {
        use crate::Size;

        let x = new_variable_tensor("x".to_string(), vec![Size::Many, Size::Many]);
        let mu = new_variable_tensor("mu".to_string(), vec![Size::Many, Size::Many]);
        let sigma = new_variable_tensor("sigma".to_string(), vec![Size::Many, Size::Many]);
        let expression = x * mu / sigma;
        let diff_x = expression.clone().differential(&["x"]);
        println!("diff: {:?}", diff_x);
    }
}
