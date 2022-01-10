use crate::{Expression, Symbol};
use opensrdk_linear_algebra::Matrix;

pub mod operations;
pub mod operators;
pub mod symbol;
pub mod value;

pub use symbol::*;
pub use value::*;

#[derive(Clone, Debug)]
pub enum MatrixExpression {
    Symbol(Symbol),
    Constant(Matrix),
    Add(Box<MatrixExpression>, Box<MatrixExpression>),
    Sub(Box<MatrixExpression>, Box<MatrixExpression>),
    Mul(Box<MatrixExpression>, Box<MatrixExpression>),
    MulScalar(Box<Expression>, Box<MatrixExpression>),
    Neg(Box<MatrixExpression>),
    Det(Box<MatrixExpression>),
    T(Box<MatrixExpression>),
    Inv(Box<MatrixExpression>),
    Solve(Box<MatrixExpression>, Box<MatrixExpression>),
    Pow(Box<MatrixExpression>, Box<Expression>),
    MatrixExp(Box<MatrixExpression>),
}
