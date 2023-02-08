use opensrdk_linear_algebra::Tensor;

use crate::{Expression, TensorExpression};

impl TensorExpression {
    pub fn inner_prod(self, rhs: TensorExpression, level_pairs: &[(usize, usize)]) -> Self {
        // Merge constant
        if let TensorExpression::Constant(vl) = &self {
            if let TensorExpression::Constant(vr) = &rhs {
                return TensorExpression::Constant(vl.inner_prod(vr, level_pairs));
            }
            if vl.total_size() == 1 {
                return TensorExpression::MulScalarLhs(
                    Expression::Constant(vl[&vec![0; vl.levels()]]).into(),
                    rhs.into(),
                );
            }
            if let TensorExpression::KroneckerDeltas {
                levels: r_levels,
                level_pairs: r_level_pairs,
            } = &rhs
            {
                todo!()
            }
        }
        if let TensorExpression::Constant(vr) = &rhs {
            if vr.total_size() == 0 {
                return TensorExpression::MulScalarRhs(
                    self.into(),
                    Expression::Constant(vr[&vec![0; vr.levels()]]).into(),
                );
            }
            if let TensorExpression::KroneckerDeltas {
                levels: l_levels,
                level_pairs: l_level_pairs,
            } = &self
            {
                todo!()
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
        if let TensorExpression::KroneckerDeltas {
            levels: l_levels,
            level_pairs: l_level_pairs,
        } = &self
        {
            if let TensorExpression::KroneckerDeltas {
                levels: r_levels,
                level_pairs: r_level_pairs,
            } = &rhs
            {
                return TensorExpression::KroneckerDeltas {
                    levels: l_levels + r_levels,
                    level_pairs: [l_level_pairs.to_owned(), r_level_pairs.to_owned()].concat(),
                };
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
                dl.inner_prod(r.clone(), level_pairs) + l.clone().inner_prod(dr, level_pairs)
            })
            .collect()
    }

    pub(crate) fn rust_code_inner_prod(
        lhs: &Box<TensorExpression>,
        rhs: &Box<TensorExpression>,
        level_pairs: &[(usize, usize)],
        parentheses: bool,
    ) -> String {
        if parentheses {
            format!(
                "({}.inner_prod({}, &{:#?}))",
                lhs._rust_code(true),
                rhs._rust_code(true),
                level_pairs
            )
        } else {
            format!(
                "{}.inner_prod({}, &{:#?})",
                lhs._rust_code(true),
                rhs._rust_code(true),
                level_pairs
            )
        }
    }
}
