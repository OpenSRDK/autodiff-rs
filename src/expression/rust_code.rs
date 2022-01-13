use crate::Expression;

impl Expression {
    pub fn rust_code(&self) -> String {
        match self {
            Expression::Symbol(symbol) => Expression::rust_code_symbol(symbol),
            Expression::Constant(v) => v.to_string(),
            Expression::Add(l, r) => Expression::rust_code_add(l, r),
            Expression::Sub(l, r) => Expression::rust_code_sub(l, r),
            Expression::Mul(l, r) => Expression::rust_code_mul(l, r),
            Expression::Div(l, r) => Expression::rust_code_div(l, r),
            Expression::Neg(v) => Expression::rust_code_neg(v),
            Expression::Pow(base, exponent) => Expression::rust_code_powr(base, exponent),
            Expression::Transcendental(v) => todo!(),
            Expression::MatrixScalar(v) => todo!(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::iter::once;

    #[test]
    fn it_works() {
        let x = Expression::new_symbol("x".to_string());

        let expression = x.clone().powr(2.into());
        let diff = expression.differential(&["x"])[0].clone();

        println!("{:#?}", diff);
    }

    #[test]
    fn it_works2() {
        let x = Expression::new_symbol("x".to_string());

        let expression = (2.0 * x.clone()).exp();
        let diff = expression.differential(&["x"])[0].clone();

        println!("{:#?}", diff);
    }

    #[test]
    fn it_works3() {
        let x = Expression::new_symbol("x".to_string());

        let expression = x.clone().sin() + x.clone().cos().exp();
        let diff = expression.differential(&["x"])[0].clone();

        println!("{:#?}", diff);
    }

    #[test]
    fn it_works4() {
        let x = Expression::new_symbol("x".to_string());

        let expression = x.clone().powr(2.into());
        let diff = expression.differential(&["x"])[0].clone();

        println!(
            "{:#?}",
            expression.evaluate(&once(("x", Value::Scalar(3.0))).collect())
        );

        println!(
            "{:#?}",
            diff.evaluate(&once(("x", Value::Scalar(3.0))).collect())
        );
    }
}
