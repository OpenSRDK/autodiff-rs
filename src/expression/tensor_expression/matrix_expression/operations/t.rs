use crate::{MatrixExpression, TensorExpression};

impl MatrixExpression {
    pub fn t(self) -> MatrixExpression {
        match self {
            MatrixExpression::Mat(_) => MatrixExpression::T(self.into()),
            MatrixExpression::Constant(v) => MatrixExpression::Constant(v.t()),
            MatrixExpression::T(v) => *v,
            MatrixExpression::Inv(_) => MatrixExpression::T(self.into()),
            MatrixExpression::Det(_) => panic!(),
        }
    }
}

impl MatrixExpression {
    pub(crate) fn diff_t(v: &MatrixExpression, symbols: &[&str]) -> Vec<TensorExpression> {
        let tensor = TensorExpression::KroneckerDeltas(vec![[0, 1]])
            .inner_prod(TensorExpression::Matrix(v.clone().into()), &[[0, 1]])
            .inner_prod(TensorExpression::KroneckerDeltas(vec![[0, 1]]), &[[0, 1]]);

        tensor.differential(symbols)
    }
}
