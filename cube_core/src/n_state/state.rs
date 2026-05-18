use std::{ptr, ops::Range};

use crate::{
    CubeError,
    error::Result,
    n_state::{array::Array, ops::Bit},
};

/// The modulus of the orientation of each kind of pieces.
// pub const ORI_MOD: [usize; NUM_KIND] = [3, 2, 1, 1, 1, 1];
/// Number of orbits Kinds
pub const NUM_KIND: usize = 6;
/// Mainly useful for algebraic analisys.
pub const IDENTITY_PERM: [u128; NUM_KIND] = [
    247132686368,
    42535295865117307933329727397822564384,
    85070591730234615865843651858114119712,
    128590705839887678326869026826371565600,
    171126001705004986259790852755342592032,
    213661297570122294192712678684313618464,
];
/// Number of pieces per orbit.
///
/// Index corresponds to the canonical kind ordering.
pub const NUM_PER_KIND: [usize; NUM_KIND] = [8, 12, 6, 24, 24, 24];

#[derive(Debug, Clone)]
pub struct CubeState {
    pub perm: Array,

    pub ori: [u32; 2],
}

impl CubeState {
    #[inline(always)]
    fn len(dimension: usize) -> usize {
        (dimension.pow(2) + 5 * (dimension & 1) - 2 * dimension + 4) / 4
    }
    /// Create a new cube state with given dimension
    ///
    /// # Panics
    /// This function assumes `2 <= dimension <= 4292967296`
    /// use [`new`] for a checked constructor.
    #[inline]
    pub fn unchecked_new(dimension: usize) -> Self {
        let dim_mod_2 = dimension & 1;
        let tmp1 = (dimension - 2 - dim_mod_2) >> 1;
        let orbits = [1,dim_mod_2, dim_mod_2, tmp1.pow(2), tmp1, tmp1 * dim_mod_2];
        let len = (dimension.pow(2) + 5 * (dimension & 1) - 2 * dimension + 4) / 4;      
        
        let mut perm = Array::with_capacity(len);
        
        let mut idx = 0;
        orbits.iter().zip(IDENTITY_PERM).for_each(|(&len, value)| {
            for index in idx..idx + len {
                perm.write(index, value);
            }
            idx += len;
        });
        Self {
            perm,
            ori: [0, 1 << 29],
        }
    }
    /// Create a new cube state with given dimension
    ///
    /// # Errors
    /// Returns [`CubeError::InvalidDimension`] if the dimension is outside the supported range.
    #[inline]
    pub fn new(dimension: usize) -> Result<Self> {
        if dimension < 2 || dimension >= 4292967296 {
            Err(CubeError::InvalidDimension { got: dimension })
        } else {
            Ok(Self::unchecked_new(dimension))
        }
    }

    #[inline]
    pub fn to_vec(&self) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
        (
            self.perm.iter().map(|val| val.to_vec()).collect(),
            self.ori.iter().map(|val| val.to_vec()).collect(),
        )
    }

    #[inline]
    pub fn from_vec(vect: (Vec<Vec<u8>>, Vec<Vec<u8>>)) -> Self {
        let (perm_vec, ori_vec) = vect;

        let perm = Array::from_slice(
            &mut perm_vec
                .iter()
                .map(|val| u128::from_slice(&val))
                .collect::<Vec<u128>>(),
        );

        let ori: Vec<u32> = ori_vec.iter().map(|val| u32::from_slice(&val)).collect();
        Self {
            perm,
            ori: [ori[0], ori[1]],
        }
    }
}

impl PartialEq for CubeState {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        self.perm.eq(&other.perm) && self.ori == other.ori
    }
}
