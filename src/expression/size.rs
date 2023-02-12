use crate::{ConstantValue, Expression};
use opensrdk_linear_algebra::Tensor;
use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
pub enum Size {
    One,
    Many,
}

impl Expression {
    pub fn sizes(&self) -> Vec<Size> {
        match self {
            Expression::Variable(_, sizes) => sizes.clone(),
            Expression::Constant(v) => match v {
                ConstantValue::Scalar(v) => vec![],
                ConstantValue::Matrix(v) => vec![
                    if v.rows() > 1 { Size::Many } else { Size::One },
                    if v.cols() > 1 { Size::Many } else { Size::One },
                ],
                ConstantValue::Tensor(v) => (0..v.rank())
                    .into_iter()
                    .map(|rank| v.size(rank))
                    .map(|s| if s > 1 { Size::Many } else { Size::One })
                    .collect(),
            },
            Expression::Add(l, r) => l.sizes(),
            Expression::Sub(l, r) => l.sizes(),
            Expression::Mul(l, r) => l.sizes(),
            Expression::Div(l, r) => l.sizes(),
            Expression::Neg(v) => v.sizes(),
            Expression::Transcendental(v) => v.sizes(),
            Expression::Tensor(v) => v.sizes(),
            Expression::IndexedTensor(v) => todo!(),
            Expression::Matrix(v) => v.sizes(),
        }
    }

    pub fn is_same_size(&self, other: &Expression) -> bool {
        let sl = self.sizes();
        let sr = other.sizes();

        if sl.len() == 0 || sr.len() == 0 {
            return true;
        }

        sl == sr
    }

    pub fn not_1dimension_ranks(&self) -> usize {
        self.sizes().iter().filter(|&d| *d != Size::One).count()
    }
}
