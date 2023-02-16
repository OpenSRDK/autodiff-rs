use crate::{BracketsLevel, Expression, Size, TensorExpression};
use opensrdk_linear_algebra::{generate_rank_combinations, RankIndex};
use std::{collections::HashMap, iter::once};

pub trait DirectProduct {
    fn direct_product(self) -> Expression;
}

impl<I> DirectProduct for I
where
    I: Iterator<Item = Expression>,
{
    fn direct_product(self) -> Expression {
        todo!()
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
        todo!()
    }

    pub(crate) fn tex_code_direct_product(
        terms: &Vec<Expression>,
        symbols: &HashMap<&str, &str>,
    ) -> String {
        todo!()
    }

    pub(crate) fn size_direct_product(terms: &Vec<Expression>) -> Vec<Size> {
        todo!()
    }
}
