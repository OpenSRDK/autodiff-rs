use crate::Float;

pub mod functions;
pub mod operators;

pub use functions::*;
pub use operators::*;

pub struct Dual<T>
where
    T: Float,
{
    re: T,
    du: T,
}

impl<T> Dual<T>
where
    T: Float,
{
    pub fn new(re: T, du: T) -> Self {
        Self { re, du }
    }
}
