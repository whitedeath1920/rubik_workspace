use crate::error::{CubeError, Result};

/// Describes the geometry (`Layout`) of the `Cube`. \
/// Contains important indexing values pre-calculated
/// for the ease of use of the `CubeState` struct
#[derive(Debug, Clone, Copy)]
pub struct Layout {
    dimension: usize,
    par: bool,
    subgroups: [usize; 7],
    subgroups_offset: [usize; 7],
    groups_len: usize,
}

impl Layout {
    /// Constructs a new `Layout` with given dimension, knowing it won't fail
    pub fn new(dimension: usize) -> Self {
        let par = dimension % 2 == 0;

        let dim_mod_2 = dimension % 2;
        let tmp1 = (dimension - 2 - dim_mod_2) / 2;
        let subgroups = [
            1,
            dim_mod_2,
            dim_mod_2,
            tmp1,
            tmp1,
            tmp1 * dim_mod_2,
            ((dimension - 2).pow(2) - dim_mod_2) / 4 - tmp1 * (dim_mod_2 + 1),
        ];

        let mut subgroups_offset = [0; 7];
        let mut groups_len = 1;
        for i in 1..7 {
            subgroups_offset[i] += subgroups[i - 1];
            groups_len += subgroups[i];
        }

        Layout {
            dimension,
            par,
            subgroups,
            subgroups_offset,
            groups_len,
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
    pub fn subgroups(&self) -> [usize; 7] {
        self.subgroups
    }

    /// Returns the subgroup for the given kind
    #[inline]
    pub fn subgroup(&self, kind: usize) -> usize {
        self.subgroups[kind]
    }

    /// Returns de offset of the subgroup
    #[inline]
    pub fn subgroup_offset(&self, kind: usize) -> usize {
        self.subgroups_offset[kind]
    }

    /// Returns the offset of the subgroups
    #[inline]
    pub fn subgroups_offset(&self) -> [usize; 7] {
        self.subgroups_offset
    }

    /// Returns the total groups length
    #[inline]
    pub fn groups_len(&self) -> usize {
        self.groups_len
    }
}
