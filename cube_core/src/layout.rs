use crate::{error::{CubeError, Result}, state::NUM_KIND};

/// Describes the geometry (`Layout`) of the `Cube`. \
/// Contains important indexing values pre-calculated
/// for the ease of use of the `CubeState` struct
#[derive(Debug, Clone, Copy)]
pub struct Layout {
    dimension: usize,
    par: bool,
    orbits: [usize; NUM_KIND],
    orbits_offset: [usize; NUM_KIND],
    orbits_len: usize,
}

impl Layout {
    /// Constructs a new `Layout` with given dimension, knowing it won't fail
    pub fn new(dimension: usize) -> Self {
        let dim_mod_2 = dimension & 1;
        let par = dim_mod_2 == 0;
        

        let tmp1 = (dimension - 2 - dim_mod_2) / 2;
        let orbits = [
            1,
            dim_mod_2,
            dim_mod_2,
            tmp1.pow(2),
            tmp1,
            tmp1 * dim_mod_2,
        ];

        let mut orbits_offset = [0; NUM_KIND];
        let mut orbits_len = 1;
        for i in 1..NUM_KIND {
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
    pub fn orbits(&self) -> [usize; NUM_KIND] {
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
    pub fn orbits_offset(&self) -> [usize; NUM_KIND] {
        self.orbits_offset
    }

    /// Returns the total orbits length
    #[inline]
    pub fn orbits_len(&self) -> usize {
        self.orbits_len
    }
}
