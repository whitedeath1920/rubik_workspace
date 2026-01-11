use crate::error::{CubeError, Result};

/// Describes the geometry (`Layout`) of the `Cube`. \
/// Contains important indexing values pre-calculated
/// for the ease of use of the `CubeState` struct
#[derive(Debug, Clone, Copy)]
pub struct Layout {
    dimension: usize,
    par: bool,
    orbits: [usize; 7],
    orbits_offset: [usize; 7],
    orbits_len: usize,
}

impl Layout {
    /// Constructs a new `Layout` with given dimension, knowing it won't fail
    pub fn new(dimension: usize) -> Self {
        let par = dimension % 2 == 0;

        let dim_mod_2 = dimension % 2;
        let tmp1 = (dimension - 2 - dim_mod_2) / 2;
        let orbits = [
            1,
            dim_mod_2,
            dim_mod_2,
            tmp1,
            tmp1,
            tmp1 * dim_mod_2,
            ((dimension - 2).pow(2) - dim_mod_2) / 4 - tmp1 * (dim_mod_2 + 1),
        ];

        let mut orbits_offset = [0; 7];
        let mut orbits_len = 1;
        for i in 1..7 {
            orbits_offset[i] += orbits[i - 1];
            orbits_len += orbits[i];
        }

        Layout {
            dimension,
            par,
            orbits,
            orbits_offset,
            orbits_len,
        }
    }

    /// Creates a new `Layout` with given dimension.
    /// # Errors
    /// If dimension is less than 2.
    pub fn try_new(dimension: usize) -> Result<Self> {
        if dimension < 2 {
            Err(CubeError::InvalidDimension { got: dimension })
        } else if dimension >= 4294967296 {
            Err(CubeError::InvalidDimension { got: dimension })
        } else {
            Ok(Self::new(dimension))
        }
    }

    // Getters
    /// Returns the dimension of the cube.
    #[inline]
    pub fn dimension(&self) -> usize {
        self.dimension
    }

    /// Returns true if the number of layers in the cube is par
    #[inline]
    pub fn par(&self) -> bool {
        self.par
    }

    /// Returns the total number of pieces in the cube
    #[inline]
    pub fn orbits(&self) -> [usize; 7] {
        self.orbits
    }

    /// Returns the orbit for the given kind
    #[inline]
    pub fn orbit(&self, kind: usize) -> usize {
        self.orbits[kind]
    }

    /// Returns de offset of the orbit
    #[inline]
    pub fn orbit_offset(&self, kind: usize) -> usize {
        self.orbits_offset[kind]
    }

    /// Returns the offset of the orbits
    #[inline]
    pub fn orbits_offset(&self) -> [usize; 7] {
        self.orbits_offset
    }

    /// Returns the total orbits length
    #[inline]
    pub fn orbits_len(&self) -> usize {
        self.orbits_len
    }
}
