use crate::{BracketsLevel, TensorExpression};
use opensrdk_linear_algebra::RankIndex;

impl TensorExpression {
    pub(crate) fn tex_code_kronecker_deltas(
        rank_pairs: &[[RankIndex; 2]],
        brackets_level: BracketsLevel,
    ) -> String {
        let inner = rank_pairs
            .iter()
            .map(|rank_pair| format!(r"{{\delta_{{[{}], [{}]}}}}", rank_pair[0], rank_pair[1]))
            .collect::<Vec<_>>()
            .join(" ");

        match brackets_level {
            BracketsLevel::None | BracketsLevel::ForMul => inner,
            BracketsLevel::ForDiv | BracketsLevel::ForOperation => {
                format!(r"\left({}\right)", inner)
            }
        }
    }
}
