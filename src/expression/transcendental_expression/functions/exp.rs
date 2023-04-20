use std::collections::HashMap;

use crate::{Expression, TranscendentalExpression};

impl Expression {
    pub fn exp(self) -> Self {
        if let Expression::Constant(mut v) = self {
            v.elems_mut().into_iter().for_each(|v| *v = v.exp());
            return v.into();
        }

        TranscendentalExpression::Exp(self.into()).into()
    }
}

impl TranscendentalExpression {
    pub(crate) fn tex_code_exp(arg: &Box<Expression>, symbols: &HashMap<&str, &str>) -> String {
        format!(
            r"\exp{{{}}}",
            arg._tex_code(symbols, crate::BracketsLevel::ForOperation)
        )
    }
}

#[cfg(test)]
mod tests {

    use crate::new_variable;

    #[test]
    fn it_works() {
        let id = "mu";
        let va = new_variable(id.to_string());
        let tex_symbols = vec![("mu", r"\mu")].into_iter().collect();

        let ea_exp = va.clone().exp();
        let tex_a_exp = ea_exp.tex_code(&tex_symbols);
        assert_eq!(r"\exp{{\mu}}", tex_a_exp);
    }
}
