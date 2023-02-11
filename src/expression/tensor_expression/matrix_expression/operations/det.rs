use crate::{Expression, MatrixExpression, TensorExpression};

impl MatrixExpression {
    pub fn det(self) -> Expression {
        match self {
            MatrixExpression::Mat(_) => Expression::DegeneratedTensor(
                TensorExpression::Matrix(MatrixExpression::Det(self.into()).into()).into(),
            ),
            MatrixExpression::Constant(v) => Expression::Constant(v.getrf().unwrap().0.trdet()),
            MatrixExpression::T(v) => v.det(),
            MatrixExpression::Inv(_) => Expression::DegeneratedTensor(
                TensorExpression::Matrix(MatrixExpression::Det(self.into()).into()).into(),
            ),
            MatrixExpression::Det(_) => panic!(),
        }
    }
}

impl MatrixExpression {
    pub(crate) fn diff_det(v: &MatrixExpression, symbols: &[&str]) -> Vec<TensorExpression> {
        v.differential(symbols)
            .into_iter()
            .map(|d_v_d_symbol| {
                let v_det = v.clone().det();
                let v_inv_t: TensorExpression = v.clone().inv().t().into();
                let d_v_det_d_v = v_det * v_inv_t;

                d_v_det_d_v.inner_prod(d_v_d_symbol, &[[0, 0], [1, 1]])
            })
            .collect()
    }
}
