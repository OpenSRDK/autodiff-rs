use crate::Expression;
use opensrdk_linear_algebra::Matrix;

pub mod as_scalar;
pub mod differential;
pub mod evaluate;
pub mod operations;
pub mod operators;
pub mod rust_code;
pub mod symbol;

pub use as_scalar::*;
pub use differential::*;
pub use evaluate::*;
pub use rust_code::*;
pub use symbol::*;

#[derive(Clone, Debug, PartialEq)]
pub enum MatrixExpression {
    Symbol(String),
    Constant(Matrix),
    Zero,
    Unit,
    Add(Box<MatrixExpression>, Box<MatrixExpression>),
    Sub(Box<MatrixExpression>, Box<MatrixExpression>),
    Mul(Box<MatrixExpression>, Box<MatrixExpression>),
    MulScalar(Box<Expression>, Box<MatrixExpression>),
    Neg(Box<MatrixExpression>),
    Pow(Box<MatrixExpression>, i32),
    T(Box<MatrixExpression>),
    Det(Box<MatrixExpression>),
    MatrixExp(Box<MatrixExpression>),
}
