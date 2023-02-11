use crate::TensorExpression;

impl TensorExpression {
    pub fn tex_code(&self) -> String {
        match self {
            TensorExpression::Symbol(symbol, _) => format!("{{{}}}", symbol),
            TensorExpression::Constant(_) => r"\text{const.}".to_owned(),
            TensorExpression::Zero => r"\mathbf{0}".to_owned(),
            TensorExpression::Add(l, r) => format!("({} + {})", l.tex_code(), r.tex_code()),
            TensorExpression::Sub(l, r) => format!("({} - {})", l.tex_code(), r.tex_code()),
            TensorExpression::MulScalarLhs(l, r) => format!("({} {})", l.tex_code(), r.tex_code()),
            TensorExpression::MulScalarRhs(l, r) => format!("({} {})", l.tex_code(), r.tex_code()),
            TensorExpression::Neg(v) => format!("-{}", v.tex_code()),
            TensorExpression::KroneckerDeltas(rank_pairs) => {
                TensorExpression::tex_code_kronecker_deltas(rank_pairs)
            }
            TensorExpression::InnerProd {
                terms,
                rank_combinations,
            } => TensorExpression::tex_code_inner_prod(terms, rank_combinations),
            TensorExpression::Matrix(m) => m.tex_code(),
        }
    }
}
