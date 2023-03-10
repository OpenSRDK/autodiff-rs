use opensrdk_linear_algebra::sparse::SparseTensor;

use crate::{BracketsLevel, Expression, Size, TensorExpression};
use std::{collections::HashMap, iter::once};

pub trait DirectProduct {
    fn direct_product(self) -> Expression;
}

impl<I> DirectProduct for I
where
    I: Iterator<Item = Expression>,
{
    fn direct_product(self) -> Expression {
        TensorExpression::DirectProduct(self.collect()).into()
    }
}

impl Expression {
    pub fn direct(self, rhs: Expression) -> Expression {
        vec![self, rhs].into_iter().direct_product()
    }
}

impl TensorExpression {
    pub(crate) fn diff_direct_product(
        terms: &Vec<Expression>,
        symbols: &[&str],
    ) -> Vec<Expression> {
        let terms_len = terms.len();
        let symbols_len = symbols.len();

        let result = (0..terms_len)
            .map(|i| {
                let elems_left = (0..i).map(|j| terms[j].clone()).direct_product();
                let elems_right = (i + 1..terms_len)
                    .map(|k| terms[k].clone())
                    .direct_product();
                let elem_diff = terms[i].differential(symbols);

                let elems = (0..symbols_len)
                    .map(|l| {
                        elems_left
                            .clone()
                            .direct(elem_diff[l].clone())
                            .direct(elems_right.clone())
                    })
                    .collect::<Vec<Expression>>();
                elems
            })
            .fold(vec![Expression::from(0f64); symbols_len], |sum, x| {
                let result_orig = (0..symbols_len)
                    .map(|m| sum[m].clone() + x[m].clone())
                    .collect::<Vec<Expression>>();
                result_orig
            });
        result
    }

    pub(crate) fn tex_code_direct_product(
        terms: &Vec<Expression>,
        symbols: &HashMap<&str, &str>,
        brackets_level: BracketsLevel,
    ) -> String {
        let inner = terms
            .into_iter()
            .map(|t| t.tex_code(symbols))
            .collect::<Vec<_>>()
            .join(r" \otimes ");

        match brackets_level {
            BracketsLevel::None => inner,
            BracketsLevel::ForMul | BracketsLevel::ForDiv | BracketsLevel::ForOperation => {
                format!(r"\left({}\right)", inner)
            }
        }
    }

    pub(crate) fn size_direct_product(terms: &Vec<Expression>) -> Vec<Size> {
        terms
            .into_iter()
            .map(|t| t.sizes())
            .fold(vec![], |mut acc, next| {
                if acc.len() < next.len() {
                    for i in 0..acc.len() {
                        if next[i] == Size::Many {
                            acc[i] = next[i];
                        }
                    }
                    acc.extend(next[acc.len()..].iter().copied());
                } else {
                    for i in 0..next.len() {
                        if next[i] == Size::Many {
                            acc[i] = next[i];
                        }
                    }
                }
                acc
            })
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::{HashMap, HashSet},
        ops::Add,
    };

    use opensrdk_linear_algebra::{sparse::SparseTensor, Matrix};

    use crate::{new_variable, Expression, MatrixExpression, TensorExpression};

    #[test]
    fn it_works() {
        let mut hash1 = HashMap::new();
        hash1.insert(vec![3usize; 8], 2.0);
        hash1.insert(vec![1usize; 8], 3.0);
        hash1.insert(vec![4usize; 8], 4.0);
        hash1.insert(vec![5usize; 8], 2.0);
        let a = SparseTensor::from(vec![6usize; 8], hash1).unwrap();

        let ea = Expression::from(a);

        let mut hash2 = HashMap::new();
        hash2.insert(vec![3usize; 8], 2.0);
        hash2.insert(vec![2usize; 8], 3.0);
        hash2.insert(vec![4usize; 8], 1.0);
        let b = SparseTensor::from(vec![6usize; 8], hash2).unwrap();

        let eb = Expression::from(b);

        let dp = ea.direct(eb);

        println!("{:?}", dp);
    }

    #[test]
    fn it_works1() {
        let mut hash1 = HashMap::new();
        hash1.insert(vec![3usize; 8], 2.0);
        hash1.insert(vec![1usize; 8], 3.0);
        hash1.insert(vec![4usize; 8], 4.0);
        hash1.insert(vec![5usize; 8], 2.0);
        let a = SparseTensor::from(vec![6usize; 8], hash1).unwrap();

        let ea = Expression::from(a);

        let mut hash2 = HashMap::new();
        hash2.insert(vec![3usize; 8], 2.0);
        hash2.insert(vec![2usize; 8], 3.0);
        hash2.insert(vec![4usize; 8], 1.0);
        let b = SparseTensor::from(vec![6usize; 8], hash2).unwrap();

        let eb = Expression::from(b);

        let id = "x";
        let ec = new_variable((id).to_string());

        let ids = &["x", "y"];

        let diff_dp = TensorExpression::diff_direct_product(&vec![ec.clone(), ea, eb, ec], ids);
        println!("{:?}", diff_dp);

        let tex_symbols = vec![("x", "y")].into_iter().collect();
        let tex_x = diff_dp[0].tex_code(&tex_symbols);
        let tex_y = diff_dp[1].tex_code(&tex_symbols);

        println!("{:?}", tex_x);
        println!("{:?}", tex_y);
    }
}
