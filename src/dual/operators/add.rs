use crate::{Dual, Float};
use std::ops::Add;

impl<T> Add<Dual<T>> for Dual<T>
where
    T: Float,
{
    type Output = Self;

    fn add(self, rhs: Dual<T>) -> Self::Output {
        Self::new(self.re + rhs.re, self.du + rhs.du)
    }
}
