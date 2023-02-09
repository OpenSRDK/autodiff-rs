use opensrdk_symbolic_computation::{Expression, TensorExpression, TranscendentalExpression};

extern crate blas_src;
extern crate lapack_src;
extern crate num_rational;
extern crate num_traits;
extern crate opensrdk_linear_algebra;
extern crate rayon;
extern crate thiserror;

#[test]
fn test_main() {
    let x = TensorExpression::Symbol("x".to_owned(), 1);
    let mu = TensorExpression::Symbol("mu".to_owned(), 1);
    let lsigma = TensorExpression::Symbol("lsigma".to_owned(), 2);
    let precision = TensorExpression::Symbol("precision".to_owned(), 2);

    let pdf_expression = (-0.5
        * Expression::TensorElement(
            (x.clone() - mu.clone())
                .inner_prod(precision, &[(0, 0)])
                .inner_prod((x.clone() - mu.clone()), &[(1, 0)])
                .into(),
            vec![0, 0],
        ))
    .exp();

    let diff = pdf_expression.differential(&["x", "mu", "precision"]);
    println!("x diff");
    println!("{:#?}", diff[0].rust_code());
    println!("mu diff");
    println!("{:#?}", diff[1].rust_code());
    println!("sigma diff");
    println!("{:#?}", diff[2].rust_code());
}
