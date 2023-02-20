use crate::{BracketsLevel, ConstantValue, Expression};
use std::{collections::HashMap, ops::Div};

impl Div<Expression> for Expression {
    type Output = Self;

    fn div(self, rhs: Expression) -> Self::Output {
        if !self.is_same_size(&rhs) {
            panic!("Cannot add expressions of different sizes");
        }
        if let Expression::Constant(vr) = &rhs {
            if let Expression::Constant(mut vl) = self {
                vl.elems_mut()
                    .into_iter()
                    .zip(vr.elems().into_iter())
                    .for_each(|(vl, vr)| *vl = *vl / vr);
                return vl.into();
            }

            if vr == &ConstantValue::Scalar(1.0) {
                return self;
            }
        }

        Expression::Div(self.into(), rhs.into())
    }
}

impl Div<f64> for Expression {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output {
        self / Expression::Constant(ConstantValue::Scalar(rhs))
    }
}

impl Div<Expression> for f64 {
    type Output = Expression;

    fn div(self, rhs: Expression) -> Self::Output {
        Expression::Constant(ConstantValue::Scalar(self)) / rhs
    }
}

impl Expression {
    pub(crate) fn diff_div(
        l: &Box<Expression>,
        r: &Box<Expression>,
        variable_ids: &[&str],
    ) -> Vec<Expression> {
        l.differential(variable_ids)
            .into_iter()
            .zip(r.differential(variable_ids).into_iter())
            .map(|(li, ri)| {
                (li * r.as_ref().clone() - l.as_ref().clone() * ri)
                    / r.as_ref().clone().pow(2.0.into())
            })
            .collect()
    }

    pub(crate) fn tex_code_div(
        l: &Box<Expression>,
        r: &Box<Expression>,
        symbols: &HashMap<&str, &str>,
        brackets_level: BracketsLevel,
    ) -> String {
        let inner = format!(
            "{{{} / {}}}",
            l._tex_code(symbols, BracketsLevel::ForDiv),
            r._tex_code(symbols, BracketsLevel::ForDiv)
        );

        match brackets_level {
            BracketsLevel::None | BracketsLevel::ForMul => inner,
            BracketsLevel::ForDiv | BracketsLevel::ForOperation => {
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

        let ea = ea1 / ea2;
        let eb = eb1 / eb2;
        let ec = ec1 / ec2;

        let a = Expression::from(a1 / a2);
        let b = Expression::from(
            b1.iter()
                .enumerate()
                .map(|(i, j)| j / b2[i])
                .collect::<Vec<f64>>(),
        );
        //let c = Expression::from(c1 / c2);

        assert_eq!(ea, a);
        assert_eq!(eb, b);
        //assert_eq!(ec, c);
        println!("{:?}", ec);
    }
}
