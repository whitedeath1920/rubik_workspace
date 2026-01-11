use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};

use crate::{
    CubePerm,
    cube_perm::cube_perm::{IDENTITY_ORD, NUM_PER_KIND, ORI_MOD},
};

macro_rules! oper_impl {
    ($Op:path, $iOp:ident, $OpAssign:path, $iOpAssign:ident, $lhs:ty, $rhs:ty, $ori:ident, $perm:ident) => {
        impl $Op for $lhs {
            type Output = Self;

            #[inline]
            fn $iOp(mut self, rhs: $rhs) -> Self::Output {
                self.$iOpAssign(rhs);
                self
            }
        }
        impl $OpAssign for $lhs {
            #[inline]
            fn $iOpAssign(&mut self, rhs: $rhs) {
                let mut cont = 1;

                $ori(&mut self.perm[0], rhs.perm[0], &mut self.ori[0], rhs.ori[0]);

                if self.perm.len() > 1 && self.perm[1].get_kind() == 1 {
                    $ori(&mut self.perm[1], rhs.perm[1], &mut self.ori[1], rhs.ori[1]);
                    cont = 2;
                }
                (cont..self.perm.len()).for_each(|i| $perm(&mut self.perm[i], rhs.perm[i]));
            }
        }
    };
}
oper_impl!(
    Sub<&CubePerm>,
    sub,
    SubAssign<&CubePerm>,
    sub_assign,
    CubePerm,
    &CubePerm,
    sub_ori_mut,
    sub_perm_mut
);
oper_impl!(
    Add<&CubePerm>,
    add,
    AddAssign<&CubePerm>,
    add_assign,
    CubePerm,
    &CubePerm,
    add_ori_mut,
    add_perm_mut
);
macro_rules! neg_impl {
    ($($T:ty),*) => {$(
       impl Neg for $T {
           type Output = CubePerm;
           #[inline]
           fn neg(self) -> Self::Output {
               self.identity() - &self
           }
       }
    )*};
}
neg_impl!(CubePerm, &CubePerm);

macro_rules! mul_impl {
    (signed: $($T:ty),+$(,)?) => {$(
        impl Mul<CubePerm> for $T {
            type Output = CubePerm;
            #[inline]
            fn mul(self, mut rhs: CubePerm) -> Self::Output {
                if self == 0 {
                    return rhs.identity();
                } else if self < 0 {
                    rhs  = -rhs;
                }
                mul_by_u128(self.abs() as u128, rhs)
            }
        }
    )+};
    (unsigned: $($T:ty),+$(,)?) => {$(
        impl Mul<CubePerm> for $T {
            type Output = CubePerm;
            #[inline]
            fn mul(self, rhs: CubePerm) -> Self::Output {
                if self == 0 {
                    return rhs.identity();
                }
                mul_by_u128(self as u128, rhs)
            }
        }
    )+};
}
#[inline]
fn mul_by_u128(mut n: u128, mut base: CubePerm) -> CubePerm {
    let mut acc = base.identity();
    while n != 0 {
        if (n & 1) == 1 {
            acc += &base;
        }
        n >>= 1;
        if n != 0 {
            base += &base.clone();
        }
    }
    acc
}
mul_impl!(signed: i8, i16, i32, i64, i128, isize);
mul_impl!(unsigned: u8, u16, u32, u64, u128, usize);

#[inline]
fn add_ori_mut(a_perm: &mut u128, b_perm: u128, a_ori: &mut u32, b_ori: u32) {
    let kind = a_perm.get_kind();

    let tmp_perm = *a_perm;
    let tmp_ori = *a_ori;
    (0..NUM_PER_KIND[kind]).for_each(|i| {
        let idx = b_perm.get(i) as usize;
        a_perm.set(i, tmp_perm.get(idx));
        a_ori.set(i, (tmp_ori.get(idx) + b_ori.get(i)) % ORI_MOD[kind] as u8);
    });
}
#[inline]
fn add_perm_mut(a_perm: &mut u128, b_perm: u128) {
    let kind = a_perm.get_kind();

    let tmp_perm = *a_perm;
    (0..NUM_PER_KIND[kind]).for_each(|i| {
        a_perm.set(i, tmp_perm.get(b_perm.get(i) as usize));
    });
}
#[inline]
fn sub_ori_mut(a_perm: &mut u128, b_perm: u128, a_ori: &mut u32, b_ori: u32) {
    let kind = a_perm.get_kind();
    let mod_ = ORI_MOD[kind] as u8;

    let tmp_perm = *a_perm;
    let tmp_ori = *a_ori;
    (0..NUM_PER_KIND[kind]).for_each(|i| {
        let idx = b_perm.get(i) as usize;
        a_perm.set(idx, tmp_perm.get(i));
        a_ori.set(idx, (tmp_ori.get(i) + mod_ - b_ori.get(i)) % mod_);
    });
}
#[inline]
fn sub_perm_mut(a_perm: &mut u128, b_perm: u128) {
    let kind = a_perm.get_kind();

    let tmp_perm = *a_perm;
    (0..NUM_PER_KIND[kind]).for_each(|i| {
        let idx = b_perm.get(i) as usize;
        a_perm.set(idx, tmp_perm.get(i));
    });
}
// Define las operaciones atómicas y traits utiles para el cubo
pub trait Bit {
    fn set(&mut self, i: usize, v: u8);
    fn get(&self, i: usize) -> u8;
    fn get_kind(&self) -> usize;
    fn set_kind(&mut self, kind: usize);
    fn to_vec(&self) -> Vec<u8>;
    fn from_vec(vect: &[u8]) -> Self;
}

macro_rules! bit_impl {
    ($T:ty, $bit:literal, $payload:literal) => {
        impl Bit for $T {
            #[inline]
            fn set(&mut self, i: usize, v: u8) {
                let shift = i * $bit;
                *self &= !(((1 << $bit) - 1) << shift);
                *self |= (v as $T) << shift;
            }
            #[inline]
            fn get(&self, i: usize) -> u8 {
                ((self >> (i * $bit)) & ((1 << $bit) - 1)) as u8
            }
            #[inline]
            fn get_kind(&self) -> usize {
                (self >> $payload) as usize
            }
            #[inline]
            fn set_kind(&mut self, kind: usize) {
                *self = (*self & !(7 << $payload)) | (kind as $T) << $payload;
            }
            #[inline]
            fn to_vec(&self) -> Vec<u8> {
                let mut vect: Vec<u8> = (0..crate::cube_perm::cube_perm::NUM_PER_KIND[self.get_kind()])
                    .map(|i| self.get(i))
                    .collect();
                vect.push(self.get_kind() as u8);
                vect
            }
            #[inline]
            fn from_vec(vect: &[u8]) -> $T {
                let kind = vect[vect.len() - 1] as usize;
                let mut a=  <$T>::default();
                a.set_kind(kind);
                (0..crate::cube_perm::cube_perm::NUM_PER_KIND[a.get_kind()])
                    .for_each(|i| a.set(i, vect[i]));
                a
            }
        }
    };
}
// Type, number of bytes, payload
bit_impl!(u128, 5, 125);
bit_impl!(u32, 2, 29);

impl PartialEq for CubePerm {
    #[inline]
    fn eq(&self, other: &Self) -> bool {
        if self.perm.len() != other.perm.len() {
            return false;
        }
        if self.ori != other.ori {
            return false;
        }
        for (a, b) in self.perm.iter().zip(&other.perm) {
            if !eq_subgroup(a, b) {
                return false;
            }
        }
        true
    }
}

#[inline]
fn eq_subgroup(a: &u128, b: &u128) -> bool {
    let kind = a.get_kind();
    for i in 0..NUM_PER_KIND[a.get_kind()] {
        if IDENTITY_ORD[kind].get(a.get(i) as usize) != IDENTITY_ORD[kind].get(b.get(i) as usize) {
            return false;
        }
    }
    true
}

impl Eq for CubePerm {}

#[inline]
pub fn gcd(mut u: usize, mut v: usize) -> usize {
    if u == 0 {
        return v;
    }
    if v == 0 {
        return u;
    }

    let shift = (u | v).trailing_zeros();
    u >>= shift;
    v >>= shift;
    u >>= u.trailing_zeros();

    loop {
        v >>= v.trailing_zeros();

        if u > v {
            let temp = u;
            u = v;
            v = temp;
        }

        v -= u; // here v >= u

        if v == 0 {
            break;
        }
    }

    u << shift
}

#[inline]
pub fn mcm(u: usize, v: usize) -> usize {
    (u * v) / gcd(u, v)
}

#[inline]
pub fn normalized_lcm(u: usize, v: usize) -> usize {
    (u * v) / gcd(u, v).pow(2)
}
