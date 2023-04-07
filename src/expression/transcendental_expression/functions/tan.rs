use std::collections::HashMap;

use crate::{BracketsLevel, Expression, TranscendentalExpression};

impl Expression {
    pub fn tan(self) -> Self {
        if let Expression::Constant(mut v) = self {
            v.elems_mut().into_iter().for_each(|v| *v = v.tan());
            return v.into();
        }

        TranscendentalExpression::Tan(self.into()).into()
    }
}

impl TranscendentalExpression {
    pub(crate) fn tex_code_tan(arg: &Box<Expression>, symbols: &HashMap<&str, &str>) -> String {
        format!(
            r"\tan\right({}\left)",
            arg._tex_code(symbols, BracketsLevel::None)
        )
    }
}

#[cfg(test)]
mod tests {

    use crate::new_variable;

    #[test]
    fn it_works() {
        let id = "theta";
        let va = new_variable(id.to_string());
        let tex_symbols = vec![("theta", r"\theta")].into_iter().collect();

        let ea_tan = va.clone().tan();
        let tex_a_tan = ea_tan.tex_code(&tex_symbols);
        assert_eq!(r"\tan\right({\theta}\left)", tex_a_tan);
    }
}
