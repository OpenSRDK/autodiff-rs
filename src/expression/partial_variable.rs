use crate::Expression;
use std::collections::HashMap;

pub fn new_partial_variable(
    sizes: Vec<usize>,
    factory: impl Fn(&[usize]) -> Expression,
) -> Expression {
    let elems = sizes
        .iter()
        .fold(Vec::<Vec<usize>>::new(), |accum, &next_size| {
            accum
                .into_iter()
                .flat_map(|acc| {
                    (0..next_size)
                        .map(|i| [&acc[..], &[i]].concat())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>()
        })
        .into_iter()
        .map(|index| {
            let value = factory(&index);
            (index, value)
        })
        .collect::<HashMap<_, _>>();

    Expression::PartialVariable(sizes, elems)
}

impl Expression {
    pub(crate) fn diff_partial_variable(
        sizes: &Vec<usize>,
        elems: &HashMap<Vec<usize>, Expression>,
        variable_ids: &[&str],
    ) -> Vec<Expression> {
        variable_ids
            .iter()
            .map(|&variable_id| {
                new_partial_variable(sizes.clone(), |index| {
                    elems[index].differential(&[variable_id])[0].clone()
                })
            })
            .collect()
    }
}
