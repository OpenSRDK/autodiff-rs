use crate::TensorExpression;

impl TensorExpression {
    pub fn tex_code(&self) -> String {
        match self {
            TensorExpression::Symbol(symbol, _) => format!("{{{}}}", symbol),
            TensorExpression::Constant(value) => r"\text{const.}".to_owned(),
            TensorExpression::Zero => r"\mathbf{0}".to_owned(),
            TensorExpression::Add(l, r) => format!("({} + {})", l.tex_code(), r.tex_code()),
            TensorExpression::Sub(l, r) => format!("({} - {})", l.tex_code(), r.tex_code()),
            TensorExpression::MulScalarLhs(l, r) => format!("({} {})", l.tex_code(), r.tex_code()),
            TensorExpression::MulScalarRhs(l, r) => format!("({} {})", l.tex_code(), r.tex_code()),
            TensorExpression::Neg(v) => format!("-{}", v.tex_code()),
            TensorExpression::KroneckerDeltas(rank_pairs) => {
                let mut result = String::new();
                for [l, r] in rank_pairs {
                    result.push_str(&format!(r"\delta_{{[{}], [{}]}}", l, r));
                }
                format!("{{{}}}", result)
            }
            TensorExpression::InnerProd {
                v,
                rank_combinations,
            } => TensorExpression::tex_code_inner_prod(v, rank_combinations),
        }
    }
}
