use crate::Expression;

impl Expression {
    pub fn differential(&self, symbols: &[&str]) -> Vec<Expression> {
        match self {
            Expression::Symbol(symbol) => Expression::diff_symbol(symbols, symbol),
            Expression::Constant(_) => vec![Expression::Constant(0.0); symbols.len()],
            Expression::Add(l, r) => Expression::diff_add(symbols, l, r),
            Expression::Sub(l, r) => Expression::diff_sub(symbols, l, r),
            Expression::Mul(l, r) => Expression::diff_mul(symbols, l, r),
            Expression::Div(l, r) => Expression::diff_div(symbols, l, r),
            Expression::Neg(v) => Expression::diff_neg(symbols, v),
            Expression::Pow(base, exponent) => Expression::diff_powr(symbols, base, exponent),
            Expression::Transcendental(v) => v.differential(symbols),
            Expression::DegeneratedTensor(v) => v
                .differential(symbols)
                .into_iter()
                .map(|expression| Expression::DiffResultTensor(expression.into()))
                .collect(),
            Expression::DiffResultTensor(v) => v
                .differential(symbols)
                .into_iter()
                .map(|expression| Expression::DiffResultTensor(expression.into()))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::iter::once;

    #[test]
    fn it_works() {
        let x = new_symbol("x".to_string());

        let expression = x.clone().powr(2.into());
        let diff = expression.differential(&["x"])[0].clone();

        println!("{:#?}", diff);
    }

    #[test]
    fn it_works2() {
        let x = new_symbol("x".to_string());

        let expression = (2.0 * x.clone()).exp();
        let diff = expression.differential(&["x"])[0].clone();

        println!("{:#?}", diff);
    }

    #[test]
    fn it_works3() {
        let x = new_symbol("x".to_string());

        let expression = x.clone().sin() + x.clone().cos().exp();
        let diff = expression.differential(&["x"])[0].clone();

        println!("{:#?}", diff);
    }

    #[test]
    fn it_works4() {
        let x = new_symbol("x".to_string());

        let expression = x.clone().powr(2.into());
        let diff = expression.differential(&["x"])[0].clone();

        println!(
            "{:#?}",
            expression.assign(&once(("x", Value::Scalar(3.0))).collect())
        );

        println!(
            "{:#?}",
            diff.assign(&once(("x", Value::Scalar(3.0))).collect())
        );
    }

    #[test]
    fn it_works5() {
        let x = new_symbol("x".to_string());
        let mu = new_symbol("mu".to_string());
        let sigma = new_symbol("sigma".to_string());
        let expression = -(x - mu).powr(2.into()) / (2.0 * sigma.powr(2.into()));
        let diff_x = expression.differential(&["x"])[0].clone();
        let diff_mu = expression.differential(&["mu"])[0].clone();
        let diff_sigma = expression.differential(&["sigma"])[0].clone();
        println!("{:#?}", diff_x.rust_code());
        println!("{:#?}", diff_mu.rust_code());
        println!("{:#?}", diff_sigma.rust_code());
    }
}
