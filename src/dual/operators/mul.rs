use crate::{Dual, Float};
use std::ops::Mul;

impl<T> Mul<Dual<T>> for Dual<T>
where
    T: Float,
{
    type Output = Self;

    fn mul(self, rhs: Dual<T>) -> Self::Output {
        Self::new(self.re * rhs.re, self.du * rhs.re + self.re * rhs.du)
    }
}
