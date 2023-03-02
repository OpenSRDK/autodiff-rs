use crate::MatrixExpression;
use std::collections::HashSet;

impl MatrixExpression {
    pub fn variable_ids(&self) -> HashSet<&str> {
        match self {
            MatrixExpression::T(v) => v.variable_ids(),
            MatrixExpression::Inv(v) => v.variable_ids(),
            MatrixExpression::Det(v) => v.variable_ids(),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use crate::{new_variable, new_variable_tensor, size, MatrixExpression, Size};

    #[test]
    fn it_works() {
        let id = "x";
        let a = HashSet::from([id; 1]);
        let ea = new_variable_tensor((id).to_string(), vec![Size::Many, Size::Many]);

        let ea_t = ea.clone().t();
        let a_t = ea_t.variable_ids();

        assert_eq!(a, a_t);

        let ea_inv = ea.clone().inv();
        let a_inv = ea_inv.variable_ids();

        assert_eq!(a, a_inv);

        let ea_det = ea.clone().det();
        let a_det = ea_det.variable_ids();

        assert_eq!(a, a_det);
    }
}
