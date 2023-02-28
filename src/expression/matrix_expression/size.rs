use crate::{MatrixExpression, Size};

impl MatrixExpression {
    pub fn sizes(&self) -> Vec<Size> {
        match self {
            MatrixExpression::T(v) => {
                let sizes = v.sizes();
                vec![sizes[1], sizes[0]]
            }
            MatrixExpression::Inv(v) => v.sizes(),
            MatrixExpression::Det(_) => vec![Size::One, Size::One],
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use std::{collections::HashMap, ops::Add, collections::HashSet};

//     use opensrdk_linear_algebra::Matrix;

//     use crate::{Expression, MatrixExpression, Size, new_variable};

//     #[test]
//     fn it_works() {
//         let id = "x";
//         let a = HashSet::from([id; 1]);
//         let ea = new_variable((id).to_string());

//         // let len = 3usize;
//         // let a = Matrix::from(len, vec![1.0, 3.0, 4.0, 0.0, 1.0, 0.0, 0.0, 0.0, 3.0]).unwrap();
//         // let ea = Expression::from(a);

//         let size = ea.sizes();

//         let ea_t = ea.clone().t();
//         let size_t = ea_t.sizes();

//         assert_eq!(vec![size[1], size[0]], size_t);

//         let ea_inv = ea.clone().inv();
//         let size_inv = ea_inv.sizes();

//         assert_eq!(size, size_inv);

//         let ea_det = ea.clone().det();
//         let size_det = ea_det.sizes();

//         assert_eq!(vec![Size::One, Size::One], size_det);
//     }
// }
