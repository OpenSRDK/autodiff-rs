pub mod continuous_distribution;
pub mod gaussian_process;
pub mod joint_array_distribution;
pub mod joint_distribution;
pub mod kernel;
pub mod multivatiate_normal;
pub mod rbf;
pub mod svgd;

pub use continuous_distribution::*;
pub use joint_array_distribution::*;
pub use joint_distribution::*;
pub use kernel::*;
pub use multivatiate_normal::*;
pub use rbf::*;
pub use svgd::*;
