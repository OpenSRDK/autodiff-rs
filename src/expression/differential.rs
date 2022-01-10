use crate::{Expression, Symbol};

impl Expression {
    pub fn differential(&self, symbols: &[Symbol]) -> Vec<Expression> {
        match self {
            Expression::Symbol(symbol) => symbols
                .iter()
                .map(|s| {
                    if s.eq(symbol) {
                        Expression::Constant(1.0)
                    } else {
                        Expression::Constant(0.0)
                    }
                })
                .collect(),
            Expression::Constant(_) => vec![Expression::Constant(0.0); symbols.len()],
            Expression::Add(l, r) => l
                .differential(symbols)
                .into_iter()
                .zip(r.differential(symbols).into_iter())
                .map(|(li, ri)| li + ri)
                .collect(),
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
                .map(|(li, ri)| li * ri)
                .collect(),
            Expression::Div(l, r) => l
                .differential(symbols)
                .into_iter()
                .zip(r.differential(symbols).into_iter())
                .map(|(li, ri)| li / ri)
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
            Expression::Pow(base, exponent) => {
                if base.symbols().intersection(&exponent.symbols()).count() > 0 {
                    panic!("Intersection of symbols of base and exponent must be empty set.");
                }

                todo!()
            }
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
                .map(|a| a / (arg.clone().cos() * arg.clone().cos()))
                .collect(),
            Expression::MatrixScalar(expression) => todo!(),
        }
    }
}
