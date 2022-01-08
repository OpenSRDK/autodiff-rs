use crate::{Dual, Float};
use std::ops::Neg;

impl<T> Neg for Dual<T>
where
    T: Float,
{
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.re, -self.du)
    }
}
