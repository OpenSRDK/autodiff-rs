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
