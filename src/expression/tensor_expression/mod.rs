pub mod differential;
pub mod evaluate;
pub mod operations;
pub mod operators;
pub mod rust_code;
pub mod symbol;

pub use differential::*;
pub use evaluate::*;
use opensrdk_linear_algebra::sparse::SparseTensor;
pub use rust_code::*;
pub use symbol::*;

use crate::Expression;

#[derive(Clone, Debug, PartialEq)]
pub enum TensorExpression {
    Symbol(String),
    Constant(SparseTensor<f64>),
    Zero,
    Unit,
    Add(Box<TensorExpression>, Box<TensorExpression>),
    Sub(Box<TensorExpression>, Box<TensorExpression>),
    MulScalarLhs(Box<Expression>, Box<TensorExpression>),
    MulScalarRhs(Box<TensorExpression>, Box<Expression>),
    Neg(Box<TensorExpression>),
    InnerProd {
        lhs: Box<TensorExpression>,
        rhs: Box<TensorExpression>,
        level_pairs: Vec<(usize, usize)>,
    },
    Det(Box<TensorExpression>),
}
