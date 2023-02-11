use crate::{Expression, MatrixExpression, TensorExpression};

impl MatrixExpression {
    pub fn det(self) -> Expression {
        match self {
            MatrixExpression::Mat(v) => {
                Expression::DegeneratedTensor(MatrixExpression::Det(v).into())
            }
            MatrixExpression::Constant(v) => {
                Expression::Constant(v.to_mat().getrf().unwrap().0.trdet())
            }
            MatrixExpression::Det(v) => panic!(),
            _ => Expression::DegeneratedTensor(MatrixExpression::Det(self.into()).into()),
        }
    }
}

impl MatrixExpression {
    pub(crate) fn diff_det(v: &TensorExpression, symbols: &[&str]) -> Vec<TensorExpression> {
        v.differential(symbols)
            .into_iter()
            .map(|d_v_d_symbol| {
                let v_det: TensorExpression = v.clone().to_mat().det().into();
                let v_inv: TensorExpression = v.clone().to_mat().inv().into();
                let d_v_det_d_v = v_det.inner_prod(v_inv, &[[1, 1]]);

                d_v_det_d_v.inner_prod(d_v_d_symbol, &[[0, 0], [1, 1]])
            })
            .collect()
    }
}
