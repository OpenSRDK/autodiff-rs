use crate::{AbstractSize, ConstantValue, Expression, ExpressionArray};
use std::collections::HashMap;

impl Expression {
    pub fn assign(self, variables: &HashMap<&str, ConstantValue>) -> Expression {
        match self {
            Expression::Variable(id, sizes) => {
                let v = variables.get(id.as_str());

                match v {
                    Some(v) => {
                        if sizes != v.sizes().into_abstract_size() {
                            panic!("Variable {} has sizes {:?} but is assigned a value with sizes {:?}", id, sizes, v.sizes());
                        }
                        v.clone().into()
                    }
                    None => Expression::Variable(id.clone(), sizes.clone()),
                }
            }
            Expression::Constant(_) => self,
            Expression::PartialVariable(v) => Expression::PartialVariable(
                ExpressionArray::from_factory(v.sizes().to_vec(), |indices| {
                    v[indices].clone().assign(variables)
                }),
            ),
            Expression::Add(l, r) => l.assign(variables) + r.assign(variables),
            Expression::Sub(l, r) => l.assign(variables) - r.assign(variables),
            Expression::Mul(l, r) => l.assign(variables) * r.assign(variables),
            Expression::Div(l, r) => l.assign(variables) / r.assign(variables),
            Expression::Neg(v) => -v.assign(variables),
            Expression::Transcendental(v) => v.assign(variables),
            Expression::Tensor(v) => v.assign(variables),
            Expression::Matrix(v) => v.assign(variables),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use opensrdk_linear_algebra::sparse::SparseTensor;

    use crate::{new_variable, new_variable_tensor, AbstractSize, ConstantValue, Expression};

    #[test]
    fn it_works1() {
        let id = "x";
        let ea = new_variable((id).to_string());
        let mut hash1 = HashMap::new();
        hash1.insert(id, ConstantValue::Scalar(2.0));
        let result = ea.assign(&hash1);
        assert_eq!(result, Expression::from(ConstantValue::Scalar(2.0)))
    }

    #[test]
    fn it_works2() {
        let id = "x";
        let mut hash1 = HashMap::new();

        let mut hash = HashMap::new();
        hash.insert(vec![3usize; 8], 2.0);
        hash.insert(vec![1usize; 8], 3.0);
        hash.insert(vec![4usize; 8], 4.0);
        hash.insert(vec![5usize; 8], 2.0);
        let c = SparseTensor::from(vec![6usize; 8], hash.clone()).unwrap();

        hash1.insert(id, ConstantValue::Tensor(c));

        let ec = new_variable_tensor(id.to_string(), [6usize; 8].into_abstract_size());

        let result = ec.assign(&hash1);
        assert_eq!(
            result,
            Expression::from(ConstantValue::Tensor(
                SparseTensor::from(vec![6usize; 8], hash).unwrap()
            ))
        )
    }
}
