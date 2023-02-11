use crate::{Expression, Size, TensorExpression};
use opensrdk_linear_algebra::{
    generate_rank_combinations, sparse::operations::kronecker_delta::KroneckerDelta, RankIndex,
    Tensor,
};
use std::{collections::HashMap, iter::once};

type TermIndex = usize;

fn next_char(c: char, count: usize) -> char {
    std::char::from_u32(c as u32 + count as u32).unwrap_or(c)
}

pub trait InnerProd {
    fn inner_prod(self, rank_combinations: &[HashMap<RankIndex, String>]) -> TensorExpression;
}

impl<I> InnerProd for I
where
    I: Iterator<Item = TensorExpression>,
{
    fn inner_prod(self, rank_combinations: &[HashMap<RankIndex, String>]) -> TensorExpression {
        // Flatten InnerProd
        let terms = self
            .zip(rank_combinations.iter())
            .flat_map(|(t, rank_combination)| {
                if let TensorExpression::InnerProd {
                    terms: t,
                    mut rank_combinations,
                } = t
                {
                    let not_1dimension_ranks = TensorExpression::not_1dimension_ranks_in_inner_prod(
                        &t,
                        &rank_combinations,
                    );

                    for (&rank, id) in rank_combination.iter() {
                        let term_index = not_1dimension_ranks[&rank];
                        rank_combinations[term_index].insert(rank, id.to_owned());
                    }

                    return t
                        .into_iter()
                        .zip(rank_combinations.into_iter())
                        .collect::<Vec<_>>();
                }

                vec![(t, rank_combination.clone())]
            })
            .collect::<Vec<_>>();

        // Merge Zero
        if terms
            .iter()
            .find(|&(t, _)| TensorExpression::Zero.eq(t))
            .is_some()
        {
            return TensorExpression::Zero;
        }

        // Merge KroneckerDeltas
        let deltas = terms
            .iter()
            .filter_map(|(t, r)| {
                if let TensorExpression::KroneckerDeltas(rank_pairs) = t {
                    Some((rank_pairs, r))
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();
        let not_deltas = terms
            .iter()
            .filter(|(t, _)| !matches!(t, TensorExpression::KroneckerDeltas(_)))
            .collect::<Vec<_>>();

        let flatten_deltas = deltas
            .iter()
            .map(|&(t, _)| t)
            .flatten()
            .cloned()
            .collect::<Vec<_>>();
        let flatten_deltas_combination = deltas
            .iter()
            .flat_map(|(_, r)| r.iter())
            .map(|(&rank, id)| (rank, id.to_owned()))
            .collect::<HashMap<_, _>>();

        let mut new_terms = not_deltas
            .iter()
            .map(|(t, _)| t.clone())
            .collect::<Vec<_>>();
        let mut new_rank_combinations = not_deltas
            .iter()
            .map(|&(_, r)| r.clone())
            .collect::<Vec<_>>();

        if flatten_deltas.len() > 0 {
            let merged_deltas = TensorExpression::KroneckerDeltas(flatten_deltas);

            new_terms.insert(0, merged_deltas);
            new_rank_combinations.insert(0, flatten_deltas_combination);
        }

        TensorExpression::InnerProd {
            terms: new_terms,
            rank_combinations: new_rank_combinations,
        }
    }
}

impl TensorExpression {
    pub fn inner_prod(self, rhs: TensorExpression, rank_pairs: &[[RankIndex; 2]]) -> Self {
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
        // Merging Zero, KroneckerDeltas, InnerProds are done in InnerProd::inner_prod

        vec![self, rhs]
            .into_iter()
            .inner_prod(&generate_rank_combinations(rank_pairs))
    }
}

impl TensorExpression {
    pub(crate) fn diff_inner_prod(
        v: &Vec<TensorExpression>,
        rank_combinations: &Vec<HashMap<RankIndex, String>>,
        symbols: &[&str],
    ) -> Vec<TensorExpression> {
        let mut result = v[0]
            .differential(symbols)
            .into_iter()
            .map(|d| {
                once(d)
                    .chain(v[1..].iter().cloned())
                    .inner_prod(rank_combinations)
            })
            .collect::<Vec<_>>();

        for i in 1..v.len() {
            result
                .iter_mut()
                .zip(v[i].differential(symbols).into_iter())
                .for_each(|(r, d)| {
                    *r = r.clone()
                        + v[0..i]
                            .iter()
                            .cloned()
                            .chain(once(d))
                            .chain(v[i + 1..].iter().cloned())
                            .inner_prod(rank_combinations);
                });
        }

        result
    }

    pub(crate) fn rust_code_inner_prod(
        terms: &Vec<TensorExpression>,
        rank_combinations: &Vec<HashMap<RankIndex, String>>,
        parentheses: bool,
    ) -> String {
        todo!()
    }

    pub(crate) fn tex_code_inner_prod(
        terms: &Vec<TensorExpression>,
        rank_combinations: &Vec<HashMap<RankIndex, String>>,
    ) -> String {
        let mut ids = Vec::<String>::new();
        let mut id_index = HashMap::<String, usize>::new();

        for i in 0..terms.len() {
            for (_, id) in rank_combinations[i].iter() {
                if !id_index.contains_key(id) {
                    ids.push(id.clone());
                    id_index.insert(id.clone(), ids.len() - 1);
                }
            }
        }

        let mut result = String::new();
        result.push_str(&format!(
            r"\sum_{{{}}}",
            ids.iter()
                .enumerate()
                .map(|(k, _)| format!("{}", next_char('i', k)))
                .collect::<Vec<_>>()
                .join(", ")
        ));

        for i in 0..terms.len() {
            let mut sorted = rank_combinations[i].iter().collect::<Vec<_>>();
            sorted.sort_by(|a, b| a.0.cmp(b.0));
            result.push_str(&format!(
                "{}_{{{}}}",
                terms[i].tex_code(),
                sorted
                    .into_iter()
                    .map(|(j, id)| format!("[{}] = {}", j, next_char('i', id_index[id])))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }

        format!("{{{}}}", result)
    }

    pub(crate) fn size_inner_prod(
        terms: &Vec<TensorExpression>,
        rank_combinations: &Vec<HashMap<RankIndex, String>>,
    ) -> Vec<Size> {
        let max_rank = terms.iter().map(|vi| vi.sizes().len()).max().unwrap();
        let mut sizes = vec![Size::One; max_rank];

        for i in 0..terms.len() {
            let term_sizes = terms[i].sizes();

            for (rank, size) in term_sizes.iter().enumerate() {
                if sizes[rank].eq(&Size::Many) {
                    continue;
                }
                if let Some(_) = rank_combinations[i].get(&rank) {
                    continue;
                }
                sizes.insert(rank, size.clone());
            }
        }

        sizes
    }

    pub fn not_1dimension_ranks_in_inner_prod(
        terms: &Vec<TensorExpression>,
        rank_combinations: &Vec<HashMap<RankIndex, String>>,
    ) -> HashMap<RankIndex, TermIndex> {
        let mut not_1dimension_ranks = HashMap::new();

        for i in 0..terms.len() {
            let term_sizes = terms[i].sizes();
            for (rank, size) in term_sizes.iter().enumerate() {
                if let Some(_) = rank_combinations[i].get(&rank) {
                    continue;
                }

                if *size != Size::One {
                    if not_1dimension_ranks.contains_key(&rank) {
                        panic!(
                            "Rank {} is not 1-dimension in terms[{}] and terms[{}]",
                            rank,
                            not_1dimension_ranks.get(&rank).unwrap(),
                            i
                        );
                    }
                    not_1dimension_ranks.insert(rank, i);
                }
            }
        }

        not_1dimension_ranks
    }
}
