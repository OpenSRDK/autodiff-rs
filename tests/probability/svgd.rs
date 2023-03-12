use std::collections::HashMap;

use opensrdk_symbolic_computation::ConstantValue;

use super::{ContinuousDistribution, PositiveDefiniteKernel};

pub struct SteinVariationalGradientDescent<'a, D, K>
where
    D: ContinuousDistribution,
    K: PositiveDefiniteKernel,
{
    distribution: &'a D,
    kernel: &'a K,
    kernel_params: &'a [f64],
    samples: Vec<Vec<f64>>, // TODO: ContinuousSamplesDistribution
}

impl<'a, D, K> SteinVariationalGradientDescent<'a, D, K>
where
    D: ContinuousDistribution,
    K: PositiveDefiniteKernel,
{
    pub fn new(
        distribution: &'a D,
        kernel: &'a K,
        kernel_params: &'a [f64],
        samples: Vec<Vec<f64>>,
    ) -> Self {
        Self {
            distribution,
            kernel,
            kernel_params,
            samples,
        }
    }

    pub fn direction(&self, assignment: &HashMap<&str, ConstantValue>) -> Vec<f64> {
        todo!()
    }

    pub fn update_sample(
        &self,
        assignment: &HashMap<&str, ConstantValue>,
        step_size: f64,
    ) -> Vec<f64> {
        let direction = self.direction(assignment);
        todo!()
    }
}
