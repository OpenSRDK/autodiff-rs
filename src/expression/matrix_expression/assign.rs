use crate::{ConstantValue, Expression, MatrixExpression};
use std::collections::HashMap;

impl MatrixExpression {
    pub fn assign(self, variables: &HashMap<&str, ConstantValue>) -> Expression {
        match self {
            MatrixExpression::T(v) => v.assign(variables).t(),
            MatrixExpression::Inv(v) => v.assign(variables).inv(),
            MatrixExpression::Det(v) => v.assign(variables).det(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use opensrdk_linear_algebra::{sparse::SparseTensor, Matrix};

    use crate::{
        new_variable, new_variable_tensor, AbstractSize, ConstantValue, Expression,
        MatrixExpression, Size,
    };

    #[test]
    fn it_works1() {
        let id = "x";
        let ea = new_variable_tensor((id).to_string(), vec![Size::Many, Size::Many]);

        let ea_t = ea.clone().t();
        let mut hash1 = HashMap::new();

        let len = 7usize;
        let a = Matrix::from(len, vec![1.0; len * len]).unwrap();

        hash1.insert(id, ConstantValue::Matrix(a.clone()));

        let result = ea_t.assign(&hash1);

        assert_eq!(result, Expression::from(ConstantValue::Matrix(a.t())))
    }
}
