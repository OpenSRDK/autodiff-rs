use opensrdk_linear_algebra::Tensor;

use crate::{Expression, TensorExpression};

impl TensorExpression {
    pub fn inner_prod(self, rhs: TensorExpression, level_pairs: &[(usize, usize)]) -> Self {
        // Merge constant
        if let TensorExpression::Constant(vl) = &self {
            if let TensorExpression::Constant(vr) = &rhs {
                return TensorExpression::Constant(vl.inner_prod(vr, level_pairs));
            }
            if let TensorExpression::KroneckerDeltas(level_pairs_r) = &rhs {
                return TensorExpression::Constant(vl.mul_kronecker_deltas(level_pairs_r));
            }
            if vl.total_size() == 1 {
                return TensorExpression::MulScalarLhs(
                    Expression::Constant(vl[&vec![0; vl.levels()]]).into(),
                    rhs.into(),
                );
            }
        }
        if let TensorExpression::Constant(vr) = &rhs {
            if let TensorExpression::KroneckerDeltas(level_pairs_l) = &self {
                return TensorExpression::Constant(vr.mul_kronecker_deltas(level_pairs_l));
            }
            if vr.total_size() == 1 {
                return TensorExpression::MulScalarRhs(
                    self.into(),
                    Expression::Constant(vr[&vec![0; vr.levels()]]).into(),
                );
            }
        }
        // Merge Zero
        if let TensorExpression::Zero = self {
            return TensorExpression::Zero;
        }
        if let TensorExpression::Zero = rhs {
            return TensorExpression::Zero;
        }
        // Merge KroneckerDeltas
        if let TensorExpression::KroneckerDeltas(level_pairs_l) = &self {
            if let TensorExpression::KroneckerDeltas(level_pairs_r) = &rhs {
                return TensorExpression::KroneckerDeltas(
                    level_pairs_l
                        .iter()
                        .chain(level_pairs_r.iter())
                        .cloned()
                        .collect(),
                );
            }
        }

        TensorExpression::InnerProd {
            lhs: self.into(),
            rhs: rhs.into(),
            level_pairs: level_pairs.to_vec(),
        }
    }
}

impl TensorExpression {
    pub(crate) fn diff_inner_prod(
        symbols: &[&str],
        l: &Box<TensorExpression>,
        r: &Box<TensorExpression>,
        level_pairs: &[(usize, usize)],
    ) -> Vec<TensorExpression> {
        l.differential(symbols)
            .iter()
            .zip(r.differential(symbols).iter())
            .map(|(dl, dr)| {
                dl.clone().inner_prod(*r.clone(), level_pairs)
                    + l.clone().inner_prod(dr.clone(), level_pairs)
            })
            .collect()
    }

    pub(crate) fn rust_code_inner_prod(
        lhs: &Box<TensorExpression>,
        rhs: &Box<TensorExpression>,
        level_pairs: &[(usize, usize)],
        parentheses: bool,
    ) -> String {
        let inner = format!(
            "{}.inner_prod({}, &[{}])",
            lhs._rust_code(true),
            rhs._rust_code(true),
            level_pairs
                .iter()
                .map(|level_pair| format!("({}, {})", level_pair.0, level_pair.1))
                .collect::<Vec<_>>()
                .join(", ")
        );
        if parentheses {
            format!("({} )", inner)
        } else {
            inner
        }
    }
}
