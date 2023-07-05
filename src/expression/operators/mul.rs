use crate::{BracketsLevel, ConstantValue, Expression, ExpressionArray, TranscendentalExpression};
use std::{collections::HashMap, ops::Mul};

impl Mul<Expression> for Expression {
    type Output = Self;

    fn mul(self, rhs: Expression) -> Self::Output {
        if !self.is_same_size(&rhs) {
            panic!("Cannot add expressions of different sizes");
        }
        // Merge constant

        if let (Expression::PartialVariable(vl), Expression::PartialVariable(vr)) = (&self, &rhs) {
            // if vl.sizes() == vr.sizes() {
            //     panic!("Mistach Sizes of Variables");
            // }

            return Expression::PartialVariable(ExpressionArray::from_factory(
                vr.sizes().to_vec(),
                |indices| vl[indices].clone().mul(vr[indices].clone()),
            ));
        }

        // if let Expression::PartialVariable(vr) = &rhs {
        //     return Expression::PartialVariable(ExpressionArray::from_factory(
        //         vr.sizes().to_vec(),
        //         |indices| self.clone().mul(vr[indices].clone()),
        //     ));
        // }

        // if let Expression::PartialVariable(vl) = &self {
        //     return Expression::PartialVariable(ExpressionArray::from_factory(
        //         vl.sizes().to_vec(),
        //         |indices| vl[indices].clone().mul(rhs.clone()),
        //     ));
        // }

        if let Expression::Constant(vl) = &self {
            if vl == &ConstantValue::Scalar(0.0) {
                return 0.0.into();
            }
            if vl == &ConstantValue::Scalar(1.0) {
                return rhs;
            }
            if let Expression::Constant(vr) = rhs {
                return vl.mul(vr).into();
            }
        }
        if let Expression::Constant(vr) = &rhs {
            if vr == &ConstantValue::Scalar(0.0) {
                return 0.0.into();
            }
            if vr == &ConstantValue::Scalar(1.0) {
                return self;
            }
        }
        // Merge pow
        if let Expression::Transcendental(vl) = &self {
            if let TranscendentalExpression::Pow(vl, el) = vl.as_ref() {
                if let Expression::Transcendental(vr) = &rhs {
                    if let TranscendentalExpression::Pow(vr, er) = vr.as_ref() {
                        if vl.as_ref() == vr.as_ref() {
                            return vl.clone().pow(*el.clone() + *er.clone());
                        }
                    }
                }
                if vl.as_ref() == &rhs {
                    let one: Expression = 1.0.into();
                    return vl.clone().pow(*el.clone() + one);
                }
            }
        }
        if let Expression::Transcendental(vr) = &rhs {
            if let TranscendentalExpression::Pow(vr, er) = vr.as_ref() {
                if vr.as_ref() == &self {
                    let one: Expression = 1.0.into();
                    return vr.clone().pow(*er.clone() + one);
                }
            }
        }

        Expression::Mul(self.into(), rhs.into())
    }
}

impl Mul<Expression> for f64 {
    type Output = Expression;

    fn mul(self, rhs: Expression) -> Self::Output {
        Expression::Constant(ConstantValue::Scalar(self)) * rhs
    }
}

impl Mul<f64> for Expression {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        self * Expression::Constant(ConstantValue::Scalar(rhs))
    }
}

impl Expression {
    pub(crate) fn diff_mul(
        l: &Box<Expression>,
        r: &Box<Expression>,
        variable_ids: &[&str],
    ) -> Vec<Expression> {
        l.differential(variable_ids)
            .into_iter()
            .zip(r.differential(variable_ids).into_iter())
            .map(|(li, ri)| li * r.as_ref().clone() + l.as_ref().clone() * ri)
            .collect()
    }

    pub(crate) fn tex_code_mul(
        l: &Box<Expression>,
        r: &Box<Expression>,
        symbols: &HashMap<&str, &str>,
        brackets_level: BracketsLevel,
    ) -> String {
        let inner = format!(
            r"{{{} \times {}}}",
            l._tex_code(symbols, BracketsLevel::ForMul),
            r._tex_code(symbols, BracketsLevel::ForMul)
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

        let ea = ea1 * ea2;
        let eb = eb1 * eb2;
        let ec = ec1 * ec2;

        let a = Expression::from(a1 * a2);
        let b = Expression::from(
            b1.iter()
                .enumerate()
                .map(|(i, j)| j * b2[i])
                .collect::<Vec<f64>>(),
        );
        let c = Expression::from(c1 * c2);

        assert_eq!(ea, a);
        assert_eq!(eb, b);
        assert_eq!(ec, c);
    }
}
