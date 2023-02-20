use crate::{BracketsLevel, Expression};
use std::{collections::HashMap, ops::Neg};

impl Neg for Expression {
    type Output = Self;

    fn neg(self) -> Self::Output {
        if let Expression::Constant(mut v) = self {
            v.elems_mut().into_iter().for_each(|v| *v = -*v);
            return v.into();
        }
        if let Expression::Neg(v) = self {
            return *v;
        }

        Expression::Neg(self.into())
    }
}

impl Expression {
    pub(crate) fn diff_neg(v: &Box<Expression>, variable_ids: &[&str]) -> Vec<Expression> {
        v.differential(variable_ids)
            .into_iter()
            .map(|e| -e)
            .collect()
    }

    pub(crate) fn tex_code_neg(v: &Box<Expression>, symbols: &HashMap<&str, &str>) -> String {
        format!("{{-{}}}", v._tex_code(symbols, BracketsLevel::ForOperation))
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::HashMap,
        ops::{Add, Neg},
    };

    use opensrdk_linear_algebra::sparse::SparseTensor;

    use crate::Expression;

    #[test]
    fn it_works() {
        let a1 = 5.0f64;
        let b1 = vec![a1; 8];
        let mut hash1 = HashMap::new();
        hash1.insert(vec![3, 2, 1], 2.0);
        hash1.insert(vec![1usize; 3], 3.0);
        hash1.insert(vec![4usize; 3], 4.0);
        hash1.insert(vec![5usize; 3], 2.0);
        let c1 = SparseTensor::from(vec![6usize; 3], hash1).unwrap();

        let ea1 = Expression::from(a1);
        let eb1 = Expression::from(b1.clone());
        let ec1 = Expression::from(c1.clone());

        let ea = ea1.neg();
        let eb = eb1.neg();
        let ec = ec1.neg();

        let a = Expression::from(-a1);
        let b = Expression::from(b1.iter().map(|j| -j).collect::<Vec<f64>>());
        //let c = Expression::from(-c1);

        assert_eq!(ea, a);
        assert_eq!(eb, b);
        //assert_eq!(ec, c);
        println!("{:?}", ec);
    }
}
