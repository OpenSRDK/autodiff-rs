use crate::TensorExpression;
use opensrdk_linear_algebra::RankIndex;

impl TensorExpression {
    pub(crate) fn rust_code_kronecker_deltas(
        rank_pairs: &[[RankIndex; 2]],
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

    pub(crate) fn tex_code_kronecker_deltas(rank_pairs: &[[RankIndex; 2]]) -> String {
        let inner = rank_pairs
            .iter()
            .map(|rank_pair| format!(r"{{\delta_{{[{}], [{}]}}}}", rank_pair[0], rank_pair[1]))
            .collect::<Vec<_>>()
            .join(" ");
        inner
    }
}
