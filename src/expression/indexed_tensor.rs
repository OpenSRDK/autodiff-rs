use crate::Expression;
use std::collections::HashMap;

pub fn new_indexed_tensor(
    sizes: Vec<usize>,
    factory: impl Fn(&[usize]) -> Expression,
) -> Expression {
    // let elems = sizes
    //     .iter()
    //     .map(|&s| s)
    //     .multi_cartesian_product()
    //     .map(|index| (index.clone(), factory(&index)))
    //     .collect::<HashMap<_, _>>();
    todo!();
    // Expression::IndexedTensor(sizes, elems)
}

impl Expression {
    pub(crate) fn diff_indexed_tensor(
        sizes: &Vec<usize>,
        elems: &HashMap<Vec<usize>, Expression>,
        variable_ids: &[&str],
    ) -> Vec<Expression> {
        // new_indexed_tensor(sizes.clone(), |index| {})
        todo!()
    }
}
