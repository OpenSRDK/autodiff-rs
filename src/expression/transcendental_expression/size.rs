use crate::{Size, TranscendentalExpression};

impl TranscendentalExpression {
    pub fn sizes(&self) -> Vec<Size> {
        match self {
            TranscendentalExpression::Abs(arg) => arg.sizes(),
            TranscendentalExpression::Pow(base, exponent) => {
                [base.sizes(), exponent.sizes()].concat()
            }
            TranscendentalExpression::Exp(arg) => arg.sizes(),
            TranscendentalExpression::Log(base, antilogarithm) => {
                [base.sizes(), antilogarithm.sizes()].concat()
            }
            TranscendentalExpression::Ln(arg) => arg.sizes(),
            TranscendentalExpression::Sin(arg) => arg.sizes(),
            TranscendentalExpression::Cos(arg) => arg.sizes(),
            TranscendentalExpression::Tan(arg) => arg.sizes(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::{new_variable_tensor, Size, TranscendentalExpression};

    #[test]
    fn it_works() {
        let id = "mu";
        let va = new_variable_tensor(id.to_string(), vec![Size::Many, Size::Many]);
        let abs = TranscendentalExpression::Abs(Box::new(va.clone()));
        assert_eq!(abs.sizes(), vec![Size::Many, Size::Many]);

        let pow = TranscendentalExpression::Pow(Box::new(va.clone()), Box::new(2.0.into()));
        assert_eq!(pow.sizes(), vec![Size::Many, Size::Many]);

        let exp = TranscendentalExpression::Exp(Box::new(va.clone()));
        assert_eq!(exp.sizes(), vec![Size::Many, Size::Many]);

        let log = TranscendentalExpression::Log(Box::new(va.clone()), Box::new(2.0.into()));
        assert_eq!(log.sizes(), vec![Size::Many, Size::Many]);

        let ln = TranscendentalExpression::Ln(Box::new(va.clone()));
        assert_eq!(ln.sizes(), vec![Size::Many, Size::Many]);

        let sin = TranscendentalExpression::Sin(Box::new(va.clone()));
        assert_eq!(sin.sizes(), vec![Size::Many, Size::Many]);

        let cos = TranscendentalExpression::Cos(Box::new(va.clone()));
        assert_eq!(cos.sizes(), vec![Size::Many, Size::Many]);

        let tan = TranscendentalExpression::Tan(Box::new(va.clone()));
        assert_eq!(tan.sizes(), vec![Size::Many, Size::Many]);
    }
}
