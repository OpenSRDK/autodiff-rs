use crate::{MatrixExpression, Size};

impl MatrixExpression {
    pub fn sizes(&self) -> Vec<Size> {
        match self {
            MatrixExpression::T(v) => {
                let sizes = v.sizes();
                vec![sizes[1], sizes[0]]
            }
            MatrixExpression::Inv(v) => v.sizes(),
            MatrixExpression::Det(_) => vec![Size::One, Size::One],
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, collections::HashSet, ops::Add};

    use opensrdk_linear_algebra::Matrix;

    use crate::{new_variable, new_variable_tensor, Expression, MatrixExpression, Size};

    #[test]
    fn it_works() {
        let id = "x";
        let ea = new_variable_tensor((id).to_string(), vec![Size::Many, Size::Many]);

        let size = ea.sizes();

        let ea_t = ea.clone().t();
        let size_t = ea_t.sizes();

        assert_eq!(vec![size[1], size[0]], size_t);

        let ea_inv = ea.clone().inv();
        let size_inv = ea_inv.sizes();

        assert_eq!(size, size_inv);

        let ea_det = ea.clone().det();
        let size_det = ea_det.sizes();

        assert_eq!(vec![Size::One, Size::One], size_det);
    }
}
