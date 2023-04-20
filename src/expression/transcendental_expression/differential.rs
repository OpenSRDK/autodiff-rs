use crate::{Expression, TranscendentalExpression};

impl TranscendentalExpression {
    pub fn differential(&self, variable_ids: &[&str]) -> Vec<Expression> {
        match self {
            TranscendentalExpression::Abs(arg) => arg
                .differential(variable_ids)
                .into_iter()
                .map(|a| a.abs())
                .collect(),
            TranscendentalExpression::Pow(base, exponent) => base
                .differential(variable_ids)
                .into_iter()
                .zip(exponent.differential(variable_ids).into_iter())
                .map(|(b, e)| {
                    println!("b: {:?}, e: {:?}", b, e);
                    base.as_ref().clone().pow(exponent.as_ref().clone())
                        * (e * base.as_ref().clone().ln()
                            + exponent.as_ref().clone() * (b / base.as_ref().clone()))
                })
                .collect(),
            TranscendentalExpression::Exp(arg) => arg
                .differential(variable_ids)
                .into_iter()
                .map(|a| arg.clone().exp() * a)
                .collect(),
            TranscendentalExpression::Log(_, _) => todo!(),
            TranscendentalExpression::Ln(arg) => arg
                .differential(variable_ids)
                .into_iter()
                .map(|a| a / arg.as_ref().clone())
                .collect(),
            TranscendentalExpression::Sin(arg) => arg
                .differential(variable_ids)
                .into_iter()
                .map(|a| arg.clone().cos() * a)
                .collect(),
            TranscendentalExpression::Cos(arg) => arg
                .differential(variable_ids)
                .into_iter()
                .map(|a| -arg.clone().sin() * a)
                .collect(),
            TranscendentalExpression::Tan(arg) => arg
                .differential(variable_ids)
                .into_iter()
                .map(|a| a / (arg.clone().cos().pow(2.0.into())))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{new_variable, ConstantValue, Expression, TranscendentalExpression};

    #[test]
    fn it_works() {
        let id = "mu";
        let va = new_variable(id.to_string());
        let abs = TranscendentalExpression::Abs(Box::new(va.clone()));
        let pow = TranscendentalExpression::Pow(Box::new(va.clone()), Box::new(2.0.into()));
        let exp = TranscendentalExpression::Exp(Box::new(va.clone()));
        let ln = TranscendentalExpression::Ln(Box::new(va.clone()));
        let sin = TranscendentalExpression::Sin(Box::new(va.clone()));
        let cos = TranscendentalExpression::Cos(Box::new(va.clone()));
        let tan = TranscendentalExpression::Tan(Box::new(va.clone()));

        let variables = vec![id];

        assert_eq!(
            vec![Expression::Constant(ConstantValue::Scalar(1.0))],
            abs.differential(&variables)
        );
        println!("pow_diff: {:?}", pow.differential(&variables));
        assert_eq!(
            vec![Expression::from(exp.clone())],
            exp.differential(&variables)
        );
        println!("ln_diff: {:?}", ln.differential(&variables));
        println!("sin_diff: {:?}", sin.differential(&variables));
        println!("cos_diff: {:?}", cos.differential(&variables));
        println!("tan_diff: {:?}", tan.differential(&variables));
    }
}
