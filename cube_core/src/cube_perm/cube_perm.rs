use crate::{
    cube_perm::ops::{Bit, mcm},
    error::{CubeError, Result},
};
/// Number of pieces per kind.
pub const NUM_PER_KIND: [usize; 7] = [8, 12, 6, 24, 24, 24, 24];
/// Number of unique pieces per kind.
const PERM_MOD: [usize; 7] = [8, 12, 6, 6, 24, 6, 6];
/// Number of orientations per kind.
pub const ORI_MOD: [usize; 7] = [3, 2, 1, 1, 1, 1, 1];
/// Number of pieces per unique piece kind
const NUM_PER_UNIQUE: [usize; 7] = [1, 1, 1, 4, 1, 4, 4];
/// Names of kinds.
pub const KIND: [&str; 7] = [
    "Corners",
    "Odd Edges",
    "Odd Centers",
    "Corner Centers",
    "Par Edges",
    "Par Centers",
    "Edge Centers",
];
/// This takes in consideration that some pieces of the same color doesn't care of permutation
pub const IDENTITY_ORD: [u128; 7] = [
    247132686368,
    42535295865117307933329727397822564384,
    85070591730234615865843651858114119712,
    127820279166683303482936997031151927296,
    171126001705004986259790852755342592032,
    212890870896917919348780648889093980160,
    255426166762035227281702474818065006592,
];
/// This takes in consideration all permutations
pub const IDENTITY_PERM: [u128; 7] = [
    247132686368,
    42535295865117307933329727397822564384,
    85070591730234615865843651858114119712,
    128590705839887678326869026826371565600,
    171126001705004986259790852755342592032,
    213661297570122294192712678684313618464,
    256196593435239602125634504613284644896,
];

#[derive(Debug, Clone)]
pub struct CubePerm {
    pub perm: Vec<u128>,
    pub ori: [u32; 2],
}

impl CubePerm {
    /// Create a new cube state with the given layout.
    pub fn new(dimension: usize) -> Self {
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

        let mut perm = vec![0; subgroups.iter().sum()];

        let mut cont = 0;
        for (i, &g) in subgroups.iter().enumerate() {
            for _ in 0..g {
                perm[cont] = IDENTITY_PERM[i];
                cont += 1;
            }
        }

        Self {
            perm,
            ori: [0, 1 << 29],
        }
    }

    pub fn try_new(dimension: usize) -> Result<Self> {
        if dimension < 2 {
            Err(CubeError::InvalidDimension { got: dimension })
        } else if dimension >= 4292967296 {
            Err(CubeError::InvalidDimension { got: dimension })
        } else {
            Ok(Self::new(dimension))
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
        let ori: Vec<u32> = vect.1.into_iter().map(|v| u32::from_vec(&v)).collect();
        Self {
            perm: vect.0.into_iter().map(|v| u128::from_vec(&v)).collect(),
            ori: [ori[0], ori[1]],
        }
    }

    #[inline]
    pub fn inverse(self) -> Self {
        -self
    }

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

    pub fn get_perm_list(&self) -> Vec<Vec<Vec<usize>>> {
        self.perm.iter().map(|value| _get_perm(*value)).collect()
    }

    /// Checks if the state of the cube is valid
    /// Errors \
    /// When the cube state is not posible with the canonical moves
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
        for a in self.get_perm_list() {
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

    pub fn get_modulus(&self) -> usize {
        let mut m = 1;
        for a in self.get_perm_list().iter() {
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
pub fn _new_subgroup(kind: usize) -> u128 {
    let mut tmp = (kind as u128) << 125;
    for j in 0..PERM_MOD[kind] {
        for k in 0..NUM_PER_UNIQUE[kind] {
            tmp.set(j * NUM_PER_UNIQUE[kind] + k, j as u8);
        }
    }
    tmp
}

#[inline]
pub fn _get_perm(value: u128) -> Vec<Vec<usize>> {
    let mut seen = [false; 24];
    let mut vect = Vec::new();

    for piece in 0..NUM_PER_KIND[value.get_kind()] {
        if !seen[piece] {
            let mut perm = Vec::new();
            let mut j = value.get(piece) as usize;
            while !seen[j] {
                seen[j] = true;
                j = value.get(j) as usize;
                perm.push(j);
            }
            vect.push(perm);
        }
    }

    vect
}
