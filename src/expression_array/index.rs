use crate::{Expression, ExpressionArray};
use std::ops::Index;

impl Index<&[usize]> for ExpressionArray {
    type Output = Expression;

    fn index(&self, indices: &[usize]) -> &Self::Output {
        self.elems().get(indices).unwrap_or(&self.default)
    }
}

#[cfg(test)]
mod tests {
    use std::ops::Index;

    use crate::{Expression, ExpressionArray};

    #[test]
    fn it_works() {
        let v1 = ExpressionArray::new(vec![2, 3]);
        let index = v1.index(&([0, 0]));
        println!("{:?}", index);
        assert_eq!(v1.sizes(), &[2, 3]);
        assert_eq!(v1[&([0, 0])], 0.0.into());

        let v2 = ExpressionArray::from_factory(vec![2, 3], |index| {
            let sum_index = index.iter().sum::<usize>();
            Expression::from(sum_index as f64)
        });
        assert_eq!(v2.sizes(), &[2, 3]);
        assert_eq!(v2.elems().len(), 6);
    }
}
