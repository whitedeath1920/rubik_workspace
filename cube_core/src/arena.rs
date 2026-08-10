use std::ops::{BitAnd, Shr, ShrAssign};

use rand::{RngExt, seq::SliceRandom};

use crate::{CubeError, error::Result};

pub const BIT_PACKING: [(u128, u128, usize, u128); 4] =
    [(3, 7, 8, 3), (1, 15, 12, 4), (0, 31, 6, 5), (0, 31, 24, 5)];
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
pub const LUT: [[u8; 3]; 1 << 15] = {
    let mut lut = [[0u8; 3]; 1 << 15];
    let mask = (1 << 5) - 1;
    let mut i = 0;
    while i < 1 << 15 {
        lut[i][0] = (i & mask) as u8;
        lut[i][1] = ((i >> 5) & mask) as u8;
        lut[i][2] = ((i >> 10) & mask) as u8;
        i += 1;
    }

    lut
};

pub trait PieceKind {
    const ORI: u128;
    const PERM: u128;
    const LEN: usize;
    const SHIFT: u32;
    const MOD: u128;
}

pub struct Corner;
impl PieceKind for Corner {
    const ORI: u128 = 3;
    const PERM: u128 = 7;
    const LEN: usize = 8;
    const SHIFT: u32 = 3;
    const MOD: u128 = 3;
}

pub struct Edge;
impl PieceKind for Edge {
    const ORI: u128 = 1;
    const PERM: u128 = 15;
    const LEN: usize = 12;
    const SHIFT: u32 = 4;
    const MOD: u128 = 2;
}

pub struct Center;
impl PieceKind for Center {
    const ORI: u128 = 0;
    const PERM: u128 = 31;
    const LEN: usize = 6;
    const SHIFT: u32 = 0;
    const MOD: u128 = 1; // to avoid any probable division by 0
}

pub struct Piece24;
impl PieceKind for Piece24 {
    const ORI: u128 = 0;
    const PERM: u128 = 31;
    const LEN: usize = 24;
    const SHIFT: u32 = 0;
    const MOD: u128 = 1; // to avoid any probable division by 0
}

#[repr(C, align(64))]
#[derive(Debug, Clone)]
pub struct CubeArena {
    data: Vec<u128>,

    /// Number of cubes in the arena.
    len: usize,
    /// (start, length), of the 24 cubies orbits
    orbit: [(u16, u16); 3],
    /// Number of packed arrays per cube
    pub stride: u16,
    /// number of orbits in cube
    len_orbits: u16,
    /// Dimensions of the cube.
    n: u8,
}

impl CubeArena {
    pub fn new_arena(n: u8, len: usize) -> Self {
        assert!(len > 0);
        let n_mod_2 = (n & 1) as u16;
        let tmp = (n as u16 - 2 - n_mod_2) >> 1; // Temporal value represents de number of pieces between the corner en the edge
        let stride = tmp.pow(2) + tmp + tmp * n_mod_2 + 1;
        let orbit = [
            (1, tmp.pow(2)),
            (1 + tmp.pow(2), tmp),
            ((1 + tmp.pow(2) + tmp) * n_mod_2, tmp * n_mod_2),
        ];
        let mut data = vec![984818244535754528103549039458486304; (len + 2) * stride as usize];
        for cube in data.chunks_exact_mut(stride as usize) {
            cube[0] = 247132686368
                | (((407901468851537952 << 40) | (172066848 << (100))) * (n as u128 & 1));
        }
        let len_orbits = stride + n_mod_2 * 2;
        Self {
            data,
            orbit,
            n,
            len,
            stride,
            len_orbits,
        }
    }
    #[inline(always)]
    pub fn get_cube(&self, index: usize) -> &[u128] {
        debug_assert!(index < self.len);
        let start = index * self.stride as usize;
        &self.data[start..start + self.stride as usize]
    }
    #[inline(always)]
    pub fn get_cube_mut(&mut self, index: usize) -> &mut [u128] {
        debug_assert!(index < self.len);
        let start = index * self.stride as usize;
        &mut self.data[start..start + self.stride as usize]
    }
    #[inline(always)]
    pub fn cube_from_slice(&mut self, index: usize, slice: &[u128]) {
        debug_assert!(index < self.len);
        debug_assert!(slice.len() == self.stride as usize);

        let start = index * self.stride as usize;
        self.data[start..start + slice.len()].copy_from_slice(slice);
    }
    #[inline]
    pub fn cube_to_vec(&self, index: usize) -> Vec<Vec<(u8, u8)>> {
        let cube = self.get_cube(index);
        let mut result = Vec::with_capacity(self.len_orbits as usize);

        result.push(unpack_u128::<3, 7, 8, 3>(get_corner(cube[0]) as u128));
        if self.n & 1 == 1 {
            result.push(unpack_u128::<2, 15, 12, 4>(get_edge(cube[0]) as u128));
            result.push(unpack_u128::<0, 31, 6, 5>(get_center(cube[0]) as u128));
        }
        for i in 1..self.stride as usize {
            result.push(unpack_u128::<0, 31, 24, 5>(cube[i]));
        }
        result
    }
    #[inline]
    pub fn cube_from_vec(&mut self, index: usize, cube: &[Vec<(u8, u8)>]) {
        debug_assert!(cube.len() == self.len_orbits as usize);
        debug_assert!(index < self.len);
        let n = self.n;

        let c = self.get_cube_mut(index);
        let mut offset = 1;
        let mut block = pack_u128::<8, 3>(&cube[0]);
        if n & 1 == 1 {
            block |= pack_u128::<12, 4>(&cube[1]) << 40;
            block |= pack_u128::<6, 5>(&cube[2]) << 100;
            offset = 3;
        }
        c[0] = block;

        for (orbit, vect) in c[1..].iter_mut().zip(cube[offset..].iter()) {
            *orbit = pack_u128::<24, 5>(vect);
        }
    }
    #[inline(always)]
    pub fn cube_ptr(&self, index: usize) -> *const u128 {
        debug_assert!(index < self.len + 2);
        unsafe { self.data.as_ptr().add(index * self.stride as usize) }
    }
    #[inline(always)]
    pub fn cube_mut_ptr(&mut self, index: usize) -> *mut u128 {
        debug_assert!(index < self.len + 2);
        unsafe { self.data.as_mut_ptr().add(index * self.stride as usize) }
    }
    #[inline]
    /// c = a + b
    pub fn add(&mut self, mut a: usize, mut b: usize, c: usize) {
        debug_assert!(a < self.len + 2 && b < self.len + 2 && c < self.len + 2);
        if a == c && b == c {
            self.clone(a, self.len);
            a = self.len;
            b = self.len;
        } else if a == c {
            self.clone(a, self.len);
            a = self.len;
        } else if b == c {
            self.clone(b, self.len);
            b = self.len;
        }
        unsafe {
            let c_a = self.cube_ptr(a);
            let c_b = self.cube_ptr(b);
            let c_c = self.cube_mut_ptr(c);

            let mut block = add_ori::<7,40,3,4>(get_corner(*c_a), get_corner(*c_b));
            if self.n & 1 == 1 {
                block |= add_ori::<15,4,60,2>(get_edge(*c_a), get_edge(*c_b)) << 40;
                block |= add_perm::<2>(get_center(*c_a), get_center(*c_b)) << 100;
            }
            *c_c = block;

            for a in 1..self.stride as usize {
                *c_c.add(a) = add_perm::<8>(*c_a.add(a), *c_b.add(a));
            }
        }
    }
    #[inline]
    /// c = a - b
    pub fn sub(&mut self, mut a: usize, mut b: usize, c: usize) {
        debug_assert!(a < self.len && b < self.len && c < self.len);
        if a == c && b == c {
            self.clone(a, self.len);
            a = self.len;
            b = self.len;
        } else if a == c {
            self.clone(a, self.len);
            a = self.len;
        } else if b == c {
            self.clone(b, self.len);
            b = self.len;
        }
        unsafe {
            let c_a = self.cube_ptr(a);
            let c_b = self.cube_ptr(b);
            let c_c = self.cube_mut_ptr(c);

            let mut block = sub_8(get_corner(*c_a), get_corner(*c_b));
            if self.n & 1 == 1 {
                block |= sub_12(get_edge(*c_a), get_edge(*c_b)) << 40;
                // block |= sub_6(get_center(*c_a), get_center(*c_b)) << 100;
            }
            *c_c = block;

            for a in 1..self.stride as usize {
                *c_c.add(a) = sub_24(*c_a.add(a), *c_b.add(a));
            }
        }
    }
    pub fn clone(&mut self, a: usize, b: usize) {
        debug_assert!(a < self.len + 2 && b < self.len + 2);
        if a == b {
            return;
        }
        let c_a = self.cube_ptr(a);
        let c_b = self.cube_mut_ptr(b);
        unsafe {
            std::ptr::copy_nonoverlapping(c_a, c_b, self.stride as usize);
        }
    }
    #[inline]
    pub fn neg(&mut self, a: usize, c: usize) {
        debug_assert!(a < self.len && c < self.len);
        unsafe {
            let c_a = self.cube_ptr(a);
            let c_c = self.cube_mut_ptr(c);

            let mut block = sub_8(247132686368, get_corner(*c_a));
            if self.n & 1 == 1 {
                block |= sub_12(407901468851537952, get_edge(*c_a)) << 40;
                // block |= sub_6(172066848, get_center(*c_a)) << 100;
            }
            *c_c = block;

            for a in 1..self.stride as usize {
                *c_c.add(a) = sub_24(984818244535754528103549039458486304, *c_a.add(a));
            }
        }
    }
    #[inline]
    /// c = n * a
    pub fn mul(&mut self, a: usize, n: isize, c: usize) {
        if n == 0 {
            self.identity(c);
        }
        self.clone(a, c);
        if n == 1 {
            return;
        }
        if n < 0 {
            self.neg(c, c);
        }

        let acc = self.len + 1;
        self.identity(acc);
        let mut n = n.abs();
        while n != 0 {
            if (n & 1) == 1 {
                self.add(acc, c, acc);
            }
            n >>= 1;
            if n != 0 {
                self.add(c, c, c);
            }
        }
        self.clone(acc, c);
    }
    #[inline(always)]
    pub fn identity(&mut self, index: usize) {
        debug_assert!(index < self.len + 2);
        let start = index * self.stride as usize;

        self.data[start] = 247132686368
            | ((407901468851537952 << 40) | (172066848_u128 << (100_u128)) * (self.n as u128 & 1));

        if self.n > 3 {
            self.data[start + 1..start + self.stride as usize]
                .fill(984818244535754528103549039458486304);
        }
    }
    pub fn print_cube(&self, index: usize) {
        debug_assert!(index < self.len);
        let cube = self.get_cube(index);

        println!("Cube {}:", index);

        println!(
            "Corner: {:?}",
            unpack_u128::<3, 7, 8, 3>(get_corner(cube[0]) as u128)
        );
        if self.n & 1 == 1 {
            println!(
                "Edge: {:?}",
                unpack_u128::<1, 15, 12, 4>(get_edge(cube[0]) as u128)
            );
            println!(
                "Center: {:?}",
                unpack_u128::<0, 31, 6, 5>(get_center(cube[0]) as u128)
            );
        }
        let name = ["Par Center", "Par Edge", "Par Corner"];
        for (&(start, len), n) in self.orbit.iter().zip(name.iter()) {
            for (i, &val) in &mut cube[start as usize..(start + len) as usize]
                .iter()
                .enumerate()
            {
                println!("{} {}: {:?}", n, i, unpack_u128::<0, 31, 24, 5>(val));
            }
        }
    }
    pub fn random_cube(&mut self, index: usize, rng: &mut impl rand::Rng) {
        debug_assert!(index < self.len + 2);
        let mut _6: [(u8, u8); 6] = std::array::from_fn(|i| (i as u8, 0));
        let mut _8: [(u8, u8); 8] = std::array::from_fn(|i| (i as u8, 0));
        let mut _12: [(u8, u8); 12] = std::array::from_fn(|i| (i as u8, 0));
        let mut _24: [(u8, u8); 24] = std::array::from_fn(|i| (i as u8, 0));

        let mut par = false;
        let stride = self.stride;
        let n = self.n;
        let cube = self.cube_mut_ptr(index);

        par ^= shuffle(&mut _8, rng);

        let mut ori_sum = 0;
        for i in 0..7 {
            let ori = rng.random_range(0..3);
            _8[i].1 = ori;
            ori_sum += ori;
        }

        _8[7].1 = (3 - (ori_sum % 3)) % 3;
        let mut block = pack_u128::<8, 3>(&_8);

        if n & 1 == 1 {
            par ^= shuffle(&mut _12, rng);
            ori_sum = 0;
            for i in 0..11 {
                let ori = rng.random::<bool>() as u8;
                _12[i].1 = ori;
                ori_sum ^= ori;
            }
            _12[11].1 = ori_sum;

            block |= pack_u128::<12, 4>(&_12) << 40;
            par ^= shuffle(&mut _6, rng);
            block |= pack_u128::<6, 5>(&_6) << 100;
        }
        unsafe {
            *cube = block;
            for i in 1..stride as usize {
                par ^= shuffle(&mut _24, rng);
                *cube.add(i) = pack_u128::<24, 5>(&_24);
            }

            if par {
                block = add_ori::<7,40,3,3>(get_corner(*cube), 247132686337);
                *cube &= !((1 << 40) - 1);
                *cube |= block;
            }
        }
    }
    pub fn cycle_decomposition_cube(&self, index: usize) -> Vec<Vec<Vec<usize>>> {
        debug_assert!(index < self.len + 2);
        let mut vect = Vec::with_capacity((self.stride as u8 + 2 * (self.n & 1)) as usize);
        let cube = self.get_cube(index);
        vect.push(cycle_decomp::<7, 8>(get_corner(cube[0]) as u128));

        if self.n & 1 == 1 {
            vect.push(cycle_decomp::<15, 12>(get_edge(cube[0]) as u128));
            vect.push(cycle_decomp::<31, 6>(get_center(cube[0]) as u128));
        }
        for i in 1..self.stride as usize {
            vect.push(cycle_decomp::<31, 24>(cube[i]));
        }
        vect
    }
    pub fn check_cube(&mut self, index: usize) -> Result<()> {
        debug_assert!(index < self.len + 2);
        let mut par = false;

        let cube = self.get_cube(index);
        par ^= parity::<7, 8>(get_corner(cube[0]) as u128);

        if self.n & 1 == 1 {
            par ^= parity::<15, 12>(get_edge(cube[0]) as u128);
            par ^= parity::<31, 6>(get_center(cube[0]) as u128);
        }
        for i in 1..self.stride as usize {
            par ^= parity::<31, 24>(cube[i]);
        }

        if par {
            return Err(CubeError::InvalidPermutation);
        }
        let block = self.data[index * self.stride as usize];
        orientation_check::<3, 7, 8, 3, 3>(get_corner(block) as u128)?;
        orientation_check::<1, 15, 12, 4, 2>(get_edge(block) as u128)?;
        Ok(())
    }
}
#[inline(always)]
fn unpack_u128<const ORI: u128, const PERM: u128, const LEN: usize, const SHIFT: u32>(
    value: u128,
) -> Vec<(u8, u8)> {
    let mut vect: Vec<(u8, u8)> = Vec::with_capacity(LEN);
    for i in 0..LEN {
        let block = (value >> (i * 5)) & 31;
        let p = (block & PERM) as u8;
        let o = ((block >> SHIFT) & ORI) as u8;

        vect.push((p, o));
    }

    vect
}
#[inline(always)]
pub fn pack_u128<const LEN: usize, const SHIFT: u32>(slice: &[(u8, u8)]) -> u128 {
    debug_assert!(slice.len() == LEN);
    let mut value = 0u128;
    for (i, &(p, o)) in slice.iter().enumerate() {
        let block = (p as u128) | ((o as u128) << SHIFT);
        value |= block << (i * 5);
    }
    value
}
#[inline(always)]
fn get_corner(a: u128) -> u64 {
    (a & ((1 << 40) - 1)) as u64
}
#[inline(always)]
fn get_edge(a: u128) -> u64 {
    ((a >> 40) & ((1 << 60) - 1)) as u64
}
#[inline(always)]
fn get_center(a: u128) -> u128 {
    a >> 100
}
#[inline(always)]
fn add_6(a: u64, mut b: u64) -> u128 {
    if 172066848 == b {
        return a as u128;
    }
    let mask = (1 << (15)) - 1;

    let mut p = 0;
    let mut produced = 0;
    for _ in 0..2 {
        let idx = (b & mask) as usize;
        let v = &LUT[idx];
        p |= PERM_SHIFT[produced][((a >> (v[0] * 5)) as usize) & 0b11111];
        p |= PERM_SHIFT[produced + 1][((a >> (v[1] * 5)) as usize) & 0b11111];
        p |= PERM_SHIFT[produced + 2][((a >> (v[2] * 5)) as usize) & 0b11111];
        produced += 3;
        b >>= 15;
    }
    p as u128
}

#[inline]
fn add_ori<const PERM: u64,const LEN: u8, const SHIFT: u64, const MOD: u64>(a: u64, b: u64) -> u128 {
    let mut out = 0;
    for shift in (0..LEN).step_by(5) {
        let b_block = (b >> shift) & 31;
        let b_perm = (b_block & PERM) * 5;
        let b_ori = b_block >> SHIFT;

        let a_block = (a >> b_perm) & 31;
        let a_perm = a_block & PERM;
        let a_ori = a_block >> SHIFT;

        let new_ori = ((b_ori + a_ori) % MOD) << SHIFT;
        out |= (a_perm | new_ori) << shift;
    }

    out as u128
}
#[inline]
fn add_perm<const LEN: u8>(a: u128, mut b: u128) -> u128 {
    let mut p = 0;
    let mut produced = 0;
    for _ in 0..LEN {
        let idx = (b & ((1 << (15)) - 1)) as usize;
        let v = &LUT[idx];
        let p1 = PERM_SHIFT[produced][((a >> (v[0] * 5)) as usize) & 0b11111];
        let p2 = PERM_SHIFT[produced + 1][((a >> (v[1] * 5)) as usize) & 0b11111];
        let p3 = PERM_SHIFT[produced + 2][((a >> (v[2] * 5)) as usize) & 0b11111];
        produced += 3;
        b >>= 15;
        p |= p1 | p2 | p3;
    }
    p 
}
#[inline(always)]
pub fn sub_6(mut a: u64, mut b: u64) -> u128 {
    if 172066848 == b {
        return a as u128;
    }
    let mask = (1 << (15)) - 1;

    let mut p = 0;

    for _ in 0..2 {
        let idx = (b & mask) as usize;
        let val = (a & mask) as usize;
        let v = &LUT[idx];
        let w = &LUT[val];
        p |= (w[0] as u128) << (v[0] as u128 * 5);
        p |= (w[1] as u128) << (v[1] as u128 * 5);
        p |= (w[2] as u128) << (v[2] as u128 * 5);
        b >>= 15;
        a >>= 15;
    }
    p as u128
}
#[inline(always)]
pub fn sub_8(a: u64, b: u64) -> u128 {
    if 247132686368 == b {
        return a as u128;
    }
    let mut out = 0;
    for shift in (0..40).step_by(5) {
        let b_block = (b >> shift) & 31;
        let b_perm = (b_block & 7) * 5;
        let b_ori = b_block >> 3;

        let a_block = (a >> shift) & 31;
        let a_perm = a_block & 7;
        let a_ori = a_block >> 3;

        let new_ori = ((a_ori + 3 - b_ori) % 3) << 3;

        out |= (a_perm | new_ori) << b_perm;
    }
    out as u128
}
#[inline(always)]
pub fn sub_12(a: u64, b: u64) -> u128 {
    if 407901468851537952 == b {
        return a as u128;
    }
    let mut out = 0;
    for shift in (0..60).step_by(5) {
        let b_block = (b >> shift) & 31;
        let b_perm = (b_block & 15) * 5;
        let b_ori = b_block >> 4;

        let a_block = (a >> shift) & 31;
        let a_perm = a_block & 15;
        let a_ori = a_block >> 4;

        let new_ori = ((a_ori + 2 - b_ori) & 1) << 4;
        out |= (a_perm | new_ori) << b_perm;
    }
    out as u128
}
#[inline(always)]
pub fn sub_24(mut a: u128, mut b: u128) -> u128 {
    if 984818244535754528103549039458486304 == b {
        return a;
    }
    let mask = (1 << (15)) - 1;

    let mut p = 0;

    for _ in 0..8 {
        let idx = (b & mask) as usize;
        let val = (a & mask) as usize;
        let v = &LUT[idx];
        let w = &LUT[val];
        p |= (w[0] as u128) << (v[0] as u128 * 5);
        p |= (w[1] as u128) << (v[1] as u128 * 5);
        p |= (w[2] as u128) << (v[2] as u128 * 5);
        b >>= 15;
        a >>= 15;
    }
    p
}
#[inline]
fn cycle_decomp<const PERM: u128, const LEN: usize>(value: u128) -> Vec<Vec<usize>> {
    let mut seen = [false; 24];
    let mut out = Vec::with_capacity(LEN >> 1);
    let mut cycle = Vec::with_capacity(LEN >> 1);
    for start in 0..LEN {
        if seen[start] {
            continue;
        }

        cycle.clear();
        let mut j = start;
        loop {
            if seen[j] {
                break;
            }

            seen[j] = true;
            cycle.push(j);

            j = ((value >> (j * 5)) & PERM) as usize;
        }
        if cycle.len() > 1 {
            out.push(std::mem::replace(&mut cycle, Vec::with_capacity(LEN >> 1)));
        }
    }
    out
}
#[inline]
pub fn parity<const PERM: u128, const LEN: usize>(value: u128) -> bool {
    let mut visited: u64 = 0;
    let mut cycles = 0;

    for i in 0..LEN {
        if (visited & (1 << i)) == 0 {
            cycles += 1;

            let mut j = i;

            loop {
                if (visited & (1 << j)) != 0 {
                    break;
                }

                visited |= 1 << j;
                j = ((value >> (j * 5)) & PERM) as usize;
            }
        }
    }

    ((LEN - cycles) & 1) != 0
}
#[inline]
fn orientation_check<
    const ORI: u128,
    const PERM: u128,
    const LEN: usize,
    const SHIFT: u32,
    const MOD: u128,
>(
    value: u128,
) -> Result<()> {
    if (0..LEN as u128)
        .map(|i| {
            let o = (value >> (i * 5 + SHIFT as u128)) & ORI;
            o
        })
        .sum::<u128>()
        % MOD
        != 0
    {
        return Err(CubeError::InvalidOrientation {
            got: unpack_u128::<ORI, PERM, LEN, SHIFT>(value),
            mod_: MOD as usize,
        });
    }
    Ok(())
}
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
pub fn shuffle(slice: &mut [(u8, u8)], rng: &mut impl rand::Rng) -> bool {
    let mut parity = false;
    for i in (1..slice.len()).rev() {
        let j = rng.random_range(0..=i);
        if i != j {
            parity = !parity;
            slice.swap(i, j);
        }
    }
    parity
}
