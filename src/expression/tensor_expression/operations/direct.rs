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
        let elems = terms
            .into_iter()
            .map(|t| t.differential(symbols))
            .collect::<Vec<_>>();
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
