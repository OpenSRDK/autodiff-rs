use std::{
    fmt::Debug,
    iter::Product,
    iter::Sum,
    ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign},
};

pub trait Float:
    Clone
    + Copy
    + Debug
    + Default
    + PartialEq
    + Send
    + Sized
    + Sync
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Mul<Output = Self>
    + MulAssign
    + Div
    + DivAssign
    + Neg<Output = Self>
    + Sum
    + Product
{
}

impl Float for f32 {}

impl Float for f64 {}
