use crate::{TranscendentalExpression, Expression};

impl TranscendentalExpression {
    pub fn differential(&self, symbols: &[&str]) -> Vec<Expression> {
        match self { 
          TranscendentalExpression::Abs(arg) => arg
                .differential(symbols)
                .into_iter()
                .map(|a| a.abs())
                .collect(),
                TranscendentalExpression::Pow(base, exponent) => symbols
                .iter()
                .map(|&s| {
                    if base.symbols().contains(s) {
                        if exponent.symbols().contains(s) {
                            panic!(
                                "Symbols which are contained by both of base and exponent cannot be differentiated."
                            );
                        }

                        exponent.as_ref().clone()
                            * base.as_ref().clone().pow_transcendental(exponent.as_ref().clone() - 1.0)
                            * base.differential(&[s])[0].clone()
                    } else if exponent.symbols().contains(s) {
                        base.as_ref().clone().pow_transcendental(exponent.as_ref().clone())
                            * exponent.as_ref().clone().ln()
                    } else {
                        0.0.into()
                    }
                })
                .collect(),
                TranscendentalExpression::Exp(arg) => arg
                .differential(symbols)
                .into_iter()
                .map(|a| arg.clone().exp() * a)
                .collect(),
                TranscendentalExpression::Log(_, _) => todo!(),
                TranscendentalExpression::Ln(arg) => arg
                .differential(symbols)
                .into_iter()
                .map(|a| a / arg.as_ref().clone())
                .collect(),
                TranscendentalExpression::Sin(arg) => arg
                .differential(symbols)
                .into_iter()
                .map(|a| arg.clone().cos() * a)
                .collect(),
                TranscendentalExpression::Cos(arg) => arg
                .differential(symbols)
                .into_iter()
                .map(|a| -arg.clone().sin() * a)
                .collect(),
                TranscendentalExpression::Tan(arg) => arg
                .differential(symbols)
                .into_iter()
                .map(|a| a / (arg.clone().cos().pow(2.into())))
                .collect(),
        }
    }
}
