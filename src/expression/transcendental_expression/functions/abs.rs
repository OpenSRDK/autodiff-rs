use std::collections::HashMap;

use crate::{BracketsLevel, Expression, TranscendentalExpression};

impl Expression {
    pub fn abs(self) -> Self {
        if let Expression::Constant(mut v) = self {
            v.elems_mut().into_iter().for_each(|v| *v = v.abs());
            return v.into();
        }

        TranscendentalExpression::Abs(self.into()).into()
    }
}

impl TranscendentalExpression {
    pub(crate) fn tex_code_abs(arg: &Box<Expression>, symbols: &HashMap<&str, &str>) -> String {
        format!(
            r"\left|{}\right|",
            arg._tex_code(symbols, BracketsLevel::None)
        )
    }
}

#[cfg(test)]
mod tests {

    use crate::new_variable;

    #[test]
    fn it_works() {
        let id = "mu";
        let ea = new_variable(id.to_string());
        let tex_symbols = vec![("mu", r"\mu")].into_iter().collect();

        let ea_abs = ea.clone().abs();
        let tex_a_abs = ea_abs.tex_code(&tex_symbols);
        assert_eq!(r"\left|{\mu}\right|", tex_a_abs);
    }
}
