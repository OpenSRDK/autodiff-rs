use super::{ContinuousDistribution, JointDistribution};
use opensrdk_symbolic_computation::{new_variable_tensor, Expression, Size};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, f64::consts::PI, ops::Mul};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MultivariateNormal {
    output_id: String,
    mu: Expression,
    sigma: Expression,
    d: usize,
}

impl MultivariateNormal {
    pub fn new(
        output_id: String,
        mu: Expression,
        sigma: Expression,
        d: usize,
    ) -> MultivariateNormal {
        MultivariateNormal {
            output_id,
            mu,
            sigma,
            d,
        }
    }
}

impl<Rhs> Mul<Rhs> for MultivariateNormal
where
    Rhs: ContinuousDistribution,
{
    type Output = JointDistribution<Self, Rhs>;

    fn mul(self, rhs: Rhs) -> Self::Output {
        JointDistribution::new(self, rhs)
    }
}

impl ContinuousDistribution for MultivariateNormal {
    fn value_ids(&self) -> HashSet<&str> {
        HashSet::from([self.output_id.as_str()])
    }

    fn conditions(&self) -> Vec<&Expression> {
        vec![&self.mu, &self.sigma]
    }

    fn pdf(&self) -> Expression {
        let x = new_variable_tensor(self.output_id.clone(), vec![Size::Many, Size::One]);
        let mu = self.mu.clone();
        let sigma = self.sigma.clone();
        let d = self.d as f64;

        let pdf_expression = (2.0 * PI).powf(-0.5 * d)
            * sigma.clone().det().pow((-0.5).into())
            * (-0.5
                * ((x.clone() - mu.clone())
                    .dot(sigma.inv(), &[[0, 0]])
                    .dot(x.clone() - mu.clone(), &[[1, 0]])))
            .exp();

        pdf_expression
    }
}
