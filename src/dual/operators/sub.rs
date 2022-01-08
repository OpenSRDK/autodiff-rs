use crate::{Dual, Float};
use std::ops::Sub;

impl<T> Sub<Dual<T>> for Dual<T>
where
    T: Float,
{
    type Output = Self;

    fn sub(self, rhs: Dual<T>) -> Self::Output {
        Self::new(self.re - rhs.re, self.du - rhs.du)
    }
}
