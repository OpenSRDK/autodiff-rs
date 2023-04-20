use crate::{Expression, ExpressionArray};
use std::collections::HashMap;

pub fn new_partial_variable(v: ExpressionArray) -> Expression {
    Expression::PartialVariable(v)
}

impl Expression {
    pub(crate) fn diff_partial_variable(
        v: &ExpressionArray,
        variable_ids: &[&str],
    ) -> Vec<Expression> {
        variable_ids
            .iter()
            .map(|&variable_id| {
                new_partial_variable(ExpressionArray::from_factory(
                    v.sizes().to_vec(),
                    |indices| v[indices].differential(&[variable_id])[0].clone(),
                ))
            })
            .collect()
    }
}
#[cfg(test)]
mod tests {
    use std::collections::HashMap;
    use std::collections::HashSet;

    use crate::new_partial_variable;
    use crate::new_variable;
    use crate::ExpressionArray;

    #[test]
    fn it_works() {
        let id_1 = "x";
        let e_1 = new_variable((id_1).to_string());
        let id_2 = "y";
        let e_2 = new_variable((id_2).to_string());
        let id_3 = "z";
        let e_3 = new_variable((id_3).to_string());

        let expression_vec = vec![e_1, e_2, e_3];

        let factory = |i: &[usize]| expression_vec[i[0].clone()].clone();
        let sizes: Vec<usize> = vec![3usize];

        let theta_array_orig = ExpressionArray::from_factory(sizes, factory);
        let theta_array = new_partial_variable(theta_array_orig);
        println!("{:?}", theta_array);
    }
}
