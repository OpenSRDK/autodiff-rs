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

    use crate::{
        new_partial_variable, new_variable, new_variable_tensor, AbstractSize, ConstantValue,
        Expression, ExpressionArray,
    };

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

    #[test]
    fn it_works3() {
        let x = new_variable("x".to_string());
        let y = new_variable("y".to_string());

        let expression = x.clone().sin() + y.clone().cos().exp();

        let theta_map = &mut HashMap::new();
        theta_map.insert("x", ConstantValue::Scalar(3f64));
        theta_map.insert("y", ConstantValue::Scalar(7f64));

        println!("{:#?}", expression);
        println!("{:#?}", expression.assign(&*theta_map));
    }

    #[test]
    fn it_works4() {
        let a = new_variable("a".to_string());
        let b = new_variable("b".to_string());
        let c = new_variable("c".to_string());
        let d = new_variable("d".to_string());
        let e = new_variable("e".to_string());
        let f = new_variable("f".to_string());

        let add_1 = a.clone().sin() + b.clone().cos().exp();
        let add_2 = add_1.clone() * c.clone();
        let add_3 = d.clone().sin() - e.clone().cos().exp();
        let add_4 = add_3.clone() / f.clone();
        let add_5 = add_2.clone() + add_4.clone();

        let theta_map = &mut HashMap::new();
        theta_map.insert("a", ConstantValue::Scalar(3f64));
        theta_map.insert("b", ConstantValue::Scalar(7f64));
        theta_map.insert("c", ConstantValue::Scalar(-3f64));
        theta_map.insert("d", ConstantValue::Scalar(-7f64));
        theta_map.insert("e", ConstantValue::Scalar(4f64));
        theta_map.insert("f", ConstantValue::Scalar(6f64));

        println!("{:#?}", add_1);
        println!("{:#?}", add_1.assign(&*theta_map));
        println!("{:#?}", add_5);
        println!("{:#?}", add_5.assign(&*theta_map));
    }
}
