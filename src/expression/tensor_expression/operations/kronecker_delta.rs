use crate::TensorExpression;

impl TensorExpression {
    pub(crate) fn rust_code_kronecker_deltas(
        level_pairs: &[(usize, usize)],
        parentheses: bool,
    ) -> String {
        let inner = level_pairs
            .iter()
            .map(|level_pair| format!("KroneckerDelta({}, {})", level_pair.0, level_pair.1))
            .collect::<Vec<_>>()
            .join(" * ");
        if parentheses {
            format!("({})", inner)
        } else {
            inner
        }
    }
}
