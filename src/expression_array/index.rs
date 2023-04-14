use crate::{Expression, ExpressionArray};
use std::ops::{Index, IndexMut};

impl Index<&[usize]> for ExpressionArray {
    type Output = Expression;

    fn index(&self, indices: &[usize]) -> &Self::Output {
        self.elems().get(indices).unwrap_or(&self.default)
    }
}

impl IndexMut<&[usize]> for ExpressionArray {
    fn index_mut(&mut self, index: &[usize]) -> &mut Self::Output {
        self.elems
            .entry(index.to_vec())
            .or_insert(*self.default.clone())
    }
}

#[cfg(test)]
mod tests {

    use crate::{Expression, ExpressionArray};

    #[test]
    fn it_works() {
        let mut v1 = ExpressionArray::new(vec![2, 3]);
        v1[&[0, 0]] = 1.0.into();
        v1[&[0, 1]] = 2.0.into();
        v1[&[0, 2]] = 3.0.into();

        assert_eq!(v1.sizes(), &[2, 3]);
        assert_eq!(v1[&([0, 0])], 1.0.into());
        assert_eq!(v1[&([0, 1])], 2.0.into());
        assert_eq!(v1[&([0, 2])], 3.0.into());

        let v2 = ExpressionArray::from_factory(vec![2, 3], |index| {
            let sum_index = index.iter().sum::<usize>();
            Expression::from(sum_index as f64)
        });
        assert_eq!(v2.sizes(), &[2, 3]);
        assert_eq!(v2.elems().len(), 6);
    }
}
