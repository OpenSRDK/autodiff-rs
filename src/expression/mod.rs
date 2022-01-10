pub mod as_scalar;
pub mod differential;
pub mod evaluate;
pub mod functions;
pub mod matrix_expression;
pub mod operators;
pub mod symbol;

pub use as_scalar::*;
pub use differential::*;
pub use evaluate::*;
pub use functions::*;
pub use matrix_expression::*;
pub use symbol::*;

#[derive(Clone, Debug)]
pub enum Expression {
    Symbol(String),
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

impl From<f64> for Expression {
    fn from(v: f64) -> Self {
        Expression::Constant(v)
    }
}
