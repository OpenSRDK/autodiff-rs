use crate::{Expression, ExpressionArray};
use std::ops::Index;

impl Index<&[usize]> for ExpressionArray {
    type Output = Expression;

    fn index(&self, indices: &[usize]) -> &Self::Output {
        self.elems().get(indices).unwrap_or(&self.default)
    }
}
