pub mod differential;
pub mod evaluate;
pub mod operators;
pub mod rust_code;
pub mod symbol;
pub mod tensor_expression;
pub mod transcendental_expression;

pub use differential::*;
pub use evaluate::*;
pub use rust_code::*;
pub use symbol::*;
pub use tensor_expression::*;
pub use transcendental_expression::*;

use num_rational::*;

#[derive(Clone, Debug, PartialEq)]
pub enum Expression {
    Symbol(String),
    Constant(f64),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Neg(Box<Expression>),
    Pow(Box<Expression>, Ratio<u32>),
    Transcendental(Box<TranscendentalExpression>),
    TensorElement(Box<TensorExpression>, Vec<usize>),
    _DiffResultTensor(Box<TensorExpression>),
}

impl From<f64> for Expression {
    fn from(v: f64) -> Self {
        Expression::Constant(v)
    }
}
