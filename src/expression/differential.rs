use crate::Expression;

impl Expression {
    pub fn differential(&self, variable_ids: &[&str]) -> Vec<Expression> {
        match self {
            Expression::Variable(id, sizes) => Expression::diff_variable(id, sizes, variable_ids),
            Expression::Constant(_) => vec![0.0.into(); variable_ids.len()],
            Expression::PartialVariable(v) => Expression::diff_partial_variable(v, variable_ids),
            Expression::Add(l, r) => Expression::diff_add(l, r, variable_ids),
            Expression::Sub(l, r) => Expression::diff_sub(l, r, variable_ids),
            Expression::Mul(l, r) => Expression::diff_mul(l, r, variable_ids),
            Expression::Div(l, r) => Expression::diff_div(l, r, variable_ids),
            Expression::Neg(v) => Expression::diff_neg(v, variable_ids),
            Expression::Transcendental(v) => v.differential(variable_ids),
            Expression::Tensor(v) => v.differential(variable_ids),
            Expression::Matrix(v) => v.differential(variable_ids),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::iter::once;

    #[test]
    fn it_works() {
        let x = new_variable("x".to_string());

        let expression = x.clone().pow(2.0.into());
        let diff = expression.differential(&["x"])[0].clone();

        println!("{:#?}", diff);
    }

    #[test]
    fn it_works2() {
        let x = new_variable("x".to_string());

        let expression = (2.0 * x.clone()).exp();
        let diff = expression.differential(&["x"])[0].clone();

        println!("{:#?}", diff);
    }

    #[test]
    fn it_works3() {
        let x = new_variable("x".to_string());

        let expression = x.clone().sin() + x.clone().cos().exp();
        let diff = expression.differential(&["x"])[0].clone();

        println!("{:#?}", diff);
    }

    #[test]
    fn it_works4() {
        let x = new_variable("x".to_string());

        let expression = x.clone().pow(2.0.into());
        let diff = expression.differential(&["x"])[0].clone();

        println!(
            "{:#?}",
            expression.assign(&once(("x", ConstantValue::Scalar(3.0))).collect())
        );

        println!(
            "{:#?}",
            diff.assign(&once(("x", ConstantValue::Scalar(3.0))).collect())
        );
    }

    #[test]
    fn it_works5() {
        let x = new_variable("x".to_string());
        let mu = new_variable("mu".to_string());
        let sigma = new_variable("sigma".to_string());
        let expression = -(x - mu).pow(2.0.into()) / (2.0 * sigma.pow(2.0.into()));
        let diff_x = expression.differential(&["x"])[0].clone();
        let diff_mu = expression.differential(&["mu"])[0].clone();
        let diff_sigma = expression.differential(&["sigma"])[0].clone();
        let diff_anpan = expression.differential(&["anpan"])[0].clone();

        let tex_symbols = vec![("x", "x"), ("mu", r"\mu"), ("sigma", r"\Sigma")]
            .into_iter()
            .collect();

        println!("{:#?}", diff_x.tex_code(&tex_symbols));
        println!("{:#?}", diff_mu.tex_code(&tex_symbols));
        println!("{:#?}", diff_sigma.tex_code(&tex_symbols));
        println!("{:#?}", diff_anpan.tex_code(&tex_symbols));
    }
}
