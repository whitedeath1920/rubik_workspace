use core::fmt;

use crate::cube_moves::Move;

pub type Result<T> = core::result::Result<T, CubeError>;

/// Error del cubo
#[derive(Debug)]
pub enum CubeError {
    // Errores de entrada
    InvalidDimension {got: usize},
    
    // Errores de estado
    InvalidOrientation {got: Vec<u8>, mod_: usize},
    InvalidPermutation,
    
    // Errores de movimientos
    InvalidMoveM {got: Move, expected: Vec<Move> },
    InvalidMoveS {got: String, expected: Vec<String> },
}

impl fmt::Display for CubeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CubeError::InvalidDimension { got } => write!(f, "Invalid dimension: expected 2 <= dimension <= 4292967296, got {}", got),
            CubeError::InvalidOrientation { got, mod_ } => write!(f, "Inval
                id orientation: got {:?}, mod: {}",got,mod_),
            CubeError::InvalidPermutation => write!(f, "Invalid permutation: Violated parity"),
            CubeError::InvalidMoveM { got, expected } => write!(f, "Invalid move: expected {:?}, got {:?}",expected, got),
            CubeError::InvalidMoveS { got, expected } => write!(f, "Invalid move: expected {:?}, got {:?}",expected, got),
        }
    }
}

impl std::error::Error for CubeError {}