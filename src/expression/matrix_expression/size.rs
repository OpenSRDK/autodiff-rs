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
    use std::{collections::HashMap, ops::Add};

    use opensrdk_linear_algebra::Matrix;

    use crate::{Expression, MatrixExpression, Size};

    #[test]
    fn it_works() {
        let len = 7usize;
        let a = Matrix::from(len, vec![1.0; len * len]).unwrap();
        let ea = Expression::from(a);
        let size = ea.sizes();

        let mea_mat_t = MatrixExpression::T(Box::new(ea.clone()));
        let size_t = mea_mat_t.sizes();

        assert_eq!(vec![size[1], size[0]], size_t);

        let mea_mat_inv = MatrixExpression::Inv(Box::new(ea.clone()));
        let size_inv = mea_mat_inv.sizes();

        assert_eq!(size, size_inv);

        let mea_mat_det = MatrixExpression::Det(Box::new(ea.clone()));
        let size_det = mea_mat_det.sizes();

        assert_eq!(vec![Size::One, Size::One], size_det);
    }
}
