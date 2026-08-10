use core::fmt;

use crate::{
    Point3,
    cube_moves::{Axis, Move},
};

/// Shorthand for `core::result::Result<T, CubeError>`. Every fallible
/// function in this crate returns this type.
pub type Result<T> = core::result::Result<T, CubeError>;

/// Error type for the cube library.
///
/// Variants are grouped by category:
///
/// - **Input errors** — invalid arguments passed to a constructor.
/// - **State errors** — a cube configuration violates a group-theoretic
///   invariant (wrong orientation sum, bad parity, etc.).
/// - **Move errors** — an unrecognised or disallowed move was attempted.
/// - **Algorithm errors** — a move sequence does not form a valid
///   conjugate / commutator.
/// - **Vector errors** — the internal `CubeVect` representation is
///   inconsistent (duplicate positions, out-of-bounds coordinates,
///   wrong cubie count).
#[derive(Debug)]
pub enum CubeError {
    // input errors
    /// The cube dimension `got` is not supported (must be >= 2).
    InvalidDimension {
        /// The dimension that was requested.
        got: usize,
    },

    // state errors
    /// A piece-orbit has an invalid orientation sum modulo `mod_`.
    /// `got` is the unpacked (perm, ori) content of the offending orbit.
    InvalidOrientation {
        /// The unpacked (perm, ori) pairs of the orbit that failed.
        got: Vec<(u8, u8)>,
        /// The modulus that the orientation sum should equal 0 under.
        mod_: usize,
    },

    /// A piece-orbit has the wrong permutation parity, either globally
    /// or relative to another orbit as required by the first law of
    /// cubology (Bonzio-Loi-Peruzzi, 2017).
    InvalidPermutation {
        /// The unpacked (perm, ori) pairs of the orbit that failed.
        got: Vec<(u8, u8)>,
    },

    // move errors
    /// A `Move` value was not found in the move table.
    InvalidMoveM {
        /// The move that was attempted.
        got: Move,
        /// The list of valid moves that were expected.
        expected: Vec<Move>,
    },

    /// A string-encoded move name was not recognised.
    InvalidMoveS {
        /// The string that was parsed.
        got: String,
        /// The list of valid move-name strings.
        expected: Vec<String>,
    },

    // algorithm errors
    /// A conjugate operation received a move that is not a valid
    /// cube-rotation axis (expected one of X, Y, Z).
    InvalidMoveConjugate {
        /// The move that was passed where a rotation axis was expected.
        got: Move,
    },

    // vector errors
    /// A cubie in the `CubeVect` data array has a coordinate whose
    /// absolute value exceeds `half` (the maximum distance from the
    /// cube centre).
    InvalidVectorDimension {
        /// Index of the offending piece in the flat data array.
        i: usize,
        /// The 3D position that violates the bound.
        p: Point3,
        /// The maximum allowed absolute value for any coordinate.
        half: i8,
    },

    /// Two cubies in the `CubeVect` data array occupy the same 3D
    /// position.  This is a hard invariant: after any move every piece
    /// must have a unique coordinate triple.
    DuplicateVector {
        /// Index of the later piece that collided.
        i: usize,
        /// The 3D position that is duplicated.
        p: Point3,
        /// The spatial hash of `p` (for diagnostic purposes).
        hash: usize,
        /// Index of the earlier piece that already held this position.
        other: usize,
        /// The 3D position of the earlier piece (should equal `p`).
        other_p: Point3,
    },

    /// The `CubeVect` data array has the wrong number of cubies.
    InvalidLength {
        /// The actual number of cubies in the data array.
        got: usize,
        /// The expected number of cubies for an NxNxN cube.
        expected: usize,
    },
}

impl fmt::Display for CubeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CubeError::InvalidDimension { got } => {
                write!(f, "Invalid dimension: expected 2 <= dimension, got {}", got)
            }
            CubeError::InvalidOrientation { got, mod_ } => {
                write!(f, "Invalid orientation: got {:?}, modulus: {}", got, mod_)
            }
            CubeError::InvalidPermutation { got } => {
                write!(f, "Invalid permutation: violated parity, got {:?}", got)
            }
            CubeError::InvalidMoveM { got, expected } => {
                write!(f, "Invalid move: expected {:?}, got {:?}", expected, got)
            }
            CubeError::InvalidMoveS { got, expected } => {
                write!(f, "Invalid move: expected {:?}, got {:?}", expected, got)
            }
            CubeError::InvalidMoveConjugate { got } => write!(
                f,
                "Invalid move: expected {:?}, got {:?}",
                [Axis::X, Axis::Y, Axis::Z],
                got
            ),
            CubeError::InvalidVectorDimension { i, p, half } => write!(
                f,
                "Piece {} at {}: coordinate out of bounds (max {})",
                i, p, half
            ),
            CubeError::DuplicateVector {
                i,
                p,
                hash,
                other,
                other_p,
            } => write!(
                f,
                "Duplicate position at piece {}: {} (hash {})\n  already held by piece {}: {}",
                i, p, hash, other, other_p
            ),
            CubeError::InvalidLength { got, expected } => {
                write!(f, "CubeVect: expected {} cubies, got {}", expected, got)
            }
        }
    }
}

/// `CubeError` implements the standard library `Error` trait so it can
/// be used with `anyhow`, `eyre`, and other error-handling ecosystems.
impl std::error::Error for CubeError {}
