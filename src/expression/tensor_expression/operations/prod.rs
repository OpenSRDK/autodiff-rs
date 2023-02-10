use crate::{Expression, TensorExpression};
use opensrdk_linear_algebra::{sparse::operations::kronecker_delta::KroneckerDelta, Tensor};
use std::{collections::HashMap, iter::once};

fn next_char(c: char, count: u32) -> char {
    std::char::from_u32(c as u32 + count).unwrap_or(c)
}

fn inner_prod_rank_combinations(
    rank_pairs: &[[usize; 2]],
    id_prefix: String,
) -> [HashMap<usize, String>; 2] {
    let mut rank_combinations = [HashMap::new(), HashMap::new()];
    for rank_pair in rank_pairs.iter() {
        rank_combinations[0].insert(
            rank_pair[0],
            format!("{}/[{}]-[{}]", id_prefix, rank_pair[0], rank_pair[1]),
        );
        rank_combinations[1].insert(
            rank_pair[1],
            format!("{}/[{}]-[{}]", id_prefix, rank_pair[0], rank_pair[1]),
        );
    }

    rank_combinations
}

fn merge_inner_prod_rank_combinations(
    mut rank_combinations: Vec<HashMap<usize, String>>,
    not_1dimension_ranks: HashMap<usize, usize>,
    rank_pairs: &[[usize; 2]],
    is_new_lhs: bool,
) -> Vec<HashMap<usize, String>> {
    let rank_pairs = rank_pairs
        .iter()
        .cloned()
        .map(|mut rank_pair| {
            if is_new_lhs {
                rank_pair.reverse()
            }
            rank_pair
        })
        .collect::<Vec<_>>();

    let mut new_combination = HashMap::new();
    let new_index = rank_combinations.len();

    for rank_pair in rank_pairs.iter() {
        let index = not_1dimension_ranks[&rank_pair[0]];
        let [rank_combinations_addition, new_combination_addition] =
            inner_prod_rank_combinations(&[*rank_pair], format!("{}-{}", index, new_index));

        rank_combinations[index].extend(rank_combinations_addition);
        new_combination.extend(new_combination_addition);
    }

    if is_new_lhs {
        [&[new_combination], &rank_combinations[..]].concat()
    } else {
        [&rank_combinations[..], &[new_combination]].concat()
    }
}
impl TensorExpression {
    pub fn inner_prod(
        self,
        rhs: TensorExpression,
        rank_pairs: &[[usize; 2]],
        not_1dimension_ranks: HashMap<usize, usize>,
    ) -> Self {
        // Merge constant
        if let TensorExpression::Constant(vl) = &self {
            if let TensorExpression::Constant(vr) = &rhs {
                return TensorExpression::Constant(vl.clone().inner_prod(vr.clone(), rank_pairs));
            }
            if let TensorExpression::KroneckerDeltas(rank_pairs_r) = &rhs {
                return TensorExpression::Constant(
                    vl.mul_kronecker_deltas(
                        &rank_pairs_r
                            .iter()
                            .map(|rank_pair| KroneckerDelta(rank_pair[0], rank_pair[1]))
                            .collect::<Vec<_>>(),
                    ),
                );
            }
            if vl.total_size() == 1 {
                return TensorExpression::MulScalarLhs(
                    Expression::Constant(vl[&vec![0; vl.rank()]]).into(),
                    rhs.into(),
                );
            }
        }
        if let TensorExpression::Constant(vr) = &rhs {
            if let TensorExpression::KroneckerDeltas(rank_pairs_l) = &self {
                return TensorExpression::Constant(
                    vr.mul_kronecker_deltas(
                        &rank_pairs_l
                            .iter()
                            .map(|rank_pair| KroneckerDelta(rank_pair[0], rank_pair[1]))
                            .collect::<Vec<_>>(),
                    ),
                );
            }
            if vr.total_size() == 1 {
                return TensorExpression::MulScalarRhs(
                    self.into(),
                    Expression::Constant(vr[&vec![0; vr.rank()]]).into(),
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
        if let TensorExpression::KroneckerDeltas(rank_pairs_l) = &self {
            if let TensorExpression::KroneckerDeltas(rank_pairs_r) = &rhs {
                return TensorExpression::KroneckerDeltas(
                    [&rank_pairs_l[..], &rank_pairs_r[..]].concat(),
                );
            }
        }
        // Merge InnerProd
        if let TensorExpression::InnerProd {
            v: v_l,
            rank_combinations: rank_combinations_l,
        } = &self
        {
            if let TensorExpression::InnerProd {
                v: v_r,
                rank_combinations: rank_combinations_r,
            } = &rhs
            {
                return TensorExpression::InnerProd {
                    v: [&v_l[..], &v_r[..]].concat(),
                    rank_combinations: merge_inner_prod_rank_combinations(
                        [&rank_combinations_l[..], &rank_combinations_r[..]].concat(),
                        not_1dimension_ranks,
                        rank_pairs,
                        false,
                    ),
                };
            }

            return TensorExpression::InnerProd {
                v: [&v_l[..], &[rhs]].concat(),
                rank_combinations: merge_inner_prod_rank_combinations(
                    rank_combinations_l.clone(),
                    not_1dimension_ranks,
                    rank_pairs,
                    false,
                ),
            };
        }

        if let TensorExpression::InnerProd {
            v: v_r,
            rank_combinations: rank_combinations_r,
        } = &rhs
        {
            return TensorExpression::InnerProd {
                v: [&[self], &v_r[..]].concat(),
                rank_combinations: merge_inner_prod_rank_combinations(
                    rank_combinations_r.clone(),
                    not_1dimension_ranks,
                    rank_pairs,
                    true,
                ),
            };
        }

        return TensorExpression::InnerProd {
            v: vec![self, rhs],
            rank_combinations: inner_prod_rank_combinations(rank_pairs, "0-1".to_string()).to_vec(),
        };
    }
}

impl TensorExpression {
    // pub(crate) fn diff_inner_prod(
    //     symbols: &[&str],
    //     l: &Box<TensorExpression>,
    //     r: &Box<TensorExpression>,
    //     rank_pairs: &[(usize, usize)],
    // ) -> Vec<TensorExpression> {
    //     l.differential(symbols)
    //         .iter()
    //         .zip(r.differential(symbols).iter())
    //         .map(|(dl, dr)| {
    //             dl.clone().inner_prod(*r.clone(), rank_pairs)
    //                 + l.clone().inner_prod(dr.clone(), rank_pairs)
    //         })
    //         .collect()
    // }

    // pub(crate) fn rust_code_inner_prod(
    //     lhs: &Box<TensorExpression>,
    //     rhs: &Box<TensorExpression>,
    //     rank_pairs: &[(usize, usize)],
    //     parentheses: bool,
    // ) -> String {
    //     let inner = format!(
    //         "{}.inner_prod({}, &[{}])",
    //         lhs._rust_code(true),
    //         rhs._rust_code(true),
    //         rank_pairs
    //             .iter()
    //             .map(|rank_pair| format!("({}, {})", rank_pair.0, rank_pair.1))
    //             .collect::<Vec<_>>()
    //             .join(", ")
    //     );
    //     if parentheses {
    //         format!("({} )", inner)
    //     } else {
    //         inner
    //     }
    // }

    // pub(crate) fn tex_code_inner_prod(
    //     lhs: &Box<TensorExpression>,
    //     rhs: &Box<TensorExpression>,
    //     rank_pairs: &[(usize, usize)],
    // ) -> String {
    //     let mut result = String::new();
    //     result.push_str(&format!(
    //         r"\sum_{{{}}} ",
    //         (0..rank_pairs.len())
    //             .into_iter()
    //             .map(|k| format!("k_{{{}}}", k))
    //             .collect::<Vec<_>>()
    //             .join(", ")
    //     ));

    //     result.push_str(&format!(
    //         "{}_{{{}}}",
    //         lhs.tex_code(),
    //         rank_pairs
    //             .iter()
    //             .enumerate()
    //             .map(|(k, &rank_pair)| format!("i_{{{}}} = k_{{{}}}", rank_pair.0, k))
    //             .collect::<Vec<_>>()
    //             .join(", ")
    //     ));

    //     result.push_str(&format!(
    //         "{}_{{{}}}",
    //         rhs.tex_code(),
    //         rank_pairs
    //             .iter()
    //             .enumerate()
    //             .map(|(k, &rank_pair)| format!("i_{{{}}} = k_{{{}}}", rank_pair.1, k))
    //             .collect::<Vec<_>>()
    //             .join(", ")
    //     ));

    //     format!("{{{}}}", result)
    // }

    pub(crate) fn diff_inner_prod(
        symbols: &[&str],
        v: &Vec<TensorExpression>,
        rank_combinations: &Vec<HashMap<usize, String>>,
    ) -> Vec<TensorExpression> {
        let mut result = v[0]
            .clone()
            .differential(symbols)
            .into_iter()
            .map(|d| TensorExpression::InnerProd {
                v: once(d).chain(v[1..].iter().cloned()).collect(),
                rank_combinations: rank_combinations.clone(),
            })
            .collect::<Vec<_>>();

        for i in 1..v.len() {
            result.iter_mut().for_each(|r| {
                *r = r.clone()
                    + TensorExpression::InnerProd {
                        v: once(v[i].clone())
                            .chain(v[0..i].iter().cloned())
                            .chain(v[i + 1..].iter().cloned())
                            .collect(),
                        rank_combinations: rank_combinations.clone(),
                    }
            });
        }

        result
    }

    pub(crate) fn rust_code_inner_prod(
        v: &Vec<TensorExpression>,
        rank_combinations: &Vec<HashMap<usize, String>>,
        parentheses: bool,
    ) -> String {
        todo!()
    }

    pub(crate) fn tex_code_inner_prod(
        v: &Vec<TensorExpression>,
        rank_combinations: &Vec<HashMap<usize, String>>,
    ) -> String {
        let mut identifier = HashMap::<String, usize>::new();

        for i in 0..v.len() {
            for (_, id) in rank_combinations[i].iter() {
                if !identifier.contains_key(id) {
                    identifier.insert(id.clone(), identifier.len());
                }
            }
        }

        let mut result = String::new();
        result.push_str(&format!(
            r"\sum_{{{}}}",
            identifier
                .iter()
                .map(|(_, l)| format!("{}", next_char('i', *l as u32)))
                .collect::<Vec<_>>()
                .join(", ")
        ));

        for i in 0..v.len() {
            result.push_str(&format!(
                "{}_{{{}}}",
                v[i].tex_code(),
                rank_combinations[i]
                    .iter()
                    .map(|(j, id)| format!("[{}] = {}", j, next_char('i', identifier[id] as u32)))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }

        format!("{{{}}}", result)
    }
}
