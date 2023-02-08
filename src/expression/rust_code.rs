use crate::Expression;

impl Expression {
    pub(crate) fn _rust_code(&self, parentheses: bool) -> String {
        match self {
            Expression::Symbol(symbol) => Expression::rust_code_symbol(symbol),
            Expression::Constant(v) => Self::f64_to_string_with_precision(*v),
            Expression::Add(l, r) => Expression::rust_code_add(l, r, parentheses),
            Expression::Sub(l, r) => Expression::rust_code_sub(l, r, parentheses),
            Expression::Mul(l, r) => Expression::rust_code_mul(l, r, parentheses),
            Expression::Div(l, r) => Expression::rust_code_div(l, r, parentheses),
            Expression::Neg(v) => Expression::rust_code_neg(v),
            Expression::Pow(base, exponent) => Expression::rust_code_powr(base, exponent),
            Expression::Transcendental(v) => v.rust_code(),
            Expression::Tensor(v, _) => v._rust_code(parentheses),
        }
    }

    pub fn rust_code(&self) -> String {
        Self::_rust_code(&self, false)
    }

    pub fn f64_to_string_with_precision(v: f64) -> String {
        let v_str = v.to_string();
        if v_str.contains('.') {
            v_str
        } else {
            v_str + ".0"
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
