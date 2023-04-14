use crate::{Expression, ExpressionArray};

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
    use crate::{new_partial_variable, new_variable_tensor, ExpressionArray, Size};

    #[test]
    fn diff_partial_variable() {
        let x = new_variable_tensor("x".to_string(), vec![Size::Many, Size::Many]);
        let y = new_variable_tensor("y".to_string(), vec![Size::Many, Size::Many]);
        let z = new_variable_tensor("z".to_string(), vec![Size::Many, Size::Many]);

        let mut a = ExpressionArray::new(vec![1, 3]);
        a[&[0, 0]] = x;
        a[&[0, 1]] = y;
        a[&[0, 2]] = z;

        let v = new_partial_variable(a);
        let diff_x = v.differential(&["x"]);
        println!("diff_x: {:?}", diff_x);
        let diff_y = v.differential(&["y"]);
        println!("diff_y: {:?}", diff_y);
        let diff_z = v.differential(&["z"]);
        println!("diff_z: {:?}", diff_z);

        assert_eq!(diff_x.len(), 1);
        assert_eq!(diff_y.len(), 1);
        assert_eq!(diff_z.len(), 1);
    }
}
