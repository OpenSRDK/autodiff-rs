use crate::Expression;

pub mod assign;
pub mod differential;
pub mod functions;
pub mod into;
pub mod rust_code;
pub mod symbol;
pub mod tex_code;

pub use assign::*;
pub use differential::*;
pub use rust_code::*;
pub use symbol::*;
pub use tex_code::*;

#[derive(Clone, Debug, PartialEq)]
pub enum TranscendentalExpression {
    Abs(Box<Expression>),
    Pow(Box<Expression>, Box<Expression>),
    Exp(Box<Expression>),
    Log(Box<Expression>, Box<Expression>),
    Ln(Box<Expression>),
    Sin(Box<Expression>),
    Cos(Box<Expression>),
    Tan(Box<Expression>),
}
