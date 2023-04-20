use opensrdk_symbolic_computation::Expression;

pub trait PositiveDefiniteKernel {
    fn expression(&self, x: Expression, x_prime: Expression, params: &[Expression]) -> Expression;
    fn params_len(&self) -> usize;
}

#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use opensrdk_symbolic_computation::{new_variable, ConstantValue, Expression};

    #[test]
    fn test_kernel_add() {
        #[derive(Debug)]
        struct Kernel1;
        impl PositiveDefiniteKernel for Kernel1 {
            fn expression(
                &self,
                x: Expression,
                x_prime: Expression,
                params: &[Expression],
            ) -> Expression {
                x + x_prime + Expression::Constant(ConstantValue::Scalar(1.0))
            }

            fn params_len(&self) -> usize {
                1
            }
        }

        let kernel1_id = "x";
        let kernel1 = new_variable((kernel1_id).to_string());
        let kernel2_id = "x'";
        let kernel2 = new_variable((kernel2_id).to_string());

        let kernel_1 = PositiveDefiniteKernel::expression(
            &Kernel1,
            Expression::from(kernel1.clone()),
            Expression::from(kernel2.clone()),
            &[Expression::Constant(ConstantValue::Scalar(1.0))],
        );

        println!("kernel1: {:?}", kernel1);

        #[derive(Debug)]
        struct Kernel2;
        impl PositiveDefiniteKernel for Kernel2 {
            fn expression(
                &self,
                x: Expression,
                x_prime: Expression,
                params: &[Expression],
            ) -> Expression {
                x * x_prime * Expression::Constant(ConstantValue::Scalar(1.0))
            }

            fn params_len(&self) -> usize {
                1
            }
        }

        let kernel_2 = PositiveDefiniteKernel::expression(
            &Kernel2,
            Expression::from(kernel1),
            Expression::from(kernel2),
            &[Expression::Constant(ConstantValue::Scalar(1.0))],
        );

        let kernel = KernelAdd::new(Kernel1, Kernel2);
        println!("kernel: {:#?}", kernel);

        let lhs_kernel_id = "x + x' + 1 +  x * x' * 1";
        let lhs_kernel = new_variable((lhs_kernel_id).to_string());
        assert_eq!(
            kernel.expression(
                Expression::from(kernel_1),
                Expression::from(kernel_2),
                &[Expression::Constant(ConstantValue::Scalar(1.0))]
            ),
            Expression::from(lhs_kernel),
        );
    }

    // #[test]
    // fn test_kernel_mul() {
    //     struct Kernel1;
    //     impl PositiveDefiniteKernel for Kernel1 {
    //         fn expression(
    //             &self,
    //             x: Expression,
    //             x_prime: Expression,
    //             params: &[Expression],
    //         ) -> Expression {
    //             x + x_prime + params[0]
    //         }

    //         fn params_len(&self) -> usize {
    //             1
    //         }
    //     }

    //     struct Kernel2;
    //     impl PositiveDefiniteKernel for Kernel2 {
    //         fn expression(
    //             &self,
    //             x: Expression,
    //             x_prime: Expression,
    //             params: &[Expression],
    //         ) -> Expression {
    //             x * x_prime * params[0]
    //         }

    //         fn params_len(&self) -> usize {
    //             1
    //         }
    //     }

    //     let kernel = KernelMul::new(Kernel1, Kernel2);
    //     assert_eq!(
    //         kernel.expression(
    //             Expression::from("x"),
    //             Expression::from("x'"),
    //             &[Expression::from(1)]
    //         ),
    //         Expression::from("(x + x' + 1) * (x * x' * 1)")
    //     );
    // }
}
