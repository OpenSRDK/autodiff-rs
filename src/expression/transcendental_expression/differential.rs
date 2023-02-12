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
