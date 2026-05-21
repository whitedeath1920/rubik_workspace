use core::slice;
use std::alloc;
use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Mul, Neg, Sub, SubAssign};
use std::ptr::{self, NonNull};

use crate::CubeError;
use crate::error::Result;
use crate::state::Layout;

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
pub const NAME: [&str; 6] = [
    "Corner",
    "Edge",
    "Center",
    "Par Center",
    "Par Edge",
    "Edge Center",
];
pub const BIT_PACKING: [(u128, u128, usize, u128); 6] = [
    (3, 7, 8, 3),
    (1, 15, 12, 4),
    (0, 31, 6, 5),
    (0, 31, 24, 5),
    (0, 31, 24, 5),
    (0, 31, 24, 5),
];

#[repr(C)]
pub struct CubeState<'a> {
    data: NonNull<u128>,
    corner: u64,
    layout: &'a Layout,
}

impl<'a> CubeState<'a> {
    /// Data layout for the cube state.
    #[inline]
    fn data_layout(len: usize) -> alloc::Layout {
        let align = align_of::<u128>();
        let size = len.checked_mul(size_of::<u128>()).expect("size overflow");
        unsafe { alloc::Layout::from_size_align_unchecked(size, align) }
    }
    /// Creates a new cube state with the given layout.
    #[inline]
    pub fn with_layout(layout: &'a Layout) -> Self {
        let data_layout = Self::data_layout(layout.len);

        let data = unsafe {
            let ptr = alloc::alloc(data_layout) as *mut u128;
            if ptr.is_null() {
                alloc::handle_alloc_error(data_layout);
            }
            NonNull::new_unchecked(ptr)
        };
        let corner = 247132686368;
        for &(start, len, value) in &layout.orbit {
            unsafe {
                let dst = data.as_ptr().add(start);
                for i in 0..len {
                    dst.add(i).write(value);
                }
            }
        }
        Self {
            data,
            layout,
            corner,
        }
    }
    #[inline(always)]
    fn get(&self, index: usize) -> u128 {
        unsafe { self.data.as_ptr().add(index).read() }
    }
    #[inline(always)]
    fn set(&mut self, index: usize, value: u128) {
        unsafe { self.data.as_ptr().add(index).write(value) }
    }
    #[inline(always)]
    fn get_mut(&mut self, index: usize) -> *mut u128 {
        unsafe { self.data.as_ptr().add(index) }
    }
    /// Extracts a slice of the data starting at `start` with length `len`.
    #[inline(always)]
    fn extract(&self, start: usize, len: usize) -> &[u128] {
        unsafe { slice::from_raw_parts(self.data.as_ptr().add(start), len) }
    }
    #[inline(always)]
    fn extract_mut(&mut self, start: usize, len: usize) -> &mut [u128] {
        unsafe { slice::from_raw_parts_mut(self.data.as_ptr().add(start), len) }
    }
    pub fn to_vec(&self) -> Vec<Vec<(u8, u8)>> {
        let mut result: Vec<Vec<(u8, u8)>> = Vec::with_capacity(self.layout.len + 1);
        result.push(unpack_u128(self.corner as u128, BIT_PACKING[0]));
        for (i, &(start, len, _)) in self.layout.orbit.iter().enumerate() {
            if len > 0 {
                for value in self.extract(start, len) {
                    result.push(unpack_u128(*value, BIT_PACKING[i + 1]));
                }
            }
        }
        result
    }
    pub fn from_vec(vect: Vec<Vec<(u8, u8)>>, layout: &'a Layout) -> Self {
        assert_eq!(vect.len(), layout.len + 1);
        let data_layout = Self::data_layout(layout.len);

        let data = unsafe {
            let ptr = alloc::alloc(data_layout) as *mut u128;
            if ptr.is_null() {
                alloc::handle_alloc_error(data_layout);
            }
            NonNull::new_unchecked(ptr)
        };
        let corner = pack_u128(&vect[0], BIT_PACKING[0]) as u64;
        for (i, &(start, len, _)) in layout.orbit.iter().enumerate() {
            unsafe {
                let slice = &vect[start + 1..start + len + 1];
                let dst = data.as_ptr().add(start);
                for (j, v) in (0..len).zip(slice.iter()) {
                    dst.add(j).write(pack_u128(v, BIT_PACKING[i + 1]));
                }
            }
        }
        Self {
            data,
            layout,
            corner,
        }
    }
    pub fn to_slice(&self) -> &[u128] {
        let mut vect = Vec::with_capacity(self.layout.len + 1);
        vect.push(self.corner as u128);
        for a in self.extract(0, self.layout.len) {
            vect.push(*a);
        }

        let (ptr, len, _) = vect.into_raw_parts();
        unsafe { slice::from_raw_parts(ptr, len) }
    }
    pub fn from_slice(s: &[u128], layout: &'a Layout) -> Self {
        let data_layout = Self::data_layout(layout.len);
        let data = unsafe {
            let ptr = alloc::alloc(data_layout) as *mut u128;
            if ptr.is_null() {
                alloc::handle_alloc_error(data_layout);
            }
            NonNull::new_unchecked(ptr)
        };
        unsafe {
            ptr::copy_nonoverlapping(s[1..].as_ptr(), data.as_ptr(), layout.len);
        }
        Self {
            data,
            corner: s[0] as u64,
            layout,
        }
    }
    pub fn cycle_decomposition(&self) -> Vec<Vec<Vec<usize>>> {
        let mut vect = Vec::with_capacity(self.layout.len + 1);
        vect.push(cycle_decomp(self.corner as u128, BIT_PACKING[0]));

        if self.layout.n & 1 == 1 {
            vect.push(cycle_decomp(self.get(0), BIT_PACKING[1]));
            vect.push(cycle_decomp(self.get(1), BIT_PACKING[2]));
        }
        if self.layout.n > 3 {
            for &a in self.extract(self.layout.orbit[2].0, self.layout.len_24) {
                vect.push(cycle_decomp(a, BIT_PACKING[3]));
            }
        }

        vect
    }
    pub fn get_modulus(&self) -> usize {
        let mut m = 1;
        for a in self.cycle_decomposition().iter() {
            for b in a.iter() {
                m = mcm(m, b.len());
            }
        }
        if self.corner & 0b11000_11000_11000_11000_11000_11000_11000_11000 != 0 {
            m *= 3;
        }
        if self.layout.n & 1 == 1 {
            if self.get(0) & 0b10000_10000_10000_10000_10000_10000_10000_10000_10000_10000_10000_10000  != 0 {
                m *= 2;
            }
        }
        
        m
    }
    pub fn check(&self) -> Result<()> {
        orientation_check(self.corner as u128, BIT_PACKING[0], 3)?;
        
        if self.layout.n & 1 == 1 {
            orientation_check(self.get(0), BIT_PACKING[1], 2)?;
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
}
impl<'a> Display for CubeState<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{}\n\t0:\t{:?}",
            NAME[0],
            unpack_u128(self.corner as u128, BIT_PACKING[0])
        )?;
        for (i, &(start, len, _)) in self.layout.orbit.iter().enumerate() {
            if len > 0 {
                write!(f, "{}\n", NAME[i + 1])?;
                for (j, &value) in self.extract(start, len).iter().enumerate() {
                    write!(
                        f,
                        "\t{}:\t{:?}\n",
                        j,
                        unpack_u128(value, BIT_PACKING[i + 1])
                    )?;
                }
            }
        }
        Ok(())
    }
}
impl<'a> Drop for CubeState<'a> {
    fn drop(&mut self) {
        unsafe {
            let layout = Self::data_layout(self.layout.len);
            alloc::dealloc(self.data.as_ptr() as *mut u8, layout);
        }
    }
}
impl<'a> Debug for CubeState<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self)?;
        return Ok(());
        writeln!(f, "[")?;
        writeln!(f, "\t[{}]", self.corner)?;
        for i in 0..self.layout.len {
            unsafe {
                writeln!(f, "\t[{:?}],", *self.data.as_ptr().add(i))?;
            }
        }
        writeln!(f, "]")?;
        writeln!(f, "{}", self.layout)?;
        Ok(())
    }
}
impl<'a> PartialEq for CubeState<'a> {
    fn eq(&self, other: &Self) -> bool {
        if self.layout.n != other.layout.n {
            return false;
        }
        if self.corner != other.corner {
            return false;
        }
        let nbytes = self.layout.len * size_of::<u128>();
        unsafe {
            let a = slice::from_raw_parts(self.data.as_ptr() as *const u8, nbytes);
            let b = slice::from_raw_parts(other.data.as_ptr() as *const u8, nbytes);
            a == b
        }
    }
}
impl<'a> Eq for CubeState<'a> {}
unsafe impl<'a> Send for CubeState<'a> where &'a Layout: Send {}
unsafe impl<'a> Sync for CubeState<'a> where &'a Layout: Sync {}
impl<'a> Clone for CubeState<'a> {
    fn clone(&self) -> Self {
        let data_layout = Self::data_layout(self.layout.len);
        let data = unsafe {
            let ptr = alloc::alloc(data_layout) as *mut u128;
            if ptr.is_null() {
                alloc::handle_alloc_error(data_layout);
            }
            NonNull::new_unchecked(ptr)
        };
        unsafe {
            ptr::copy_nonoverlapping(self.data.as_ptr(), data.as_ptr(), self.layout.len);
        }

        Self {
            data,
            layout: self.layout,
            corner: self.corner,
        }
    }
}
impl<'a> AddAssign<&CubeState<'a>> for CubeState<'a> {
    #[inline]
    fn add_assign(&mut self, rhs: &CubeState<'a>) {
        let orbit = self.layout.orbit;

        // unsafe {
        //     add_8(&mut *self.get_mut(0), rhs.get(0));
        // }
        add_8(&mut self.corner, rhs.corner);

        if self.layout.n & 1 == 1 {
            unsafe {
                add_12(&mut *self.get_mut(0), rhs.get(0));
                add_6(&mut *self.get_mut(1), rhs.get(1));
            }
        }
        if self.layout.n > 3 {
            for (a, &b) in self
                .extract_mut(orbit[2].0, rhs.layout.len_24)
                .iter_mut()
                .zip(rhs.extract(orbit[2].0, rhs.layout.len_24))
            {
                add_24(a, b);
            }
        }
    }
}
impl<'a> Add<&CubeState<'a>> for CubeState<'a> {
    type Output = Self;
    #[inline]
    fn add(mut self, rhs: &CubeState<'a>) -> Self {
        self += rhs;
        self
    }
}
impl<'a> SubAssign<&CubeState<'a>> for CubeState<'a> {
    #[inline]
    fn sub_assign(&mut self, rhs: &CubeState<'a>) {
        let orbit = self.layout.orbit;

        // unsafe {
        //     sub_8(&mut *self.get_mut(0), rhs.get(0));
        // }
        sub_8(&mut self.corner, rhs.corner);
        if self.layout.n & 1 == 1 {
            unsafe {
                sub_12(&mut *self.get_mut(0), rhs.get(0));
                sub_6(&mut *self.get_mut(1), rhs.get(1));
            }
        }
        if self.layout.n > 3 {
            for (a, &b) in self
                .extract_mut(orbit[2].0, rhs.layout.len_24)
                .iter_mut()
                .zip(rhs.extract(orbit[2].0, rhs.layout.len_24))
            {
                sub_24(a, b);
            }
        }
    }
}
impl<'a> Sub<&CubeState<'a>> for CubeState<'a> {
    type Output = Self;
    #[inline]
    fn sub(mut self, rhs: &CubeState<'a>) -> Self {
        self -= rhs;
        self
    }
}
impl<'a> Neg for CubeState<'a> {
    type Output = Self;
    #[inline(always)]
    fn neg(mut self) -> Self {
        let orbit = self.layout.orbit;

        // unsafe {
        //     sub_8(&mut *self.get_mut(0), rhs.get(0));
        // }
        let mut corner = 247132686368;
        sub_8(&mut corner, self.corner);
        self.corner = corner;
        if self.layout.n & 1 == 1 {
            let mut edge = 407901468851537952;
            let mut center = 172066848;
            sub_12(&mut edge, self.get(0));
            sub_6(&mut center, self.get(1));
            self.set(0, edge);
            self.set(1, center);
        }
        if self.layout.n > 3 {
            for a in self.extract_mut(orbit[2].0, self.layout.len_24).iter_mut() {
                let mut orbit = 984818244535754528103549039458486304;
                sub_24(&mut orbit, *a);
                *a = orbit;
            }
        }
        self
    }
}
macro_rules! mul_impl {
    (signed: $($T:ty),+$(,)?) => {$(
        impl<'a> Mul<CubeState<'a>> for $T {
            type Output = CubeState<'a>;
            #[inline]
            fn mul(self, mut rhs: CubeState<'a>) -> Self::Output {
                if self == 0 {
                    return CubeState::with_layout(rhs.layout);
                } else if self < 0 {
                    rhs  = -rhs;
                }
                mul_by_u128(self.abs() as u128, rhs)
            }
        }
    )+};
    (unsigned: $($T:ty),+$(,)?) => {$(
        impl<'a> Mul<CubeState<'a>> for $T {
            type Output = CubeState<'a>;
            #[inline]
            fn mul(self, rhs: CubeState<'a>) -> Self::Output {
                if self == 0 {
                    return CubeState::with_layout(rhs.layout);
                }
                mul_by_u128(self as u128, rhs)
            }
        }
    )+};
}
mul_impl!(signed: i8, i16, i32, i64, i128, isize);
mul_impl!(unsigned: u8, u16, u32, u64, u128, usize);
#[inline(always)]
pub fn unpack_u128(
    value: u128,
    (ori, perm, len, shift): (u128, u128, usize, u128),
) -> Vec<(u8, u8)> {
    let mut vec: Vec<(u8, u8)> = Vec::with_capacity(len);
    for i in 0..len {
        let block = (value >> (i * 5)) & 31;
        let p = (block & perm) as u8;
        let o = ((block >> shift) & ori) as u8;

        vec.push((p, o));
    }
    vec
}
#[inline(always)]
pub fn pack_u128(vect: &[(u8, u8)], (_, _, _, shift): (u128, u128, usize, u128)) -> u128 {
    let mut value = 0u128;
    for (i, &(p, o)) in vect.iter().enumerate() {
        let block = (p as u128) | ((o as u128) << shift);
        value |= block << (i * 5);
    }
    value
}
#[inline(always)]
pub fn add_6(a: &mut u128, mut b: u128) {
    let mask = (1 << (15)) - 1;

    let mut p = 0;
    let mut produced = 0;
    for _ in 0..2 {
        let idx = (b & mask) as usize;
        let v = &LUT[idx];
        p |= PERM_SHIFT[produced][((*a >> (v[0] * 5)) as usize) & 0b11111];
        p |= PERM_SHIFT[produced + 1][((*a >> (v[1] * 5)) as usize) & 0b11111];
        p |= PERM_SHIFT[produced + 2][((*a >> (v[2] * 5)) as usize) & 0b11111];
        produced += 3;
        b >>= 15;
    }
    *a = p;
}
#[inline(always)]
pub fn add_8(a: &mut u64, b: u64) {
    let p = *a;
    let mut out = 0;
    for shift in (0..40).step_by(5) {
        let b_block = (b >> shift) & 31;
        let b_perm = (b_block & 7) * 5;
        let b_ori = b_block >> 3;

        let a_block = (p >> b_perm) & 31;
        let a_perm = a_block & 7;
        let a_ori = a_block >> 3;

        let new_ori = ((b_ori + a_ori) % 3) << 3;
        out |= (a_perm | new_ori) << shift;
    }
    *a = out;
}
#[inline(always)]
pub fn add_12(a: &mut u128, b: u128) {
    let p = *a;
    let mut out: u128 = 0;
    for shift in (0..60).step_by(5) {
        let b_block = (b >> shift) & 31;
        let b_perm = (b_block & 15) * 5;
        let b_ori = b_block >> 4;

        let a_block = (p >> b_perm) & 31;
        let a_perm = a_block & 15;
        let a_ori = a_block >> 4;

        let new_ori = ((b_ori + a_ori) & 1) << 4;
        out |= (a_perm | new_ori) << shift;
    }
    *a = out;
}
#[inline(always)]
pub fn add_24(a: &mut u128, mut b: u128) {
    let mask = (1 << (15)) - 1;

    let mut p = 0;
    let mut produced = 0;

    for _ in 0..8 {
        let idx = (b & mask) as usize;
        let v = &LUT[idx];
        p |= PERM_SHIFT[produced][((*a >> (v[0] * 5)) as usize) & 0b11111];
        p |= PERM_SHIFT[produced + 1][((*a >> (v[1] * 5)) as usize) & 0b11111];
        p |= PERM_SHIFT[produced + 2][((*a >> (v[2] * 5)) as usize) & 0b11111];
        produced += 3;
        b >>= 15;
    }
    *a = p;
}
#[inline(always)]
pub fn sub_6(a: &mut u128, mut b: u128) {
    let mask = (1 << (15)) - 1;

    let mut p = 0;

    for _ in 0..2 {
        let idx = (b & mask) as usize;
        let val = (*a & mask) as usize;
        let v = &LUT[idx];
        let w = &LUT[val];
        p |= (w[0] as u128) << (v[0] as u128 * 5);
        p |= (w[1] as u128) << (v[1] as u128 * 5);
        p |= (w[2] as u128) << (v[2] as u128 * 5);
        b >>= 15;
        *a >>= 15;
    }
    *a = p;
}
#[inline(always)]
pub fn sub_8(a: &mut u64, b: u64) {
    let p = *a;
    let mut out = 0;
    for shift in (0..40).step_by(5) {
        let b_block = (b >> shift) & 31;
        let b_perm = (b_block & 7) * 5;
        let b_ori = b_block >> 3;

        let a_block = (p >> shift) & 31;
        let a_perm = a_block & 7;
        let a_ori = a_block >> 3;

        let new_ori = ((a_ori + 3 - b_ori) % 3) << 3;

        out |= (a_perm | new_ori) << b_perm;
    }
    *a = out;
}
#[inline(always)]
pub fn sub_12(a: &mut u128, b: u128) {
    let p = *a;
    let mut out = 0;
    for shift in (0..60).step_by(5) {
        let b_block = (b >> shift) & 31;
        let b_perm = (b_block & 15) * 5;
        let b_ori = b_block >> 4;

        let a_block = (p >> shift) & 31;
        let a_perm = a_block & 15;
        let a_ori = a_block >> 4;

        let new_ori = ((a_ori + 2 - b_ori) & 1) << 4;
        out |= (a_perm | new_ori) << b_perm;
    }
    *a = out;
}
#[inline(always)]
pub fn sub_24(a: &mut u128, mut b: u128) {
    let mask = (1 << (15)) - 1;

    let mut p = 0;

    for _ in 0..8 {
        let idx = (b & mask) as usize;
        let val = (*a & mask) as usize;
        let v = &LUT[idx];
        let w = &LUT[val];
        p |= (w[0] as u128) << (v[0] as u128 * 5);
        p |= (w[1] as u128) << (v[1] as u128 * 5);
        p |= (w[2] as u128) << (v[2] as u128 * 5);
        b >>= 15;
        *a >>= 15;
    }
    *a = p;
}
#[inline(always)]
fn mul_by_u128<'a>(mut n: u128, mut base: CubeState<'a>) -> CubeState<'a> {
    let mut acc: CubeState<'a> = CubeState::with_layout(base.layout);
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
#[inline]
fn cycle_decomp(value: u128, (_, perm, len, _): (u128, u128, usize, u128)) -> Vec<Vec<usize>> {
    let mut seen = [false; 24];
    let mut vect = Vec::with_capacity(len);
    let mut p: Vec<usize> = Vec::with_capacity(len >> 1);
    for piece in 0..len {
        if seen[piece] {
            continue;
        }
        p.clear();
        let mut j = ((value >> (piece * 5)) & perm) as usize;
        while !seen[j] {
            seen[j] = true;
            j = ((value >> (j * 5)) & perm) as usize;
            p.push(j);
        }
        if p.len() > 1 {
            vect.push(std::mem::replace(&mut p, Vec::with_capacity(len >> 1)));
        }
    }
    vect
}
#[inline]
fn orientation_check(
    value: u128,
    (ori, perm, len, shift): (u128, u128, usize, u128),
    mod_: u128,
) -> Result<()> 
{
    if (0..len as u128).map(|i| {
        let o = (value >> (i * 5 + shift)) & ori;
        println!("{}\t{}",(i * 5 + shift),o);
        o
    }).sum::<u128>() % mod_ != 0 {
        return Err(CubeError::InvalidOrientation {
            got: unpack_u128(value, (ori, perm, len, shift)),
            mod_: mod_ as usize,
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