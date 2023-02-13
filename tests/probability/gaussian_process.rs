use super::rbf;
use super::MultivariateNormal;
use opensrdk_symbolic_computation::{new_variable, Expression};

// #[test]
fn test_gp() {
    let y = (0..20).map(|yi| yi as f64).collect::<Vec<_>>();
    let y_mean = y.iter().sum::<f64>() / y.len() as f64;
    let x = vec![vec![1.0; 4]; 20];
    let sigma = new_variable("sigma".to_string());
    let param = new_variable("theta".to_string());

    let k = Expression::IndexedTensor(
        (0..20)
            .flat_map(|i| (0..20).map(move |j| (i, j)))
            .map(|(i, j)| {
                (
                    vec![i, j],
                    rbf(x[i].clone().into(), x[j].clone().into(), param.clone()),
                )
            })
            .collect(),
    );

    let normal = MultivariateNormal::new("y".to_string(), y_mean.into(), k + sigma, 20);
}
