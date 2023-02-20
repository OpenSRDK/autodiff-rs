use crate::{BracketsLevel, ConstantValue, Expression};
use std::{collections::HashMap, ops::Add};

impl Add<Expression> for Expression {
    type Output = Self;

    fn add(self, rhs: Expression) -> Self::Output {
        if !self.is_same_size(&rhs) {
            println!("{:#?}", self);
            println!("{:#?}", rhs);
            panic!("Cannot add expressions of different sizes");
        }
        if let Expression::Constant(vl) = &self {
            if let Expression::Constant(mut vr) = rhs {
                vl.elems()
                    .into_iter()
                    .zip(vr.elems_mut().into_iter())
                    .for_each(|(vl, vr)| *vr = vl + *vr);
                return vr.into();
            }
            if vl == &ConstantValue::Scalar(0.0) {
                return rhs;
            }
        }
        if let Expression::Constant(vr) = &rhs {
            if vr == &ConstantValue::Scalar(0.0) {
                return self;
            }
        }

        Expression::Add(self.into(), rhs.into())
    }
}

impl Add<f64> for Expression {
    type Output = Self;

    fn add(self, rhs: f64) -> Self::Output {
        self + Expression::Constant(ConstantValue::Scalar(rhs))
    }
}

impl Add<Expression> for f64 {
    type Output = Expression;

    fn add(self, rhs: Expression) -> Self::Output {
        Expression::Constant(ConstantValue::Scalar(self)) + rhs
    }
}

impl Expression {
    pub(crate) fn diff_add(
        l: &Box<Expression>,
        r: &Box<Expression>,
        variable_ids: &[&str],
    ) -> Vec<Expression> {
        l.differential(variable_ids)
            .into_iter()
            .zip(r.differential(variable_ids).into_iter())
            .map(|(li, ri)| li + ri)
            .collect()
    }

    pub(crate) fn tex_code_add(
        l: &Box<Expression>,
        r: &Box<Expression>,
        symbols: &HashMap<&str, &str>,
        brackets_level: BracketsLevel,
    ) -> String {
        let inner = format!(
            "{{{} + {}}}",
            l._tex_code(symbols, BracketsLevel::None),
            r._tex_code(symbols, BracketsLevel::None)
        );

        match brackets_level {
            BracketsLevel::None => inner,
            BracketsLevel::ForMul | BracketsLevel::ForDiv | BracketsLevel::ForOperation => {
                format!(r"\left({}\right)", inner)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{collections::HashMap, ops::Add};

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

        let a2 = 5.0f64;
        let b2 = vec![a2; 8];
        let mut hash2 = HashMap::new();
        hash2.insert(vec![3usize; 3], 2.0);
        hash2.insert(vec![1usize; 3], 3.0);
        hash2.insert(vec![2, 1, 1], 4.0);
        hash2.insert(vec![5usize; 3], 2.0);
        let c2 = SparseTensor::from(vec![6usize; 3], hash2).unwrap();

        let ea2 = Expression::from(a2);
        let eb2 = Expression::from(b2.clone());
        let ec2 = Expression::from(c2.clone());

        let ea = ea1 + ea2;
        let eb = eb1 + eb2;
        let ec = ec1 + ec2;

        let a = Expression::from(a1 + a2);
        let b = Expression::from(
            b1.iter()
                .enumerate()
                .map(|(i, j)| b2[i] + j)
                .collect::<Vec<f64>>(),
        );
        //let c = Expression::from(c1 + c2);

        assert_eq!(ea, a);
        assert_eq!(eb, b);
        //assert_eq!(ec, c);
        println!("{:?}", ec);
    }
}
