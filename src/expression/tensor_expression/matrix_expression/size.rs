use crate::{MatrixExpression, Size};

impl MatrixExpression {
    pub fn sizes(&self) -> Vec<Size> {
        match self {
            MatrixExpression::Mat(v) => v.sizes(),
            MatrixExpression::Constant(v) => vec![
                if v.rows() > 1 { Size::Many } else { Size::One },
                if v.cols() > 1 { Size::Many } else { Size::One },
            ],
            MatrixExpression::Inv(v) => v.sizes(),
            MatrixExpression::Det(v) => vec![Size::One, Size::One],
        }
    }
}
