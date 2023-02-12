use std::collections::HashMap;

use opensrdk_symbolic_computation::{new_symbol_tensor, Expression, Size};

extern crate blas_src;
extern crate lapack_src;
extern crate opensrdk_linear_algebra;
extern crate rayon;
extern crate ron;
extern crate serde;
extern crate thiserror;

#[test]
fn test_main() {
    let x = new_symbol_tensor("x".to_owned(), vec![Size::Many]);
    let mu = new_symbol_tensor("mu".to_owned(), vec![Size::Many]);
    let _lsigma = new_symbol_tensor("lsigma".to_owned(), vec![Size::Many; 2]);
    let precision = new_symbol_tensor("lambda".to_owned(), vec![Size::Many; 2]);

    let pdf_expression = (-0.5
        * ((x.clone() - mu.clone())
            .inner_prod(precision, &[[0, 0]])
            .inner_prod(x.clone() - mu.clone(), &[[1, 0]]))
        .as_degenerated_tensor_scalar())
    .exp();

    let diff = pdf_expression.differential(&["x", "mu", "lambda"]);
    let tex_symbols = vec![("x", "x"), ("mu", r"\mu"), ("lambda", r"\Lambda")]
        .into_iter()
        .collect();

    println!("x diff");
    println!("{:#?}", diff[0].tex_code(&tex_symbols));
    println!("mu diff");
    println!("{:#?}", diff[1].tex_code(&tex_symbols));
    println!("sigma diff");
    println!("{:#?}", diff[2].tex_code(&tex_symbols));
}

#[test]
fn test_main2() {
    let x = new_symbol_tensor("x".to_owned(), vec![Size::Many, Size::One]);
    let mu = new_symbol_tensor("mu".to_owned(), vec![Size::Many, Size::One]);
    let _lsigma = new_symbol_tensor("lsigma".to_owned(), vec![Size::Many; 2]);
    let precision = new_symbol_tensor("lambda".to_owned(), vec![Size::Many; 2]);

    let pdf_expression = (-0.5
        * ((x.clone() - mu.clone())
            .to_mat()
            .t()
            .as_tensor()
            .inner_prod(precision, &[[1, 0]])
            .inner_prod(x.clone() - mu.clone(), &[[1, 0]]))
        .as_degenerated_tensor_scalar())
    .exp();

    let diff = pdf_expression.differential(&["x", "mu", "lambda"]);
    let tex_symbols = vec![("x", "x"), ("mu", r"\mu"), ("lambda", r"\Lambda")]
        .into_iter()
        .collect();

    println!("x diff");
    println!("{:#?}", diff[0].tex_code(&tex_symbols));
    println!("mu diff");
    println!("{:#?}", diff[1].tex_code(&tex_symbols));
    println!("sigma diff");
    println!("{:#?}", diff[2].tex_code(&tex_symbols));
}

trait Distribution<T> {
    fn ln_pdf(&self, value: &T, condition: &HashMap<String, Expression>) -> Expression;

    fn pdf(&self, value: &T, condition: &HashMap<String, Expression>) -> Expression {
        self.ln_pdf(value, condition).exp()
    }
}

struct MultivariateNormal {
    x: String,
    mu: String,
    sigma: String,
}

impl MultivariateNormal {
    pub fn new(x: String, mu: String, sigma: String) -> MultivariateNormal {
        MultivariateNormal { x, mu, sigma }
    }
}

impl Distribution<Vec<f64>> for MultivariateNormal {
    fn ln_pdf(&self, value: &Vec<f64>, condition: &HashMap<String, Expression>) -> Expression {
        let x = &condition[&self.x.clone()];
        let mu = condition[&self.mu.clone()];
        let sigma = &condition[&self.sigma.clone()];

        let pdf_expression = (-0.5
            * ((x.clone() - mu.clone())
                .inner_prod(precision, &[[0, 0]])
                .inner_prod(x.clone() - mu.clone(), &[[1, 0]]))
            .as_scalar())
        .exp();

        let res: Result<usize, usize> = Ok(0);

        res.pdf_expression
    }
}
