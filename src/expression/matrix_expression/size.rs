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

    use opensrdk_linear_algebra::{Matrix, Tensor};

    use crate::{AbstractSize, Expression};

    #[test]
    fn it_works() {
        let len = 7usize;
        let a = Matrix::from(len, vec![1.0; len * len]).unwrap();
        let a_sizes = vec![a.rows(), a.cols()].into_abstract_size();
        let ea_size = Expression::from(a).sizes();
        assert_eq!(a_sizes, ea_size);
    }
}
