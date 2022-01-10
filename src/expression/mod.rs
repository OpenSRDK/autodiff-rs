use crate::Symbol;

pub mod as_scalar;
pub mod differential;
pub mod functions;
pub mod matrix_expression;
pub mod operators;
pub mod symbol;
pub mod value;

pub use as_scalar::*;
pub use differential::*;
pub use functions::*;
pub use matrix_expression::*;
pub use symbol::*;
pub use value::*;

#[derive(Clone, Debug)]
pub enum Expression {
    Symbol(Symbol),
    Constant(f64),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Neg(Box<Expression>),
    Abs(Box<Expression>),
    Pow(Box<Expression>, Box<Expression>),
    Exp(Box<Expression>),
    Log(Box<Expression>, Box<Expression>),
    Ln(Box<Expression>),
    Sin(Box<Expression>),
    Cos(Box<Expression>),
    Tan(Box<Expression>),
    MatrixScalar(Box<MatrixExpression>),
}
