use crate::MatrixExpression;

impl MatrixExpression {
    pub fn det(self) -> MatrixExpression {
        if let MatrixExpression::Constant(v) = self {
            return MatrixExpression::Constant(todo!("{:#?}", v));
        }
        if let MatrixExpression::Det(expression) = self {
            return *expression;
        }

        MatrixExpression::Det(self.into())
    }
}
