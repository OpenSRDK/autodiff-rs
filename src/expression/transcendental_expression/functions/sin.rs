use std::collections::HashMap;

use crate::{BracketsLevel, Expression, TranscendentalExpression};

impl Expression {
    pub fn sin(self) -> Self {
        if let Expression::Constant(mut v) = self {
            v.elems_mut().into_iter().for_each(|v| *v = v.sin());
            return v.into();
        }

        TranscendentalExpression::Sin(self.into()).into()
    }
}

impl TranscendentalExpression {
    pub(crate) fn tex_code_sin(arg: &Box<Expression>, symbols: &HashMap<&str, &str>) -> String {
        format!(
            r"\sin\left({}\right)",
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

        let ea_sin = va.clone().sin();
        let tex_a_sin = ea_sin.tex_code(&tex_symbols);
        assert_eq!(r"\sin\left({\theta}\right)", tex_a_sin);
    }
}
