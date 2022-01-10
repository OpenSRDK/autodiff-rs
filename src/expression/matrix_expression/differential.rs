use crate::MatrixExpression;

impl MatrixExpression {
    pub fn differential(&self, symbols: &[&str]) -> Vec<MatrixExpression> {
        match self {
            MatrixExpression::Symbol(_) => todo!(),
            MatrixExpression::Constant(_) => todo!(),
            MatrixExpression::Add(_, _) => todo!(),
            MatrixExpression::Sub(_, _) => todo!(),
            MatrixExpression::Mul(_, _) => todo!(),
            MatrixExpression::MulScalar(_, _) => todo!(),
            MatrixExpression::Neg(_) => todo!(),
            MatrixExpression::Pow(_, _) => todo!(),
            MatrixExpression::T(_) => todo!(),
            MatrixExpression::Det(_) => todo!(),
            MatrixExpression::MatrixExp(_) => todo!(),
        }
    }
}
