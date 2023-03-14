use std::ops::{Add, Mul};

use super::{KernelAdd, KernelMul, PositiveDefiniteKernel};
use opensrdk_symbolic_computation::Expression;

pub struct RBF;

impl PositiveDefiniteKernel for RBF {
    fn expression(&self, x: Expression, x_prime: Expression, params: &[Expression]) -> Expression {
        let diff = x - x_prime;

        (-diff.clone().dot(diff, &[[0, 0]]) / (2.0 * params[0].clone().pow(2.0.into()))).exp()
    }

    fn params_len(&self) -> usize {
        1
    }
}

impl<R> Add<R> for RBF
where
    R: PositiveDefiniteKernel,
{
    type Output = KernelAdd<Self, R>;

    fn add(self, rhs: R) -> Self::Output {
        KernelAdd::new(self, rhs)
    }
}

impl<R> Mul<R> for RBF
where
    R: PositiveDefiniteKernel,
{
    type Output = KernelMul<Self, R>;

    fn mul(self, rhs: R) -> Self::Output {
        KernelMul::new(self, rhs)
    }
}
