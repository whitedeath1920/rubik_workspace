use std::ops::{Add, AddAssign, Sub, SubAssign};

use crate::n_state::{CubeState, state::NUM_PER_KIND};
const ADD_MOD: [[u32; 5]; 4] = [
    [0, 1, 2, 0, 1],
    [0, 1, 0, 1, 0],
    [0, 0, 0, 0, 0],
    [0, 0, 0, 0, 0],
];
// impl Add<&CubeState> for CubeState {
//     type Output = Self;
//     #[inline]
//     fn add(mut self, rhs: &Self) -> Self::Output {
//         self.add_assign(rhs);
//         self
//     }
// }
// impl AddAssign<&CubeState> for CubeState {
//     #[inline(always)]
//     fn add_assign(&mut self, rhs: &Self) {
//         let mut idx = 0;

//         let p_a_p = self.perm.as_mut_ptr();
//         let p_b_p = rhs.perm.as_ptr();

//         let mut a_perm = unsafe { &mut *p_a_p.add(idx) };
//         let b_perm = unsafe { *p_b_p.add(idx) };
//         idx += 1;

//         from_perm_ori_to_slice(&mut a_perm, b_perm, &mut self.ori[0], rhs.ori[0], 0);

//         if self.perm.len() > 1 {
//             let mut a_perm = unsafe { &mut *p_a_p.add(idx) };
//             let b_perm = unsafe { *p_b_p.add(idx) };
//             idx += 1;
//             from_perm_ori_to_slice(&mut a_perm, b_perm, &mut self.ori[1], rhs.ori[1], 1);
//         }

//         for i in idx..self.perm.len() {
//             let mut a_perm = unsafe { &mut *p_a_p.add(i) };
//             let b_perm = unsafe { *p_b_p.add(i) };
//             from_perm_to_slice(&mut a_perm, b_perm);
//         }
//     }
// }
const PERM_SHIFT: [[u128; 24]; 24] = {
    let mut arr = [[0u128; 24]; 24];
    let mut i = 0;
    while i < 24 {
        let mut p = 0;
        while p < 24 {
            arr[i][p] = (p as u128) << (i * 5);
            p += 1;
        }
        i += 1;
    }
    arr
};
const ORI_SHIFT: [[[u32; 5]; 12]; 4] = {
    let mut arr = [[[0u32; 5]; 12]; 4];
    let mut kind = 0;
    while kind < 4 {
        let mut o = 0;
        while o < 12 {
            let mut oo = 0;
            while oo < 5 {
                arr[kind][o][oo] = ADD_MOD[kind][oo] << (o * 2);
                oo += 1;
            }
            o += 1;
        }
        kind += 1;
    }
    arr
};
macro_rules! perm_add {
    ($pos:expr, $val:expr) => {
        PERM_SHIFT[$pos][$val]
    };
}
macro_rules! perm_sub {
    ($pos:expr, $val: expr) => {
        PERM_SHIFT[$val][$pos]
    };
}
macro_rules! ori_add {
    ($arr:expr, $a:expr, $b:expr) => {
        $arr[($a + $b) as usize]
    };
}
macro_rules! ori_sub {
    ($arr:expr, $a:expr, $b:expr) => {{
        let idx = {
            if $a > $b {
                $a - $b
            } else {
                $b - $a
            }
        };
        $arr[idx as usize]
    }};
}
macro_rules! add_sub_impl {
    (base: $op:ident, $op_ori:ident, $perm_op:ident, $ori_op:ident, $Op:path, $OpAssign:path, $op_assign:ident) => {
        #[inline(always)]
        pub fn $op(a: &mut u128, mut b: u128) {
            let kind = a.get_kind();
            let count = NUM_PER_KIND[kind];
            let mask = (1 << (15)) - 1;
            let shift = 15;
            let full_chunks = count / 3;

            let p = *a;
            *a = (kind as u128) << 125;
            b.set_kind(0);

            let mut produced = 0;

            for _ in 0..full_chunks {
                let idx = (b & mask) as usize;
                let v = u128::LUT[idx];
                *a |= $perm_op!(produced, p.get(v[0] as usize) as usize);
                *a |= $perm_op!(produced + 1,p.get(v[1] as usize) as usize);
                *a |= $perm_op!(produced + 2,p.get(v[2] as usize) as usize);
                produced += 3;
                b >>= shift;
            }
            let rem = count - produced;
            if rem != 0 {
                let tail = (b & ((1 << (rem * 5)) - 1)) as u16;
                let mask = (1 << 5) - 1;
                if rem >= 1 {
                    *a |= $perm_op!(produced,p.get((tail & mask) as usize) as usize);
                }
                if rem == 2 {
                    *a |= $perm_op!(produced + 1,p.get(((tail >> 5) & mask) as usize) as usize);
                }
            }
        }

        #[inline(always)]
        pub fn $op_ori(
            a_perm: &mut u128,
            a_ori: &mut u32,
            mut b_perm: u128,
            mut b_ori: u32,
            kind: usize,
        ) {
            let count = NUM_PER_KIND[kind];
            let mask_p = (1 << 15) - 1;
            let mask_o = (1 << 6) - 1;
            let p = *a_perm;
            let o = *a_ori;
            *a_perm = (kind as u128) << 125;
            *a_ori = (kind as u32) << 29;
            b_perm.set_kind(0);
            b_ori.set_kind(0);
            let ori_shift = ORI_SHIFT[kind];
            let mut produced = 0;
            let full_chunks = count / 3;
            let shift_p = 15;
            let shift_o = 6;
            for _ in 0..full_chunks {
                let idx_p = (b_perm & mask_p) as usize;
                let idx_o = (b_ori & mask_o) as usize;

                let v_p = u128::LUT[idx_p];
                let v_o = u32::LUT[idx_o];
                *a_perm |= $perm_op!(produced, p.get(v_p[0] as usize) as usize);
                *a_perm |= $perm_op!(produced + 1, p.get(v_p[1] as usize) as usize);
                *a_perm |= $perm_op!(produced + 2, p.get(v_p[2] as usize) as usize);

                *a_ori |= $ori_op!(ori_shift[produced],o.get(v_p[0] as usize), v_o[0]);
                *a_ori |= $ori_op!(ori_shift[produced + 1],o.get(v_p[1] as usize),v_o[1]);
                *a_ori |= $ori_op!(ori_shift[produced + 2],o.get(v_p[2] as usize),v_o[2]);

                produced += 3;
                b_perm >>= shift_p;
                b_ori >>= shift_o;
            }
            let rem = count - produced;
            if rem != 0 {
                let tail_p = (b_perm & ((1 << (rem * 5)) - 1)) as u16;
                let tail_o = (b_ori & ((1 << (rem * 2)) - 1)) as u8;
                let mask_p = (1 << 5) - 1;
                let mask_o = (1 << 2) - 1;

                if rem >= 1 {
                    *a_perm |= $perm_op!(produced,p.get((tail_p & mask_p) as usize) as usize);
                    *a_ori |= $ori_op!(ori_shift[produced],
                        o.get((tail_p & mask_p) as usize), (tail_o & mask_o));
                }
                if rem == 2 {
                    *a_perm |= $perm_op!(produced + 1,p.get(((tail_p >> 5) & mask_p) as usize) as usize);
                    *a_ori |= $ori_op!(ori_shift[produced + 1],
                        o.get(((tail_p >> 5) & mask_p) as usize), (tail_o >> 2) & mask_o);
                }
            }
        }
        impl $Op for CubeState {
            type Output = Self;
            #[inline(always)]
            fn $op(mut self, rhs: &CubeState) -> Self::Output {
                self.$op_assign(rhs);
                self
            }
        }

        impl $OpAssign for CubeState {
            #[inline(always)]
            fn $op_assign(&mut self, rhs: &CubeState) {
                let mut idx = 0;

                let p_a_p = self.perm.as_mut_ptr();
                let p_b_p = rhs.perm.as_ptr();

                let mut a_perm = unsafe { &mut *p_a_p.add(idx) };
                let b_perm = unsafe { *p_b_p.add(idx) };
                idx += 1;

                $op_ori(&mut a_perm, &mut self.ori[0],b_perm, rhs.ori[0], 0);

                if self.perm.len() > 1 {
                    let mut a_perm = unsafe { &mut *p_a_p.add(idx) };
                    let b_perm = unsafe { *p_b_p.add(idx) };
                    idx += 1;
                    $op_ori(&mut a_perm, &mut self.ori[1], b_perm, rhs.ori[1], 1);
                }
                for i in idx..self.perm.len() {
                    let mut a_perm = unsafe { &mut *p_a_p.add(i) };
                    let b_perm = unsafe { *p_b_p.add(i) };
                    $op(&mut a_perm, b_perm);
                }
            }
        }
    };
    () => {
        add_sub_impl!(base: add, sum_ori, perm_add, ori_add, Add<&CubeState>, AddAssign<&CubeState>, add_assign);
        add_sub_impl!(base: sub, sub_ori, perm_sub, ori_sub, Sub<&CubeState>, SubAssign<&CubeState>, sub_assign);
    };
}
add_sub_impl!();

#[inline(always)]
pub fn from_perm_ori_to_slice(
    a_perm: &mut u128,
    mut b_perm: u128,
    a_ori: &mut u32,
    mut b_ori: u32,
    kind: usize,
) {
    let count = NUM_PER_KIND[kind];
    let mask_p = (1 << 15) - 1;
    let mask_o = (1 << 6) - 1;
    let p = *a_perm;
    let o = *a_ori;
    *a_perm = (kind as u128) << 125;
    *a_ori = (kind as u32) << 29;
    b_perm.set_kind(0);
    b_ori.set_kind(0);
    let ori_shift = ORI_SHIFT[kind];
    let mut produced = 0;
    let full_chunks = count / 3;
    let shift_p = 15;
    let shift_o = 6;
    for _ in 0..full_chunks {
        let idx_p = (b_perm & mask_p) as usize;
        let idx_o = (b_ori & mask_o) as usize;

        let v_p = u128::LUT[idx_p];
        let v_o = u32::LUT[idx_o];
        *a_perm |= PERM_SHIFT[produced][p.get(v_p[0] as usize) as usize];
        *a_perm |= PERM_SHIFT[produced + 1][p.get(v_p[1] as usize) as usize];
        *a_perm |= PERM_SHIFT[produced + 2][p.get(v_p[2] as usize) as usize];

        *a_ori |= ori_shift[produced][(o.get(v_p[0] as usize) + v_o[0]) as usize];
        *a_ori |= ori_shift[produced + 1][(o.get(v_p[1] as usize) + v_o[1]) as usize];
        *a_ori |= ori_shift[produced + 2][(o.get(v_p[2] as usize) + v_o[2]) as usize];
        produced += 3;
        b_perm >>= shift_p;
        b_ori >>= shift_o;
    }
    let rem = count - produced;
    if rem != 0 {
        let tail_p = (b_perm & ((1 << (rem * 5)) - 1)) as u16;
        let tail_o = (b_ori & ((1 << (rem * 2)) - 1)) as u8;
        let mask_p = (1 << 5) - 1;
        let mask_o = (1 << 2) - 1;

        if rem >= 1 {
            *a_perm |= PERM_SHIFT[produced][p.get((tail_p & mask_p) as usize) as usize];
            *a_ori |= ori_shift[produced]
                [(o.get((tail_p & mask_p) as usize) + (tail_o & mask_o)) as usize];
        }
        if rem == 2 {
            *a_perm |= PERM_SHIFT[produced + 1][p.get(((tail_p >> 5) & mask_p) as usize) as usize];
            *a_ori |= ori_shift[produced + 1]
                [(o.get(((tail_p >> 5) & mask_p) as usize) + (tail_o >> 2) & mask_o) as usize];
        }
    }
}
#[inline(always)]
pub fn from_perm_to_slice(a: &mut u128, mut b: u128) {
    let kind = a.get_kind();
    let count = NUM_PER_KIND[kind];
    let mask = (1 << (15)) - 1;
    let p = *a;
    *a = (kind as u128) << 125;
    b.set_kind(0);
    let mut produced = 0;
    let full_chunks = count / 3;
    let shift = 15;
    for _ in 0..full_chunks {
        let idx = (b & mask) as usize;
        let v = u128::LUT[idx];
        *a |= PERM_SHIFT[produced][p.get(v[0] as usize) as usize];
        *a |= PERM_SHIFT[produced + 1][p.get(v[1] as usize) as usize];
        *a |= PERM_SHIFT[produced + 2][p.get(v[2] as usize) as usize];
        produced += 3;
        b >>= shift;
    }
    let rem = count - produced;
    if rem != 0 {
        let tail = (b & ((1 << (rem * 5)) - 1)) as u16;
        let mask = (1 << 5) - 1;
        if rem >= 1 {
            *a |= PERM_SHIFT[produced][p.get((tail & mask) as usize) as usize];
        }
        if rem == 2 {
            *a |= PERM_SHIFT[produced + 1][p.get(((tail >> 5) & mask) as usize) as usize];
        }
    }
}
pub trait Bit<T> {
    const SHIFT: [T; 24];
    const MASK_SET: [T; 24];
    const MASK_GET: T;
    const LUT: [[u8; 3]; 1 << 15];
    const KIND_SHIFT: [T; 6];
    fn get(&self, i: usize) -> u8;
    fn set(&mut self, i: usize, v: u8);
    fn get_kind(&self) -> usize;
    fn set_kind(&mut self, kind: usize);
    fn to_slice(&self, out: &mut [u8; 25]);
    fn to_vec(&self) -> Vec<u8>;
    fn from_slice(vect: &[u8]) -> Self;
}
#[macro_export]
macro_rules! array_build {
    (mask_set: $T: ty, $bit: literal, $n: literal) => {{
        let mut arr = [0 as $T; 24];
        let mut i = 0;
        while i < $n {
            let shift = i * $bit;
            arr[i] = !(((1 << $bit) - 1) << shift);
            i += 1;
        }
        arr
    }};
    (shift: $T: ty, $bit: literal, $n:literal) => {{
        let mut arr = [0 as $T; 24];
        let mut i = 0;
        while i < $n {
            arr[i] = (i * $bit) as $T;
            i += 1;
        }
        arr
    }};
    (kind_shift: $T:ty, $payload: literal, $n: literal) => {{
        let mut arr = [0 as $T; $n];
        let mut i = 0;
        while i < $n {
            arr[i] = (i as $T) << $payload;
            i += 1;
        }
        arr
    }};
}
macro_rules! lut_build {
    ($T:ty, $bit:literal) => {{
        let mut lut = [[0u8; 3]; 1 << 15];
        let mask = (1 << $bit) - 1;
        let mut i = 0;
        while i < 1 << 15 {
            lut[i][0] = (i & mask) as u8;
            lut[i][1] = ((i >> $bit) & mask) as u8;
            lut[i][2] = ((i >> 2 * $bit) & mask) as u8;
            i += 1;
        }

        lut
    }};
}
macro_rules! bit_impl {
    ($T: ty, $bit: literal, $payload:literal, $n: literal) => {
        impl Bit<$T> for $T {
            const SHIFT: [$T; 24] = array_build!(shift: $T, $bit, $n);
            const MASK_SET: [$T; 24] = array_build!(mask_set: $T, $bit, $n);
            const MASK_GET: $T = (1 << $bit) - 1;
            const LUT: [[u8; 3]; 1 << 15] = lut_build!($T, $bit);
            const KIND_SHIFT: [$T; 6] = array_build!(kind_shift: $T ,$payload ,6);
            #[inline(always)]
            fn set(&mut self, i: usize, v: u8) {
                *self &= Self::MASK_SET[i];
                *self |= (v as $T) << Self::SHIFT[i];
            }
            #[inline(always)]
            fn get(&self, i: usize) -> u8 {
                ((self >> Self::SHIFT[i]) & Self::MASK_GET) as u8
            }
            #[inline(always)]
            fn get_kind(&self) -> usize {
                (self >> $payload) as usize
            }
            #[inline(always)]
            fn set_kind(&mut self, kind: usize) {
                *self = (*self & !(7 << $payload)) | Self::KIND_SHIFT[kind];
            }
            #[inline(always)]
            fn to_slice(&self, out: &mut [u8; 25]) {
                let kind = self.get_kind();
                out[24] = kind as u8;
                let count = crate::n_state::state::NUM_PER_KIND[kind];
                let mask = (1 << (3 * $bit)) - 1;

                let mut w = *self;
                w.set_kind(0);

                let mut produced = 0;
                let full_chunks = count / 3;
                let shift = 3 * $bit;
                for _ in 0..full_chunks {
                    let idx = (w & mask) as usize;
                    let v = Self::LUT[idx];
                    out[produced] = v[0];
                    out[produced + 1] = v[1];
                    out[produced + 2] = v[2];

                    produced += 3;
                    w >>= shift;
                }

                let rem = count - produced;
                if rem != 0 {
                    let tail = (w & ((1 << (rem * $bit)) - 1)) as u16;
                    let mask = (1 << $bit) - 1;
                    if rem >= 1 {
                        out[produced] = (tail & mask) as u8;
                    }
                    if rem == 2 {
                        out[produced + 1] = ((tail >> $bit) & mask) as u8;
                    }
                }
            }
            #[inline(always)]
            fn to_vec(&self) -> Vec<u8> {
                let kind = self.get_kind();
                let count = crate::n_state::state::NUM_PER_KIND[kind];
                let mask = (1 << (3 * $bit)) - 1;
                let mut out = vec![0; count + 1];
                out[count] = kind as u8;

                let mut w = *self;
                w.set_kind(0);

                let mut produced = 0;
                let full_chunks = count / 3;

                for _ in 0..full_chunks {
                    let idx = (w & mask) as usize;
                    let v = Self::LUT[idx];
                    out[produced] = v[0];
                    out[produced + 1] = v[1];
                    out[produced + 2] = v[2];

                    produced += 3;
                    w >>= 3 * $bit;
                }

                let rem = count - produced;
                if rem != 0 {
                    let tail = (w & ((1 << (rem * $bit)) - 1)) as u16;
                    let mask = (1 << $bit) - 1;
                    if rem >= 1 {
                        out[produced] = (tail & mask) as u8;
                    }
                    if rem == 2 {
                        out[produced + 1] = ((tail >> $bit) & mask) as u8;
                    }
                }
                out
            }
            #[inline(always)]
            fn from_slice(vect: &[u8]) -> $T {
                let kind = vect[vect.len() - 1] as usize;
                let mut a: $T = Self::KIND_SHIFT[kind];
                (0..crate::n_state::state::NUM_PER_KIND[kind])
                    .for_each(|i| a |= unsafe { (*vect.get_unchecked(i) as $T) << Self::SHIFT[i] });
                a
            }
        }
    };
}
bit_impl!(u128, 5, 125, 24);
bit_impl!(u32, 2, 29, 12);