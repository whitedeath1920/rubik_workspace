use core::fmt;

// use crate::moves::{Move, Axis};

pub type Result<T> = core::result::Result<T, CubeError>;

/// Error del cubo
#[derive(Debug)]
pub enum CubeError {
    // Errores de entrada
    InvalidDimension {got: usize},
    
    // Errores de estado
    InvalidOrientation {got: Vec<(u8, u8)>, mod_: usize},
    InvalidPermutation,
    
    // Errores de movimientos
    // InvalidMoveM {got: Move, expected: Vec<Move> },
    InvalidMoveS {got: String, expected: Vec<String> },
    
    // Errores de algoritmos
    // InvalidMoveConjugate {got: Move},
}

impl fmt::Display for CubeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CubeError::InvalidDimension { got } => write!(f, "Invalid dimension: expected 2 <= dimension <= 4292967296, got {}", got),
            CubeError::InvalidOrientation { got, mod_ } => write!(f, "Inval
                id orientation: got {:?}, mod: {}",got,mod_),
            CubeError::InvalidPermutation => write!(f, "Invalid permutation: Violated parity"),
            // CubeError::InvalidMoveM { got, expected } => write!(f, "Invalid move: expected {:?}, got {:?}",expected, got),
            CubeError::InvalidMoveS { got, expected } => write!(f, "Invalid move: expected {:?}, got {:?}",expected, got),
            // CubeError::InvalidMoveConjugate { got } => write!(f, "Invalid move: expected {:?}, got  {:?}",[Axis::X,Axis::Y,Axis::Z],got),
        }
    }
}

impl std::error::Error for CubeError {}