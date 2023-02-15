use crate::Expression;
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
            Expression::Constant(v) => v.sizes().into_abstract_size(),
            Expression::PartialVariable(v) => v.sizes().into_abstract_size(),
            Expression::Add(l, _) => l.sizes(),
            Expression::Sub(l, _) => l.sizes(),
            Expression::Mul(l, _) => l.sizes(),
            Expression::Div(l, _) => l.sizes(),
            Expression::Neg(v) => v.sizes(),
            Expression::Transcendental(v) => v.sizes(),
            Expression::Tensor(v) => v.sizes(),
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

pub trait AbstractSize {
    fn into_abstract_size(&self) -> Vec<Size>;
}

impl AbstractSize for [usize] {
    fn into_abstract_size(&self) -> Vec<Size> {
        self.iter()
            .map(|&size| if size > 1 { Size::Many } else { Size::One })
            .collect()
    }
}
