//! Permutative and orientational representation of an NxN Rubik's Cube.
//!
//! This module defines the low-level canonical state of the cube using packed permutations (`u128`) and packed orientations (`u32`)
//!
//! ## Design goals
//! - Zero allocation in hot paths.
//! - Deterministic behavior.
//! - Compact representation suitablo for hashing and group operations.
//!
//! ## Invariants
//! - Every permutation orbit encodes valid permutation of its pieces.
//! - Orientation sums satisfy the orbit modulus (`ORI_MOD`).
//! - Global permutation parity must be even.

use crate::{
    error::{CubeError, Result},
    state::{Bit, mcm},
};
/// Number of orbits Kinds
pub const NUM_KIND: usize = 6;
/// Number of pieces per orbit.
///
/// Index corresponds to the canonical kind ordering.
pub const NUM_PER_KIND: [usize; NUM_KIND] = [8, 12, 6, 24, 24, 24];
/// Number of unique pieces per kind.
pub const PERM_MOD: [usize; NUM_KIND] = [8, 12, 6, 6, 24, 6];
/// The modulus of the orientation of each kind of pieces.
pub const ORI_MOD: [usize; NUM_KIND] = [3, 2, 1, 1, 1, 1];
/// Number of pieces per unique piece kind
pub const NUM_PER_UNIQUE: [usize; NUM_KIND] = [1, 1, 4, 1, 1, 4];
/// String representation of each kind.
pub const KIND: [&str; NUM_KIND] = [
    "Corners",
    "Edges",
    "Centers",
    "Par Centers",
    "Par Edges",
    "Edge Centers",
];
/// Canonical identity permutation for each orbit.
///
/// This representation accounts for inditinguishable pieces
/// (e.g. center pieces of the same color).
///
/// Used to initialize solver states and compute identities.
pub const IDENTITY_ORD: [u128; NUM_KIND] = [
    247132686368,
    42535295865117307933329727397822564384,
    85070591730234615865843651858114119712,
    127820279166683303482936997031151927296,
    171126001705004986259790852755342592032,
    212890870896917919348780648889093980160,
];
/// Canonical identity permutation assuming all pieces are distinguishable
///
/// Unlike [`IDENTITY_ORD`], this treats every piece as unique.
/// Mainly useful for algebraic analisys.
pub const IDENTITY_PERM: [u128; NUM_KIND] = [
    247132686368,
    42535295865117307933329727397822564384,
    85070591730234615865843651858114119712,
    128590705839887678326869026826371565600,
    171126001705004986259790852755342592032,
    213661297570122294192712678684313618464,
];
/// Canonical permutation-orientation state of an NxN cube.
///
/// `CubeState` stores:
/// - one packed permutation per orbit
/// - two packked orientation bitfields (corners and edges)
///
/// ## Invariants
/// - Each permutation encodes a valid permutation of its orbit.
/// - Orientation sums satisfy orbit orientation modulus.
/// - Global permutation parity is even.
///
/// This type does not store the cube dimension explicitly;
/// it is assumed to be consisten with the layout used to construct it.
#[derive(Debug, Clone)]
pub struct CubeState {
    /// Packed orbits permutations.
    pub perm: Vec<u128>,

    /// Packed orientations (corners, edges).
    pub ori: [u32; 2],
}

impl CubeState {
    /// Create a new cube state with the given dimension.
    ///
    /// # Panics
    /// This function assumes `2 <= dimension <= 4292967296`.
    /// Use [`try_new`] for a checked constructor.
    pub fn new(dimension: usize) -> Self {
        let dim_mod_2 = dimension & 1;
        let tmp1 = (dimension - 2 - dim_mod_2) >> 1;
        let orbits = [1, dim_mod_2, dim_mod_2, tmp1.pow(2), tmp1, tmp1 * dim_mod_2];
        let perm = orbits
            .iter()
            .zip(IDENTITY_PERM)
            .map(|(&o, i)| vec![i; o])
            .flatten()
            .collect::<Vec<u128>>();

        Self {
            perm,
            ori: [0, 1 << 29],
        }
    }
    /// Creates a solved cube state, validating the dimension.
    ///
    /// # Errors
    /// Returns [`CubeError::InvalidDimension`] if the dimension is outside the supported range.
    pub fn try_new(dimension: usize) -> Result<Self> {
        if dimension < 2 || dimension >= 4292967296 {
            Err(CubeError::InvalidDimension { got: dimension })
        } else {
            Ok(Self::new(dimension))
        }
    }
    /// Converts the internal packed representation into a vector form.
    ///
    /// This representation is intended for debugging, serialization, and interoperability with external tools.
    ///
    /// The returned value is `(Permutations, orientations)`.
    #[inline]
    pub fn to_vec(&self) -> (Vec<Vec<u8>>, Vec<Vec<u8>>) {
        (
            self.perm.iter().map(|val| val.to_vec()).collect(),
            self.ori.iter().map(|val| val.to_vec()).collect(),
        )
    }
    /// Constructs a `CubeState` from its vector representation
    ///
    /// # Safety
    /// This function does not validate the input
    /// Use [`try_from_vec`] for a checked constructor.
    #[inline]
    pub fn from_vec(vect: (Vec<Vec<u8>>, Vec<Vec<u8>>)) -> Self {
        let ori: Vec<u32> = vect.1.iter().map(|v| u32::from_vec(v)).collect();
        Self {
            perm: vect.0.iter().map(|v| u128::from_vec(v)).collect(),
            ori: [ori[0], ori[1]],
        }
    }
    /// Constructs a `CubeState` from its vector representation, validating if the state is posible using canonical moves
    ///
    /// # Errors
    /// Returns an error if:
    /// - Orientation sums violate orbits modulus.
    /// - Permutation parity is invalid.
    /// - Verifies compatibility with the layout and dimension.
    #[inline]
    pub fn try_from_vec(vec: (Vec<Vec<u8>>, Vec<Vec<u8>>)) -> Result<Self> {
        let ori: Vec<u32> = vec.1.iter().map(|v| u32::from_vec(v)).collect();
        let state = Self {
            perm: vec.0.iter().map(|v| u128::from_vec(v)).collect(),
            ori: [ori[0], ori[1]],
        };
        state.check()?;
        Ok(state)
    }
    /// Returns the inverted position of the current state
    #[inline]
    pub fn invert(self) -> Self {
        -self
    }
    /// Returns the dimesion from the perm layout
    /// # NOTE
    /// assumes that the perm layout is actually valid
    #[inline]
    pub fn get_dimension(&self) -> usize {
        3
    }
    /// Returns a the identity sube (solved) from the current layout.
    #[inline]
    pub fn identity(&self) -> Self {
        Self {
            perm: self
                .perm
                .iter()
                .map(|a| IDENTITY_PERM[a.get_kind()])
                .collect(),
            ori: [0, 1 << 29],
        }
    }
    /// Returns the cycle decomposition for the cube.
    ///
    /// Kind[orbit[cycle1[c1,c2...], cycle2[b1,b2...]]]
    pub fn cycle_decomposition(&self) -> Vec<Vec<Vec<usize>>> {
        (0..self.perm.len())
            .map(|i| self.cycle_decomposition_of(i))
            .collect()
    }
    /// Computes the cycle decomposition of a single orbit
    fn cycle_decomposition_of(&self, index: usize) -> Vec<Vec<usize>> {
        let mut seen = [false; 24];
        let mut vect = Vec::with_capacity(self.perm.len());
        let value = self.perm[index];
        let mut perm = Vec::with_capacity(NUM_PER_KIND[value.get_kind()] >> 1);
        for piece in 0..NUM_PER_KIND[value.get_kind()] {
            if seen[piece] {
                continue;
            }
            perm.clear();
            
            let mut j = value.get(piece) as usize;
            while !seen[j] {
                seen[j] = true;
                j = value.get(j) as usize;
                perm.push(j);
            }
            if perm.len() > 1 {
                vect.push(std::mem::replace(
                    &mut perm,
                    Vec::with_capacity(NUM_PER_KIND[value.get_kind()] >> 1),
                ));
                // vect.push(std::mem::replace(&mut perm, Vec::with_capacity(NUM_PER_KIN[value.get_kind()] >> 1)));
            }
        }
        vect
    }

    /// Validates that the cube state is physically reachable.
    ///
    /// # Errors
    /// Returns an error if:
    /// - Orientation sums violate orbit modulus.
    /// - Permutation parity is invalid.
    ///
    /// # Notes
    /// This does not verify compatibility with a specific layout,
    /// only algebraic validity.
    pub fn check(&self) -> Result<()> {
        for i in 0..2 {
            if (0..NUM_PER_KIND[i]).map(|j| self.ori[i].get(j)).sum::<u8>() % (ORI_MOD[i] as u8)
                != 0
            {
                return Err(CubeError::InvalidOrientation {
                    got: self.ori[i].to_vec(),
                    mod_: ORI_MOD[i],
                });
            }
        }

        let mut parity = false;
        for a in self.cycle_decomposition() {
            let mut odd = false;
            for b in a {
                if b.len() & 1 == 0 {
                    odd = !odd;
                }
            }
            parity ^= odd;
        }
        if parity {
            return Err(CubeError::InvalidPermutation);
        }
        Ok(())
    }
    /// Computes the order of the cube state in the permutation group.
    ///
    /// This is the least common multiple of all cycle lengths, including orientation contributions.
    ///
    /// Useful for algebraic analysis and group-theory operations.
    pub fn get_modulus(&self) -> usize {
        let mut m = 1;
        for a in self.cycle_decomposition().iter() {
            for b in a.iter() {
                m = mcm(m, b.len());
            }
        }
        if self.ori[0] != 0 {
            m *= 3;
        }
        if self.ori[1] != 1 << 29 {
            m *= 2;
        }
        m
    }
}
#[inline]
pub fn _new_orbit(kind: usize) -> u128 {
    assert!(kind < NUM_KIND);
    let mut tmp = (kind as u128) << 125;
    for j in 0..PERM_MOD[kind] {
        for k in 0..NUM_PER_UNIQUE[kind] {
            tmp.set(j * NUM_PER_UNIQUE[kind] + k, j as u8);
        }
    }
    tmp
}
