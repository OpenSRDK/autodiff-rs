use std::collections::HashMap;

use crate::{BracketsLevel, Expression, TranscendentalExpression};

impl Expression {
    pub fn cos(self) -> Self {
        if let Expression::Constant(mut v) = self {
            v.elems_mut().into_iter().for_each(|v| *v = v.cos());
            return v.into();
        }

        TranscendentalExpression::Cos(self.into()).into()
    }
}

impl TranscendentalExpression {
    pub(crate) fn tex_code_cos(arg: &Box<Expression>, symbols: &HashMap<&str, &str>) -> String {
        format!(
            r"\cos\left({}\right)",
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

        let ea_cos = va.clone().cos();
        let tex_a_cos = ea_cos.tex_code(&tex_symbols);
        assert_eq!(r"\cos\left({\theta}\right)", tex_a_cos);
    }
}
