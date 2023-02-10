pub mod differential;
pub mod evaluate;
pub mod operations;
pub mod operators;
pub mod rust_code;
pub mod symbol;
pub mod tex_code;

use std::collections::HashMap;

pub use differential::*;
pub use evaluate::*;
use opensrdk_linear_algebra::tensor::sparse::SparseTensor;
pub use rust_code::*;
pub use symbol::*;
pub use tex_code::*;

use crate::Expression;

#[derive(Clone, Debug, PartialEq)]
pub enum TensorExpression {
    Symbol(String, usize),
    Constant(SparseTensor<f64>),
    Zero,
    Add(Box<TensorExpression>, Box<TensorExpression>),
    Sub(Box<TensorExpression>, Box<TensorExpression>),
    MulScalarLhs(Box<Expression>, Box<TensorExpression>),
    MulScalarRhs(Box<TensorExpression>, Box<Expression>),
    Neg(Box<TensorExpression>),
    KroneckerDeltas(Vec<[usize; 2]>),
    InnerProd {
        v: Vec<TensorExpression>,
        rank_combinations: Vec<HashMap<usize, String>>,
    },
}

impl TensorExpression {
    pub fn elem(self, indices: Vec<usize>) -> Expression {
        Expression::TensorElement(self.into(), indices)
    }
}
