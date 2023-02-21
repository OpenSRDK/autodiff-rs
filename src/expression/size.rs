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

#[cfg(test)]
mod tests {
    use std::{
        collections::{HashMap, HashSet},
        ops::Add,
    };

    use opensrdk_linear_algebra::{sparse::SparseTensor, Matrix, Tensor};

    use crate::{new_variable, AbstractSize, Expression, Size};

    #[test]
    fn it_works1() {
        let a = 5.0f64;
        let b = vec![a; 8];
        let mut hash = HashMap::new();
        hash.insert(vec![3usize; 8], 2.0);
        hash.insert(vec![1usize; 8], 3.0);
        hash.insert(vec![4usize; 8], 4.0);
        hash.insert(vec![5usize; 8], 2.0);
        let c = SparseTensor::from(vec![6usize; 8], hash).unwrap();

        let ea = Expression::from(a);
        let eb = Expression::from(b);
        let ec = Expression::from(c);

        let sa = ea.sizes();
        let sb = eb.sizes();
        let sc = ec.sizes();

        assert_eq!(vec![Size::Many; 0], sa);
        assert_eq!(vec![Size::Many; 1], sb);
        assert_eq!(vec![Size::Many; 8], sc);
    }

    #[test]
    fn it_works2() {
        let id = "x";
        let ea = new_variable((id).to_string());
        let sa = ea.sizes();

        assert_eq!(vec![Size::Many; 0], sa);
    }
}
