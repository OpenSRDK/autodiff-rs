use crate::Expression;
use num_traits::cast::ToPrimitive;

impl Expression {
    pub fn differential(&self, symbols: &[&str]) -> Vec<Expression> {
        match self {
            Expression::Symbol(symbol) => Expression::diff_symbol(symbols, symbol),
            Expression::Constant(_) => vec![Expression::Constant(0.0); symbols.len()],
            Expression::Add(l, r) => Expression::diff_add(symbols, l, r),
            Expression::Sub(l, r) => l
                .differential(symbols)
                .into_iter()
                .zip(r.differential(symbols).into_iter())
                .map(|(li, ri)| li - ri)
                .collect(),
            Expression::Mul(l, r) => l
                .differential(symbols)
                .into_iter()
                .zip(r.differential(symbols).into_iter())
                .map(|(li, ri)| li * r.as_ref().clone() + l.as_ref().clone() * ri)
                .collect(),
            Expression::Div(l, r) => l
                .differential(symbols)
                .into_iter()
                .zip(r.differential(symbols).into_iter())
                .map(|(li, ri)| {
                    (li * r.as_ref().clone() - l.as_ref().clone() * ri)
                        / r.as_ref().clone().pow(2.into())
                })
                .collect(),
            Expression::Neg(v) => v.differential(symbols).into_iter().map(|e| -e).collect(),
            Expression::Pow(base, exponent) => base
                .differential(symbols)
                .into_iter()
                .map(|b| {
                    Expression::Constant(exponent.to_f64().unwrap_or_default())
                        * base.as_ref().clone().pow(exponent - 1)
                        * b
                })
                .collect(),
            Expression::Transcendental(v) => v.differential(symbols),
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

        let expression = x.clone().pow(2.into());
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

        let expression = x.clone().pow(2.into());
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
