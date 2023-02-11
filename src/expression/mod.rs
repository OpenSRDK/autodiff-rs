pub mod assign;
pub mod differential;
pub mod operators;
pub mod symbol;
pub mod tensor_expression;
pub mod tex_code;
pub mod transcendental_expression;

pub use assign::*;
pub use differential::*;
pub use symbol::*;
pub use tensor_expression::*;
pub use tex_code::*;
pub use transcendental_expression::*;

use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Expression {
    Symbol(String),
    Constant(f64),
    Add(Box<Expression>, Box<Expression>),
    Sub(Box<Expression>, Box<Expression>),
    Mul(Box<Expression>, Box<Expression>),
    Div(Box<Expression>, Box<Expression>),
    Neg(Box<Expression>),
    Pow(Box<Expression>, f64),
    Transcendental(Box<TranscendentalExpression>),
    DegeneratedTensor(Box<TensorExpression>),
    DiffResultTensor(Box<TensorExpression>),
}

impl From<f64> for Expression {
    fn from(v: f64) -> Self {
        Expression::Constant(v)
    }
}
