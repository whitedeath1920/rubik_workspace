use std::{fmt::Display, ops::{Add, AddAssign}};

const NUM_PER_KIND: [usize; 6] = [8, 12, 6, 24, 24, 24];
pub const IDENTITY_PERM: [u128; 6] = [
    247132686368,
    42535295865117307933329727397822564384,
    85070591730234615865843651858114119712,
    128590705839887678326869026826371565600,
    171126001705004986259790852755342592032,
    213661297570122294192712678684313618464,
];
macro_rules! lut {
    ($name:ident, $bit:literal) => {
        const $name: [[u8; 3]; 1 << 15] = {
            let mut lut = [[0u8; 3]; 1 << 15];
            let mask = (1 << $bit) - 1;
            let mut i = 0;
            while i < 1 << 15 {
                lut[i][0] = (i & mask) as u8;
                i += 1;
            }
            lut
        };
    };
}
lut!(LUT_U32, 2);
lut!(LUT_U128, 5);

macro_rules! mask_set {
    ($name:ident, $T:ty, $bit:literal, $n:literal) => {
        const $name: [$T; $n] = {
            let mut arr = [0 as $T; $n];
            let mut i = 0;
            while i < $n {
                arr[i] = !(((1 << $bit)- 1) << (i * $bit));
                i += 1;
            }
            arr
        };
    };
}
mask_set!(MASK_SET_U32, u32, 2, 12);
mask_set!(MASK_SET_U128, u128, 5, 24);

#[derive(Debug, Copy, Clone)]
pub struct Perm<const KIND: usize>(pub u128);
impl<const KIND: usize> Perm<KIND> {
    fn get(&self, i: usize) -> u8 {
        ((self.0 >> (i * 5)) & ((1 << 5) - 1)) as u8
    }
    pub fn to_vec(&self) -> Vec<u8> {
        (0..NUM_PER_KIND[KIND]).map(|i| self.get(i)).collect()
    }
    pub fn from_slice(vect: &[u8]) -> Self {
        debug_assert!(vect.len() == NUM_PER_KIND[KIND]);
        let p = vect
            .iter()
            .enumerate()
            .fold(0, |p, (i, &v)| p | (v as u128) << (5 * i));
        Perm(p)
    }
}
impl<const KIND: usize> Default for Perm<KIND> {
    #[inline(always)]
    fn default() -> Self {
        unsafe { Self(*IDENTITY_PERM.get_unchecked(KIND)) }
    }
}
impl<const KIND: usize> Display for Perm<KIND> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f,"[")?;
        for p in self.to_vec() {
            write!(f,"{}, ", p)?;
        }
        write!(f,"]")?;
        Ok(())
    }
}
impl<const KIND: usize> Add<&Perm<KIND>> for Perm<KIND> {
    type Output = Self;
    fn add(mut self, rhs: &Self) -> Self::Output {
        self.add_assign(rhs);
        self
    }
}
impl<const KIND: usize> AddAssign<&Perm<KIND>> for Perm<KIND> {
    fn add_assign(&mut self, _rhs: &Self) {
        todo!()
    }
}
#[derive(Debug, Copy, Clone)]
pub struct Ori<const KIND: usize>(pub u128,pub u32);
impl<const KIND: usize> Ori<KIND> {
    fn get(&self, i: usize) -> (u8, u8) {
        (
            ((self.0 >> (i * 5)) & ((1 << 5) - 1)) as u8,
            ((self.1 >> (i * 2)) & ((1 << 2) - 1)) as u8 
        )
    }
    pub fn to_vec(&self) -> Vec<(u8, u8)> {
        (0..NUM_PER_KIND[KIND]).map(|i| self.get(i)).collect()
    }
    pub fn from_slice(vect: &[(u8, u8)]) -> Self {
        debug_assert!(vect.len() == NUM_PER_KIND[KIND]);
        let (p, o) = vect
            .iter()
            .enumerate()
            .fold((0, 0), |(p, o), (i, &(p_, o_))| {
                (p | (p_ as u128) << (5 * i), o | (o_ as u32) << (3 * i))
            });
        Ori(p, o)
    }
}
impl<const KIND: usize> Default for Ori<KIND> {
    #[inline(always)]
    fn default() -> Self {
        unsafe { Self(*IDENTITY_PERM.get_unchecked(KIND), 0) }
    }
}
impl<const KIND: usize> Display for Ori<KIND> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f,"[")?;
        for (p, o) in self.to_vec() {
            write!(f,"({}, {}), ", p, o)?;
        }
        write!(f,"]")?;
        Ok(())
    }
}
impl<const KIND: usize> Add<&Ori<KIND>> for Ori<KIND> {
    type Output = Self;
    fn add(mut self, rhs: &Self) -> Self::Output {
        self.add_assign(rhs);
        self
    }
}
impl<const KIND: usize> AddAssign<&Ori<KIND>> for Ori<KIND> {
    fn add_assign(&mut self, _rhs: &Ori<KIND>) {
        todo!()
    }
}