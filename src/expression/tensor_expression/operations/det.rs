use crate::TensorExpression;

impl TensorExpression {
    pub fn det(self) -> TensorExpression {
        if let TensorExpression::Constant(v) = self {
            return TensorExpression::Constant(todo!("{:#?}", v));
        }
        if let TensorExpression::Det(expression) = self {
            return *expression;
        }

        TensorExpression::Det(self.into())
    }
}
