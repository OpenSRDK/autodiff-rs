use opensrdk_symbolic_computation::{new_symbol_tensor, Size};

extern crate blas_src;
extern crate lapack_src;
extern crate num_rational;
extern crate num_traits;
extern crate opensrdk_linear_algebra;
extern crate rayon;
extern crate thiserror;

#[test]
fn test_main() {
    let tex = true;

    let x = new_symbol_tensor("x".to_owned(), vec![Size::Many]);
    let mu = new_symbol_tensor(r"\mu".to_owned(), vec![Size::Many]);
    let _lsigma = new_symbol_tensor("lsigma".to_owned(), vec![Size::Many; 2]);
    let precision = new_symbol_tensor(r"\Lambda".to_owned(), vec![Size::Many; 2]);

    let pdf_expression = (-0.5
        * ((x.clone() - mu.clone())
            .inner_prod(precision, &[[0, 0]])
            .inner_prod(x.clone() - mu.clone(), &[[1, 0]]))
        .as_scalar())
    .exp();

    let diff = pdf_expression.differential(&["x", r"\mu", r"\Lambda"]);

    if tex {
        println!("x diff");
        println!("{:#?}", diff[0].tex_code());
        println!("mu diff");
        println!("{:#?}", diff[1].tex_code());
        println!("sigma diff");
        println!("{:#?}", diff[2].tex_code());
    } else {
        println!("x diff");
        println!("{:#?}", diff[0].rust_code());
        println!("mu diff");
        println!("{:#?}", diff[1].rust_code());
        println!("sigma diff");
        println!("{:#?}", diff[2].rust_code());
    }
}
