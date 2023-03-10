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
