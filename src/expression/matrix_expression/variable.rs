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

    use crate::{new_variable, MatrixExpression};

    #[test]
    fn it_works() {
        let id = "x";
        let a = HashSet::from([id; 1]);
        let ea = new_variable((id).to_string());
        let mea_t = MatrixExpression::T(Box::new(ea.clone()));
        let a_t = mea_t.variable_ids();

        assert_eq!(a, a_t);

        let mea_inv = MatrixExpression::Inv(Box::new(ea.clone()));
        let a_inv = mea_inv.variable_ids();

        assert_eq!(a, a_inv);

        let mea_det = MatrixExpression::Det(Box::new(ea.clone()));
        let a_det = mea_det.variable_ids();

        assert_eq!(a, a_det);
    }
}
