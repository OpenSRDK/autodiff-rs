pub mod assign;
pub mod differential;
pub mod operations;
pub mod operators;
pub mod rust_code;
pub mod size;
pub mod symbol;
pub mod tex_code;

use std::collections::HashMap;

pub use assign::*;
pub use differential::*;
use opensrdk_linear_algebra::tensor::sparse::SparseTensor;
pub use rust_code::*;
pub use size::*;
pub use symbol::*;
pub use tex_code::*;

use crate::Expression;

#[derive(Clone, Debug, PartialEq)]
pub enum Size {
    One,
    Many,
}

#[derive(Clone, Debug, PartialEq)]
pub enum TensorExpression {
    Symbol(String, Vec<Size>),
    Constant(SparseTensor<f64>),
    Zero,
    Add(Box<TensorExpression>, Box<TensorExpression>),
    Sub(Box<TensorExpression>, Box<TensorExpression>),
    MulScalarLhs(Box<Expression>, Box<TensorExpression>),
    MulScalarRhs(Box<TensorExpression>, Box<Expression>),
    Neg(Box<TensorExpression>),
    KroneckerDeltas(Vec<[usize; 2]>),
    InnerProd {
        terms: Vec<TensorExpression>,
        rank_combinations: Vec<HashMap<usize, String>>,
    },
}

impl TensorExpression {
    pub fn as_scalar(self) -> Expression {
        if self.sizes().iter().find(|&s| *s != Size::One).is_some() {
            panic!("TensorExpression::as_scalar: not a scalar");
        }
        Expression::DegeneratedTensor(self.into())
    }
}
