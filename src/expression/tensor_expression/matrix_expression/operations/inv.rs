use crate::{MatrixExpression, TensorExpression};

impl MatrixExpression {
    pub fn inv(self) -> MatrixExpression {
        match self {
            MatrixExpression::Mat(v) => MatrixExpression::Inv(v.into()),
            MatrixExpression::Constant(v) => {
                MatrixExpression::Constant(v.getrf().unwrap().getri().unwrap())
            }
            MatrixExpression::Inv(v) => v,
            MatrixExpression::Det(v) => panic!(),
        }
    }
}

impl MatrixExpression {
    pub(crate) fn diff_inv(v: &TensorExpression, symbols: &[&str]) -> Vec<TensorExpression> {
        v.differential(symbols)
            .into_iter()
            .map(|d_v_d_symbol| {
                let v_inv: TensorExpression = v.clone().to_mat().inv().into();
                let d_v_inv_d_v = -v_inv.inner_prod(v_inv, &[[1, 0]]);

                d_v_inv_d_v.inner_prod(d_v_d_symbol, &[[0, 0], [1, 1]])
            })
            .collect()
    }
}
