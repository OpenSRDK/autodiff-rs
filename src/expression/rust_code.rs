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
            Expression::Transcendental(v) => v.rust_code(),
            Expression::MatrixScalar(v) => v.rust_code(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn it_works() {
        let x = Expression::new_symbol("x".to_string());

        let expression = x.clone().powr(2.into());

        println!("{}", expression.rust_code());
    }

    #[test]
    fn it_works2() {
        let x = Expression::new_symbol("x".to_string());

        let expression = (2.0 * x.clone()).exp();

        println!("{}", expression.rust_code());
    }

    #[test]
    fn it_works3() {
        let x = Expression::new_symbol("x".to_string());

        let expression = x.clone().sin() + x.clone().cos().exp();

        println!("{}", expression.rust_code());
    }

    #[test]
    fn it_works4() {
        let x = Expression::new_symbol("x".to_string());

        let expression = x.clone().powr(2.into());

        println!("{}", expression.rust_code());
    }
}
