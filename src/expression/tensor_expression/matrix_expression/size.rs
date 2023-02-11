use crate::{MatrixExpression, Size};

impl MatrixExpression {
    pub fn sizes(&self) -> Vec<Size> {
        match self {
            MatrixExpression::Mat(v) => v.sizes(),
            MatrixExpression::Constant(v) => vec![
                if v.rows() > 1 { Size::Many } else { Size::One },
                if v.cols() > 1 { Size::Many } else { Size::One },
            ],
            MatrixExpression::T(v) => {
                let sizes = v.sizes();
                vec![sizes[1], sizes[0]]
            }
            MatrixExpression::Inv(v) => v.sizes(),
            MatrixExpression::Det(_) => vec![Size::One, Size::One],
        }
    }
}
