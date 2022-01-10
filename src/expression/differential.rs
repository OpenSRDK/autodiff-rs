use crate::Expression;

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
                .map(|(li, ri)| (li * r.as_ref().clone() - l.as_ref().clone() * ri) / r.as_ref().clone().pow(2.0.into()))
                .collect(),
            Expression::Neg(expression) => expression
                .differential(symbols)
                .into_iter()
                .map(|e| -e)
                .collect(),
            Expression::Abs(arg) => arg
                .differential(symbols)
                .into_iter()
                .map(|a| a.abs())
                .collect(),
            Expression::Pow(base, exponent) => symbols
                .iter()
                .map(|&s| {
                    if base.symbols().contains(s) {
                        if exponent.symbols().contains(s) {
                            panic!(
                                "Symbols which are contained by both of base and exponent cannot be differentiated."
                            );
                        }

                        exponent.as_ref().clone()
                            * base.as_ref().clone().pow(exponent.as_ref().clone() - 1.0)
                            * base.differential(&[s])[0].clone()
                    } else if exponent.symbols().contains(s) {
                        base.as_ref().clone().pow(exponent.as_ref().clone())
                            * exponent.as_ref().clone().ln()
                    } else {
                        0.0.into()
                    }
                })
                .collect(),
            Expression::Exp(arg) => arg
                .differential(symbols)
                .into_iter()
                .map(|a| arg.clone().exp() * a)
                .collect(),
            Expression::Log(_, _) => todo!(),
            Expression::Ln(arg) => arg
                .differential(symbols)
                .into_iter()
                .map(|a| a / arg.as_ref().clone())
                .collect(),
            Expression::Sin(arg) => arg
                .differential(symbols)
                .into_iter()
                .map(|a| arg.clone().cos() * a)
                .collect(),
            Expression::Cos(arg) => arg
                .differential(symbols)
                .into_iter()
                .map(|a| -arg.clone().sin() * a)
                .collect(),
            Expression::Tan(arg) => arg
                .differential(symbols)
                .into_iter()
                .map(|a| a / (arg.clone().cos().pow(2.0.into())))
                .collect(),
            Expression::MatrixScalar(expression) => todo!(),
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

        let expression = x.clone().pow(2.0.into());
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

        let expression = x.clone().pow(2.0.into());
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
