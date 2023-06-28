use crate::{BracketsLevel, ConstantValue, Expression, ExpressionArray, Size, TensorExpression};
use opensrdk_linear_algebra::{generate_rank_combinations, RankIndex, Tensor};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use std::{
    collections::{HashMap, HashSet},
    iter::once,
    ops::Add,
};

type TermIndex = usize;

fn next_char(c: char, count: usize) -> char {
    std::char::from_u32(c as u32 + count as u32).unwrap_or(c)
}

pub trait DotProduct {
    fn dot_product(self, rank_combinations: &[HashMap<RankIndex, String>]) -> Expression;
}

impl<I> DotProduct for I
where
    I: Iterator<Item = Expression>,
{
    fn dot_product(self, rank_combinations: &[HashMap<RankIndex, String>]) -> Expression {
        // Flatten InnerProd
        let terms = self //self は ExpressionのIterator
            .zip(rank_combinations.iter())
            .flat_map(|(t, rank_combination)| {
                if let Expression::Tensor(t) = &t {
                    if let TensorExpression::DotProduct {
                        terms: t,
                        rank_combinations,
                    } = t.as_ref()
                    {
                        let t = t.clone();
                        let mut rank_combinations = rank_combinations.clone();
                        let not_1dimension_ranks =
                            TensorExpression::not_1dimension_ranks_in_dot_product(
                                &t,
                                &rank_combinations,
                            );

                        for (&rank, id) in rank_combination.iter() {
                            if let Some(&term_index) = not_1dimension_ranks.get(&rank) {
                                rank_combinations[term_index].insert(rank, id.to_owned());
                            }
                        }

                        return t
                            .into_iter()
                            .zip(rank_combinations.into_iter())
                            .collect::<Vec<_>>();
                    }
                }

                vec![(t, rank_combination.clone())]
            })
            .collect::<Vec<_>>();

        if terms.iter().find(|&t| &t.0 == &0.0.into()).is_some() {
            return 0.0.into();
        }

        // Merge KroneckerDeltas
        let deltas = terms
            .iter()
            .filter_map(|(t, r)| {
                if let Expression::Tensor(t) = t {
                    if let TensorExpression::KroneckerDeltas(rank_pairs) = t.as_ref() {
                        return Some((rank_pairs.clone(), r));
                    }
                }

                None
            })
            .collect::<Vec<_>>();
        let not_deltas = terms
            .iter()
            .filter(|(t, _)| {
                if let Expression::Tensor(t) = t {
                    if let &TensorExpression::KroneckerDeltas(_) = t.as_ref() {
                        return false;
                    }
                }

                true
            })
            .collect::<Vec<_>>();

        let flatten_deltas = deltas
            .iter()
            .map(|(t, _)| t)
            .flatten()
            .cloned()
            .collect::<Vec<_>>();
        let flatten_deltas_combination = deltas
            .iter()
            .flat_map(|(_, r)| r.iter())
            .map(|(&rank, id)| (rank, id.to_owned()))
            .collect::<HashMap<_, _>>();

        println!(
            "{:?}",
            (flatten_deltas.clone(), flatten_deltas_combination.clone())
        );

        let mut new_terms = not_deltas
            .iter()
            .map(|(t, _)| t.clone())
            .collect::<Vec<_>>();
        let mut new_rank_combinations = not_deltas
            .iter()
            .map(|&(_, r)| r.clone())
            .collect::<Vec<_>>();

        // TODO: Merge constants

        if flatten_deltas.len() > 0 {
            let merged_deltas = TensorExpression::KroneckerDeltas(flatten_deltas);

            new_terms.clone().insert(0, merged_deltas.into());
            new_rank_combinations.insert(0, flatten_deltas_combination);
        }

        let test_const = new_terms
            .iter()
            .map(|i| {
                if let Expression::Constant(_) = i {
                    0usize
                } else {
                    1usize
                }
            })
            .sum::<usize>();

        let result = if let 0usize = test_const {
            //新しいrank_combinationから、重複のない文字列のリストを作成する。
            let list_string: HashSet<&String> = new_rank_combinations
                .iter()
                .map(|i| {
                    let elem_list = i.values().clone().collect::<Vec<&std::string::String>>();
                    elem_list
                })
                .collect::<Vec<_>>()
                .concat()
                .into_iter()
                .collect();

            //抜き出した文字列を利用し、new_termsからそれぞれの文字列と一致するrankを抜き出し、そのrankをまとめる。
            let ranks_fixed = list_string
                .iter()
                .map(|hash| {
                    new_terms
                        .iter()
                        .zip(rank_combinations.iter())
                        .map(|(t, rank_combination)| {
                            let rank_orig = rank_combination
                                .values()
                                .zip(rank_combination.keys())
                                .filter(|(value, key)| value == hash)
                                .collect::<Vec<_>>();
                            let mut rank = 0usize;
                            if rank_orig.len() == 1usize {
                                rank = rank_orig[0].1.clone();
                            } else {
                                todo!() //Error
                            }
                            rank
                        })
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>();

            let ranks_fixed_dummy = list_string
                .iter()
                .map(|hash| {
                    new_terms
                        .iter()
                        .zip(rank_combinations.iter())
                        .map(|(t, rank_combination)| {
                            let rank_orig = rank_combination
                                .values()
                                .zip(rank_combination.keys())
                                .filter(|(value, key)| value == hash)
                                .collect::<Vec<_>>();
                            let mut dummy = 0usize;
                            if rank_orig.len() != 0usize {
                                dummy = rank_orig[0].1.clone();
                            } else {
                                todo!() //Error
                            }
                            dummy
                        })
                        .collect::<Vec<usize>>()
                })
                .collect::<Vec<Vec<usize>>>();

            //new_termsの各termが持つsizeをvecにまとめる。
            let sizes = new_terms
                .iter()
                .map(|term| match term {
                    Expression::Constant(a) => a.sizes(),
                    Expression::Variable(_, _) => todo!(),
                    Expression::PartialVariable(_) => todo!(),
                    Expression::Add(_, _) => todo!(),
                    Expression::Sub(_, _) => todo!(),
                    Expression::Mul(_, _) => todo!(),
                    Expression::Div(_, _) => todo!(),
                    Expression::Neg(_) => todo!(),
                    Expression::Transcendental(_) => todo!(),
                    Expression::Tensor(_) => todo!(),
                    Expression::Matrix(_) => todo!(),
                })
                .collect::<Vec<Vec<usize>>>();

            //sizesから、一致するrank以外の部分のsizeをまとめる。
            let size_without_fixed = sizes
                .iter()
                .zip(ranks_fixed.iter())
                .map(|size| {})
                .collect::<Vec<Vec<usize>>>();

            //sizesのうちから、最大の物を取り除いている。
            let max_len_size = sizes.iter().map(|i| i.len()).max().unwrap();

            //new_termsを、hashmapの形に直して格納。なお、sizesの各vecの長さを揃えるために、1を付け加えている（がいらないかも）。
            let terms_vec = new_terms
                .iter()
                .map(|term| match term {
                    Expression::Constant(a) => match a {
                        ConstantValue::Scalar(_) => {
                            let mut hashmap_s = HashMap::new();
                            hashmap_s.insert(vec![0usize; max_len_size], a.elems()[0]);
                            hashmap_s.clone()
                        }
                        ConstantValue::Tensor(v) => {
                            let mut hashmap_m = HashMap::new();
                            let hash = v.elems().clone();
                            let keys = hash.keys();
                            let values = hash.values();
                            //hash.values().zip(hash.keys()).map(|(value, key)| {
                            for i in 0..hash.keys().len() {
                                let key_next = [
                                    keys.clone().collect::<Vec<&Vec<usize>>>()[i].clone(),
                                    vec![0usize; max_len_size - keys.len()],
                                ]
                                .concat();
                                hashmap_m.insert(
                                    key_next,
                                    values.clone().collect::<Vec<&f64>>()[i].clone(),
                                );
                            }
                            hashmap_m
                        }
                        ConstantValue::Matrix(_) => {
                            let mut hashmap_m = HashMap::new();
                            let size = a.sizes();
                            for i in (0..size[0]) {
                                for j in (0..size[1]) {
                                    let key =
                                        [vec![(i - 1), (j - 1)], vec![0usize; max_len_size - 2]]
                                            .concat();
                                    hashmap_m.insert(key, a.elems()[(i - 1) * size[1] + (j - 1)]);
                                }
                            }
                            hashmap_m.clone()
                        }
                    },
                    Expression::Variable(_, _) => todo!(),
                    Expression::PartialVariable(_) => todo!(),
                    Expression::Add(_, _) => todo!(),
                    Expression::Sub(_, _) => todo!(),
                    Expression::Mul(_, _) => todo!(),
                    Expression::Div(_, _) => todo!(),
                    Expression::Neg(_) => todo!(),
                    Expression::Transcendental(_) => todo!(),
                    Expression::Tensor(_) => todo!(),
                    Expression::Matrix(_) => todo!(),
                })
                .collect::<Vec<HashMap<Vec<usize>, f64>>>();

            let constant = terms_vec.iter();

            //Expression::from(constant)
            Expression::from(5f64) //TODO: implemet to LSigma of muls.
        } else {
            TensorExpression::DotProduct {
                terms: new_terms.clone(),
                rank_combinations: new_rank_combinations,
            }
            .into()
        };

        result
    }
}

impl Expression {
    pub fn dot(self, rhs: Expression, rank_pairs: &[[RankIndex; 2]]) -> Expression {
        if let (Expression::PartialVariable(vl), Expression::PartialVariable(vr)) = (&self, &rhs) {
            // if vl.sizes() == vr.sizes() {
            //     panic!("Mistach Sizes of Variables");
            // }

            return Expression::PartialVariable(ExpressionArray::from_factory(
                vr.sizes().to_vec(),
                |indices| {
                    vec![vl[indices].clone(), vr[indices].clone()]
                        .into_iter()
                        .dot_product(&generate_rank_combinations(rank_pairs))
                },
            ));
        }

        vec![self, rhs]
            .into_iter()
            .dot_product(&generate_rank_combinations(rank_pairs))
    }
}

impl TensorExpression {
    pub(crate) fn diff_dot_product(
        terms: &Vec<Expression>,
        rank_combinations: &Vec<HashMap<RankIndex, String>>,
        symbols: &[&str],
    ) -> Vec<Expression> {
        let mut result = terms[0]
            .differential(symbols)
            .into_iter()
            .map(|d| {
                once(d)
                    .chain(terms[1..].iter().cloned())
                    .dot_product(rank_combinations)
            })
            .collect::<Vec<_>>();

        for i in 1..terms.len() {
            result
                .iter_mut()
                .zip(terms[i].differential(symbols).into_iter())
                .for_each(|(r, d)| {
                    *r = r.clone()
                        + terms[0..i]
                            .iter()
                            .cloned()
                            .chain(once(d))
                            .chain(terms[i + 1..].iter().cloned())
                            .dot_product(rank_combinations);
                });
        }

        result
    }

    pub(crate) fn tex_code_dot_product(
        terms: &Vec<Expression>,
        rank_combinations: &Vec<HashMap<RankIndex, String>>,
        symbols: &HashMap<&str, &str>,
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
                terms[i]._tex_code(symbols, BracketsLevel::ForMul),
                sorted
                    .into_iter()
                    .map(|(j, id)| format!("[{}] = {}", j, next_char('i', id_index[id])))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }

        format!("{{{}}}", result)
    }

    pub(crate) fn size_dot_product(
        terms: &Vec<Expression>,
        rank_combinations: &Vec<HashMap<RankIndex, String>>,
    ) -> Vec<Size> {
        let max_rank = terms.iter().map(|vi| vi.sizes().len()).max().unwrap();
        let mut sizes = vec![Size::One; max_rank];

        for i in 0..terms.len() {
            let term_sizes = terms[i].sizes();

            for (rank, size) in term_sizes.iter().enumerate() {
                if sizes[rank] == Size::Many {
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

    pub fn not_1dimension_ranks_in_dot_product(
        terms: &Vec<Expression>,
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
