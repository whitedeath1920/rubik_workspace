//! `cube_core` is the bare minimum for the cube state manipulation
//! this includes generating a functional cube and implementing the basic operations
//! creating the moveset from some rules
//! and implementing a fast cache `Pool` for especific operations
// #![warn(warnings)]
pub mod arena;
pub mod cube_moves;
pub mod cube_vect;
pub mod error;

pub use arena::CubeArena;
pub use cube_vect::{CubeVect, Point3, get_dim_from_len};
pub use error::CubeError;
