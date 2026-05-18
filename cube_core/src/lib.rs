//! `cube_core` is the bare minimum for the cube state manipulation
//! this includes generating a functional cube and implementing the basic operations
//! creating the moveset from some rules
//! and implementing a fast cache `Pool` for especific operations
#![feature(allocator_api)]
#![feature(generic_const_exprs)]
#![feature(new_range_api)]
mod alg;
mod error;
mod layout;
pub mod moves;
mod pool;
pub mod state;
pub mod n_state;
pub mod n2_state;
pub mod n3_state;

pub use error::CubeError;
pub use layout::Layout;
pub use pool::Pool;
