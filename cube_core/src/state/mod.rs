mod state;
mod ops;

pub use state::{CubeState, KIND, NUM_PER_KIND, NUM_PER_UNIQUE, IDENTITY_PERM, IDENTITY_ORD, ORI_MOD, PERM_MOD, NUM_KIND};
pub use ops::{Bit, mcm};