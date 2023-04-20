use super::DistributionProduct;
use super::MultivariateNormal;
use super::PositiveDefiniteKernel;
use super::RBF;
use opensrdk_linear_algebra::Matrix;
use opensrdk_linear_algebra::Vector;
use opensrdk_symbolic_computation::new_partial_variable;
use opensrdk_symbolic_computation::new_variable_tensor;
use opensrdk_symbolic_computation::ExpressionArray;
use opensrdk_symbolic_computation::Size;
use opensrdk_symbolic_computation::{new_variable, Expression};
use std::iter::once;
use std::ops::Index;

// #[test]
fn test_gp() {
    let n = 20usize;
    let xd = 4usize;
    let y = (0..n).map(|yi| yi as f64).collect::<Vec<_>>();
    let y_mean = y.iter().sum::<f64>() / y.len() as f64;
    let x = vec![vec![1.0; xd]; n];
    let sigma = new_variable("sigma".to_string());
    let param = new_variable("theta".to_string());
    let kernel = RBF;

    let k = new_partial_variable(ExpressionArray::from_factory(vec![n, n], |index| {
        let i = index[0];
        let j = index[1];
        kernel.expression(x[i].clone().into(), x[j].clone().into(), &[param.clone()])
    }));

    let normal = MultivariateNormal::new(y.into(), y_mean.into(), k + sigma, n);
}

fn test_recurrent_gp() {
    let n = 20usize;
    let yd = 2usize;
    let ud = 3usize;
    let xd = 4usize;
    let y = (0..n)
        .flat_map(|yi| vec![yi as f64; yd].into_iter())
        .collect::<Vec<_>>();
    let y_mean = y.iter().map(|yij| yij).sum::<f64>() / y.len() as f64;
    let u = ExpressionArray::from_factory(vec![n, ud], |indices| {
        new_variable(format!("u_{{{}, {}}}", indices[0], indices[1]))
    });
    let x = vec![vec![1.0; xd]; n];
    let cy = new_variable_tensor("cy".to_string(), vec![Size::Many, Size::Many]);
    let sigma_y = new_variable("sigma_y".to_string());
    let theta_y = new_variable("theta_y".to_string());
    let cu = new_variable_tensor("cu".to_string(), vec![Size::Many, Size::Many]);
    let sigma_u = new_variable("sigma_u".to_string());
    let theta_u = new_variable("theta_u".to_string());
    let kernel = RBF;

    let ky = new_partial_variable(ExpressionArray::from_factory(vec![n, n], |index| {
        let i = index[0];
        let j = index[1];
        kernel.expression(
            new_partial_variable(ExpressionArray::from_factory(vec![1, ud], |indices| {
                u.index(&[i, indices[1]]).clone()
            })),
            new_partial_variable(ExpressionArray::from_factory(vec![1, ud], |indices| {
                u.index(&[j, indices[1]]).clone()
            })),
            &[theta_y.clone()],
        )
    }));
    let ku = new_partial_variable(ExpressionArray::from_factory(vec![n, n], |index| {
        let i = index[0];
        let j = index[1];
        kernel.expression(
            new_partial_variable(ExpressionArray::from_factory(vec![1, ud + xd], |indices| {
                if indices[1] < ud {
                    u.index(&[i, indices[1]]).clone()
                } else {
                    x[i][index[1]].into()
                }
            })),
            new_partial_variable(ExpressionArray::from_factory(vec![1, ud + xd], |indices| {
                if indices[1] < ud {
                    u.index(&[j, indices[1]]).clone()
                } else {
                    x[j][index[1]].into()
                }
            })),
            &[theta_u.clone()],
        )
    }));

    let normal = MultivariateNormal::new(y.into(), y_mean.into(), (ky + sigma_y).direct(cy), yd)
        * MultivariateNormal::new(
            new_partial_variable(u),
            0.0.into(),
            (ku + sigma_u).direct(cu),
            n,
        );
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_gp() {
//         let n = 20usize;
//         let xd = 4usize;
//         let y = (0..n).map(|yi| yi as f64).collect::<Vec<_>>();
//         let y_mean = y.iter().sum::<f64>() / y.len() as f64;
//         let x = vec![vec![1.0; xd]; n];
//         let sigma = new_variable("sigma".to_string());
//         let param = new_variable("theta".to_string());
//         let kernel = RBF;

//         let k = new_partial_variable(ExpressionArray::from_factory(vec![n, n], |index| {
//             let i = index[0];
//             let j = index[1];
//             kernel.expression(x[i].clone().into(), x[j].clone().into(), &[param.clone()])
//         }));

//         let normal = MultivariateNormal::new(y.into(), y_mean.into(), k + sigma, n);
//     }

//     #[test]
//     fn test_recurrent_gp() {
//         let n = 20usize;
//         let yd = 2usize;
//         let ud = 3usize;
//         let xd = 4usize;
//         let y = (0..n)
//             .flat_map(|yi| vec![yi as f64; yd].into_iter())
//             .collect::<Vec<_>>();
//         let y_mean = y.iter().map(|yij| yij).sum::<f64>() / y.len() as f64;
//         let u = ExpressionArray::from_factory(vec![n, ud], |indices| {
//             new_variable(format!("u_{{{}, {}}}", indices[0], indices[1]))
//         });
//         let x = vec![vec![1.0; xd]; n];
//         let cy = new_variable_tensor("cy".to_string(), vec![Size::Many, Size::Many]);
//         let sigma_y = new_variable("sigma_y".to_string());
//         let theta_y = new_variable("theta_y".to_string());
//         let cu = new_variable_tensor("cu".to_string(), vec![Size::Many, Size::Many]);
//         let sigma_u = new_variable("sigma_u".to_string());
//         let theta_u = new_variable("theta_u".to_string());
//         let kernel = RBF;

//         let ky = new_partial_variable(ExpressionArray::from_factory(vec![n, n], |index| {
//             let i = index[0];
//             let j = index[1];
//             kernel.expression(
//                 new_partial_variable(ExpressionArray::from_factory(vec![1, ud], |indices| {
//                     u.index(&[i, indices[1]]).clone()
//                 })),
//                 new_partial_variable(ExpressionArray::from_factory(vec![1, ud], |indices| {
//                     u.index(&[j, indices[1]]).clone()
//                 })),
//                 &[theta_y.clone()],
//             )
//         }));
//         let ku = new_partial_variable(ExpressionArray::from_factory(vec![n, n], |index| {
//             let i = index[0];
//             let j = index[1];
//             kernel.expression(
//                 new_partial_variable(ExpressionArray::from_factory(vec![1, ud + xd], |indices| {
//                     if indices[1] < ud {
//                         u.index(&[i, indices[1]]).clone()
//                     } else {
//                         x[i][index[1]].into()
//                     }
//                 })),
//                 new_partial_variable(ExpressionArray::from_factory(vec![1, ud + xd], |indices| {
//                     if indices[1] < ud {
//                         u.index(&[j, indices[1]]).clone()
//                     } else {
//                         x[j][index[1]].into()
//                     }
//                 })),
//                 &[theta_u.clone()],
//             )
//         }));

//         let normal =
//             MultivariateNormal::new(y.into(), y_mean.into(), (ky + sigma_y).direct(cy), yd)
//                 * MultivariateNormal::new(
//                     new_partial_variable(u),
//                     0.0.into(),
//                     (ku + sigma_u).direct(cu),
//                     n,
//                 );

//         let mut variables = normal.variables();
//         variables.sort();
//         assert_eq!(
//             variables,
//             vec![
//                 "cy".to_string(),
//                 "cu".to_string(),
//                 "sigma_u".to_string(),
//                 "sigma_y".to_string(),
//                 "theta_u".to_string(),
//                 "theta_y".to_string(),
//                 "u_{0, 0}".to_string(),
//                 "u_{0, 1}".to_string(),
//                 "u_{0, 2}".to_string(),
//                 "u_{1, 0}".to_string(),
//                 "u_{1, 1}".to_string(),
//                 "u_{1, 2}".to_string(),
//                 "u_{10, 0}".to_string(),
//                 "u_{10, 1}".to_string(),
//                 "u_{10, 2}".to_string(),
//                 "u_{11, 0}".to_string(),
//                 "u_{11, 1}".to_string(),
//                 "u_{11, 2}".to_string(),
//                 "u_{12, 0}".to_string(),
//                 "u_{12, 1}".to_string(),
//                 "u_{12, 2}".to_string(),
//                 "u_{13, 0}".to_string(),
//                 "u_{13, 1}".to_string(),
//                 "u_{13, 2}".to_string(),
//                 "u_{14, 0}".to_string(),
//                 "u_{14, 1}".to_string(),
//                 "u_{14, 2}".to_string(),
//                 "u_{15, 0}".to_string(),
//                 "u_{15, 1}".to_string(),
//                 "u_{15, 2}".to_string(),
//                 "u_{16, 0}".to_string(),
//                 "u_{16, 1}".to_string(),
//                 "u_{16, 2}".to_string(),
//                 "u_{17, 0}".to_string(),
//                 "u_{17, 1}".to_string(),
//                 "u_{17, 2}".to_string(),
//                 "u_{18, 0}".to_string(),
//                 "u_{18, 1}".to_string(),
//                 "u_{18, 2}".to_string(),
//                 "u_{19, 0}".to_string(),
//                 "u_{19, 1}".to_string(),
//                 "u_{19, 2}".to_string(),
//             ]
//         );
//     }
// }
