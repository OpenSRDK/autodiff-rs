use crate::TensorExpression;

impl TensorExpression {
    pub(crate) fn rust_code_kronecker_deltas(
        rank_pairs: &[[usize; 2]],
        parentheses: bool,
    ) -> String {
        let inner = rank_pairs
            .iter()
            .map(|rank_pair| format!("KroneckerDelta({}, {})", rank_pair[0], rank_pair[1]))
            .collect::<Vec<_>>()
            .join(" * ");
        if parentheses {
            format!("({})", inner)
        } else {
            inner
        }
    }
}
