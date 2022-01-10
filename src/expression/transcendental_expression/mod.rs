use crate::Expression;

pub mod differential;
pub mod evaluate;
pub mod functions;
pub mod into;
pub mod symbol;

pub use differential::*;
pub use evaluate::*;
pub use symbol::*;

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
