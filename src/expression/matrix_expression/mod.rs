use crate::Expression;
use opensrdk_linear_algebra::Matrix;

pub mod evaluate;
pub mod operations;
pub mod operators;
pub mod symbol;

pub use evaluate::*;
pub use symbol::*;

#[derive(Clone, Debug)]
pub enum MatrixExpression {
    Symbol(String),
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
