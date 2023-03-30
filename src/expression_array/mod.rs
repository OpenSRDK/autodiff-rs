pub mod index;

pub use index::*;
use opensrdk_linear_algebra::indices_cartesian_product;

use crate::Expression;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct ExpressionArray {
    sizes: Vec<usize>,
    elems: HashMap<Vec<usize>, Expression>,
    default: Box<Expression>,
}

impl ExpressionArray {
    pub fn new(sizes: Vec<usize>) -> Self {
        Self {
            sizes,
            elems: HashMap::new(),
            default: Box::new(0.0.into()),
        }
    }

    pub fn from_factory(sizes: Vec<usize>, factory: impl Fn(&[usize]) -> Expression) -> Self {
        let mut elems = HashMap::new();
        let indices = indices_cartesian_product(&sizes);

        indices.iter().for_each(|index| {
            elems.insert(index.clone(), factory(&index));
        });

        Self {
            sizes,
            elems,
            default: Box::new(0.0.into()),
        }
    }

    pub fn sizes(&self) -> &[usize] {
        &self.sizes
    }

    pub fn elems(&self) -> &HashMap<Vec<usize>, Expression> {
        &self.elems
    }

    pub fn eject(self) -> (Vec<usize>, HashMap<Vec<usize>, Expression>) {
        (self.sizes, self.elems)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, HashSet};

    use opensrdk_linear_algebra::sparse::SparseTensor;

    use crate::{new_variable, Expression};

    #[test]
    fn it_works1() {
        let test_orig = vec![
            Expression::from(1f64),
            Expression::from(3f64),
            Expression::from(1f64),
            Expression::from(2f64),
        ];
        let factory = |i: &usize| test_orig[i];
        let sizes = vec![1usize, 2usize, 3usize, 4usize];
        let test = from_factory(sizes, factory);
    }
}
