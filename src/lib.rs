#[cfg(test)]
extern crate blas_src;
#[cfg(test)]
extern crate lapack_src;
pub extern crate opensrdk_linear_algebra;
extern crate rayon;
extern crate thiserror;

pub mod dual;
pub mod float;

pub use dual::*;
pub use float::*;
