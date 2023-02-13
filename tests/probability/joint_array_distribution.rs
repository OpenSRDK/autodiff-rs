use super::ContinuousDistribution;

pub struct JointArrayDistribution<D> {
    distributions: Vec<D>,
}

pub trait DistributionProduct<D>
where
    D: ContinuousDistribution,
{
    /// p(x|a) = Π p(xi|ai)
    fn product(self) -> JointArrayDistribution<D>;
}

impl<I, D> DistributionProduct<D> for I
where
    I: Iterator<Item = D>,
    D: ContinuousDistribution,
{
    fn product(self) -> JointArrayDistribution<D> {
        let distributions = self.collect::<Vec<_>>();

        JointArrayDistribution { distributions }
    }
}
