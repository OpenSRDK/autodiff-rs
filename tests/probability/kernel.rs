use opensrdk_symbolic_computation::Expression;

pub trait PositiveDefiniteKernel {
    fn expression(&self, x: Expression, x_prime: Expression, params: &[Expression]) -> Expression;
    fn params_len(&self) -> usize;
}

pub struct KernelAdd<L, R>
where
    L: PositiveDefiniteKernel,
    R: PositiveDefiniteKernel,
{
    lhs: L,
    rhs: R,
}

impl<L, R> KernelAdd<L, R>
where
    L: PositiveDefiniteKernel,
    R: PositiveDefiniteKernel,
{
    pub fn new(lhs: L, rhs: R) -> Self {
        Self { lhs, rhs }
    }
}

impl<L, R> PositiveDefiniteKernel for KernelAdd<L, R>
where
    L: PositiveDefiniteKernel,
    R: PositiveDefiniteKernel,
{
    fn expression(&self, x: Expression, x_prime: Expression, params: &[Expression]) -> Expression {
        self.lhs
            .expression(x.clone(), x_prime.clone(), &params[..self.lhs.params_len()])
            + self
                .rhs
                .expression(x, x_prime, &params[self.lhs.params_len()..])
    }

    fn params_len(&self) -> usize {
        self.lhs.params_len() + self.rhs.params_len()
    }
}

pub struct KernelMul<L, R>
where
    L: PositiveDefiniteKernel,
    R: PositiveDefiniteKernel,
{
    lhs: L,
    rhs: R,
}

impl<L, R> KernelMul<L, R>
where
    L: PositiveDefiniteKernel,
    R: PositiveDefiniteKernel,
{
    pub fn new(lhs: L, rhs: R) -> Self {
        Self { lhs, rhs }
    }
}

impl<L, R> PositiveDefiniteKernel for KernelMul<L, R>
where
    L: PositiveDefiniteKernel,
    R: PositiveDefiniteKernel,
{
    fn expression(&self, x: Expression, x_prime: Expression, params: &[Expression]) -> Expression {
        self.lhs
            .expression(x.clone(), x_prime.clone(), &params[..self.lhs.params_len()])
            * self
                .rhs
                .expression(x, x_prime, &params[self.lhs.params_len()..])
    }

    fn params_len(&self) -> usize {
        self.lhs.params_len() + self.rhs.params_len()
    }
}
