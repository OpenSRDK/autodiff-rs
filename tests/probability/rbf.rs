use opensrdk_linear_algebra::Vector;
use opensrdk_symbolic_computation::Expression;

pub fn rbf(x: &Vec<f64>, x_prime: &Vec<f64>, param: Expression) -> Expression {
    let diff = x.clone().col_mat() - x_prime.clone().col_mat();

    (-(diff.t() * diff)[(0, 0)] / (2.0 * param.pow(2.0.into()))).exp()
}
