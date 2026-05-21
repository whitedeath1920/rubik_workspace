//! `cube_core` is the bare minimum for the cube state manipulation
//! this includes generating a functional cube and implementing the basic operations
//! creating the moveset from some rules
//! and implementing a fast cache `Pool` for especific operations
// mod alg;
mod error;
// pub mod moves;
pub mod state;

pub use error::CubeError;
