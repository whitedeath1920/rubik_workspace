//! `cube_core` is the bare minimum for the cube state manipulation
//! this includes generating a functional cube and implementing the basic operations
//! creating the moveset from some rules
//! and implementing a fast cache `Pool` for especific operations

pub mod state;
mod error;
mod pool;
mod layout;
pub mod moves;

pub use layout::Layout;
pub use error::CubeError;
pub use pool::Pool;
