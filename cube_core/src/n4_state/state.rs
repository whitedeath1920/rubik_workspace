use core::slice;
use std::fmt::{Debug, Display};
use std::ops::{Add, AddAssign, Sub, SubAssign};
use std::ptr::{self, NonNull};
use std::alloc;

use crate::n4_state::{Layout};

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
        for &(start, len, value) in &layout.orbit[1..] {
            unsafe {
                let dst = data.as_ptr().add(start);
                for i in 0..len {
                    dst.add(i).write(value);
                }
            }
        }
        Self { data, layout, corner }
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
        let mut result: Vec<Vec<(u8, u8)>> = Vec::with_capacity(self.layout.len);
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
        assert_eq!(vect.len(), layout.len);
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
        Self { data, layout, corner }
    }
}
impl<'a> Display for CubeState<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"{}\n\t0:\t{:?}",NAME[0],unpack_u128(self.corner as u128, BIT_PACKING[0]))?;
        for (i, &(start, len, _)) in self.layout.orbit.iter().enumerate() {
            if len > 0 {
                write!(f, "{}\n", NAME[i+1])?;
                for (j, &value) in self.extract(start, len).iter().enumerate() {
                    write!(f, "\t{}:\t{:?}\n", j, unpack_u128(value, BIT_PACKING[i+1]))?;
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
        writeln!(f, "[")?;
        writeln!(f,"\t[{}]",self.corner)?;
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
        
        Self {data, layout: self.layout, corner: self.corner}
    }
}
impl<'a> AddAssign<&CubeState<'a>> for CubeState<'a> {
    fn add_assign(&mut self, rhs: &CubeState<'a>) {
        let orbit = self.layout.orbit;

        add_8(&mut self.corner, rhs.corner);

        if self.layout.n & 1 == 1 {
            for (a, &b) in self
                .extract_mut(orbit[0].0, orbit[0].1)
                .iter_mut()
                .zip(rhs.extract(orbit[0].0, orbit[0].1))
            {
                add_12(a, b);
            }
            for (a, &b) in self
                .extract_mut(orbit[1].0, orbit[1].1)
                .iter_mut()
                .zip(rhs.extract(orbit[1].0, orbit[1].1))
            {
                add_6(a, b);
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

    fn add(mut self, rhs: &CubeState<'a>) -> Self {
        self += rhs;
        self
    }
}
impl<'a> SubAssign<&CubeState<'a>> for CubeState<'a> {
    fn sub_assign(&mut self, rhs: &CubeState<'a>) {
        let orbit = self.layout.orbit;

        sub_8(&mut self.corner, rhs.corner);

        if self.layout.n & 1 == 1 {
            for (a, &b) in self
                .extract_mut(orbit[0].0, orbit[0].1)
                .iter_mut()
                .zip(rhs.extract(orbit[0].0, orbit[0].1))
            {
                sub_12(a, b);
            }
            for (a, &b) in self
                .extract_mut(orbit[1].0, orbit[1].1)
                .iter_mut()
                .zip(rhs.extract(orbit[1].0, orbit[1].1))
            {
                sub_6(a, b);
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

    fn sub(mut self, rhs: &CubeState<'a>) -> Self {
        self -= rhs;
        self
    }
}

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
pub fn add_6(a: &mut u128, b: u128) {
    let p = *a;
    let mut out = 0u128;
    for shift in (0..30).step_by(5) {
        let b_block = ((b >> shift) & 31) * 5;
        let a_block = (p >> b_block) & 31;

        out |= a_block << shift;
    }
    *a = out
}
#[inline(always)]
pub fn add_8(a: &mut u64, b: u64) {
    let p = *a;
    let mut out: u64 = 0;
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
pub fn add_24(a: &mut u128, b: u128) {
    let p = *a;
    let mut out = 0u128;
    for shift in (0..120).step_by(5) {
        let b_block = ((b >> shift) & 31) * 5;
        let a_block = (p >> b_block) & 31;

        out |= a_block << shift;
    }
    *a = out
}
#[inline(always)]
pub fn sub_6(a: &mut u128, b: u128) {
    let p = *a;
    let mut out = 0;
    for shift in (0..30).step_by(5) {
        let b_block = ((b >> shift) & 31) * 5;
        let a_block = (p >> shift) & 31;

        out |= a_block << b_block;
    }
    *a = out;
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

        let new_ori = ((a_ori + 2 - b_ori) & 2) << 4;

        out |= (a_perm | new_ori) << b_perm;
    }
    *a = out;
}
#[inline(always)]
pub fn sub_24(a: &mut u128, b: u128) {
    let p = *a;
    let mut out = 0;
    for shift in (0..120).step_by(5) {
        let b_block = ((b >> shift) & 31) * 5;
        let a_block = (p >> shift) & 31;

        out |= a_block << b_block;
    }
    *a = out;
}
