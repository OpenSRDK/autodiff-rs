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
                let elems_left = (0..i).map(|j| terms[j]).product::<Expression>();
                let elems_right = (i + 1..terms_len).map(|k| terms[k]).product::<Expression>();
                let elem_diff = terms[i].differential(symbols);

                let elems = (0..symbols_len)
                    .map(|l| elems_left * elem_diff[l] * elems_right)
                    .collect::<Vec<Expression>>();
                elems
            })
            .sum::<Expression>();

        // let mut hash = HashMap::new();

        // let a = SparseTensor::new(vec![1usize; symbols_len]);
        // let ea = Expression::from(a);
        // let tea = ea.into_tensor();

        // let mut result = tea.clone();
        // let mut elems_left = ;
        // let mut elem = tea.clone();
        // let mut elems_right = ;

        // for i in 0..terms_len {
        //   for j in 0..i {
        //     elems_left *= terms[j];
        //   }
        //   elem = terms[i].differential(symbols);
        //   for k in i + 1..terms_len {
        //     elems_right *= terms[k];
        //   }
        //   for l in 0..symbols_len{
        //     result[l] += elems_left * elem[l] * elems_right;
        //   }
        todo!()
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
    use std::{collections::HashMap, ops::Add};

    use opensrdk_linear_algebra::{sparse::SparseTensor, Matrix};

    use crate::{Expression, MatrixExpression};

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
}
