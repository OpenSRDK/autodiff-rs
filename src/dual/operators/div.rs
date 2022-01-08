use crate::{Dual, Float};
use std::ops::Div;

impl<T> Div<Dual<T>> for Dual<T>
where
    T: Float,
{
    type Output = Self;

    fn div(self, rhs: Dual<T>) -> Self::Output {
        Self::new(
            self.re / rhs.re,
            (self.du * rhs.re - self.re * rhs.du) / (rhs.re * rhs.re),
        )
    }
}
