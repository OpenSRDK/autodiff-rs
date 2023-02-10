use crate::{Size, TensorExpression};
use opensrdk_linear_algebra::Tensor;

impl TensorExpression {
    pub fn is_same_size(&self, other: &TensorExpression) -> bool {
        let sl = self.sizes();
        let sr = other.sizes();

        if sl.len() == 0 || sr.len() == 0 {
            return true;
        }

        sl == sr
    }

    pub fn sizes(&self) -> Vec<Size> {
        match self {
            TensorExpression::Symbol(_, sizes) => sizes.clone(),
            TensorExpression::Constant(v) => (0..v.rank())
                .into_iter()
                .map(|r| {
                    if v.size(r) == 1 {
                        Size::One
                    } else {
                        Size::Many
                    }
                })
                .collect(),
            TensorExpression::Zero => vec![],
            TensorExpression::Add(l, _) => l.sizes(),
            TensorExpression::Sub(l, _) => l.sizes(),
            TensorExpression::MulScalarLhs(_, r) => r.sizes(),
            TensorExpression::MulScalarRhs(l, _) => l.sizes(),
            TensorExpression::Neg(v) => v.sizes(),
            TensorExpression::KroneckerDeltas(_) => vec![],
            TensorExpression::InnerProd {
                terms,
                rank_combinations,
            } => TensorExpression::size_inner_prod(terms, rank_combinations),
        }
    }
}
