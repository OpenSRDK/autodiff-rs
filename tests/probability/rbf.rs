use opensrdk_symbolic_computation::Expression;

pub fn rbf(x: Expression, x_prime: Expression, param: Expression) -> Expression {
    let diff = x - x_prime;

    (-diff.clone().dot(diff, &[[0, 0]]) / (2.0 * param.pow(2.0.into()))).exp()
}
