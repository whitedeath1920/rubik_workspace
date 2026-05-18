use core::slice;
use std::{
    alloc::{Layout, alloc, handle_alloc_error},
    fmt::Display,
    ops::{Add, AddAssign},
    ptr::{self, NonNull},
};

#[repr(C)]
pub struct Array<T: Clone, const ALIGN: usize = 64> {
    ptr: NonNull<T>,
    len: usize,
}

impl<T: Clone, const ALIGN: usize> Array<T, ALIGN> {
    #[inline]
    pub fn with_capacity(len: usize) -> Self {
        debug_assert!(len > 0);
        let align = ALIGN.max(align_of::<T>());
        debug_assert!(align.is_power_of_two());
        let size = len.checked_mul(size_of::<T>()).expect("size overflow");
        unsafe {
            let layout = Layout::from_size_align_unchecked(size, align);
            let raw = alloc(layout) as *mut T;
            if raw.is_null() {
                handle_alloc_error(layout);
            }
            Self {
                ptr: NonNull::new_unchecked(raw),
                len,
            }
        }
    }
    #[inline(always)]
    pub fn write(&mut self, index: usize, v: T) {
        debug_assert!(index < self.len);
        unsafe {
            self.ptr.add(index).write(v);
        }
    }
    // #[inline(always)]
    // pub fn as_mut_ptr(&mut self) -> *mut T {
    //     self.ptr.as_ptr()
    // }
    // #[inline(always)]
    // pub fn as_ptr(&self) -> *const T {
    //     self.ptr.as_ptr()
    // }
    #[inline(always)]
    pub fn as_slice(&self) -> &[T] {
        unsafe { slice::from_raw_parts(self.ptr.as_ptr(), self.len) }
    }
    #[inline(always)]
    pub fn to_vec(&self) -> Vec<T> {
        self.as_slice().to_vec()
    }
    #[inline(always)]
    pub fn from_slice(src: &[T]) -> Self {
        let out = Self::with_capacity(src.len());
        unsafe {
            ptr::copy_nonoverlapping(src.as_ptr(), out.ptr.as_ptr(), src.len());
        }
        out
    }
}

impl<T: Default + Copy + Clone, const ALIGN: usize> Default for Array<T, ALIGN> {
    fn default() -> Self {
        let mut s = Self::with_capacity(1);
        s.write(0, T::default());
        s
    }
}
impl<T: Clone + Default + Display, const ALIGN: usize> Display for Array<T, ALIGN> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "[\n")?;
        for a in self.as_slice() {
            write!(f, "\t{}\n", a)?;
        }
        write!(f, "]")?;
        Ok(())
    }
}
impl<T: Clone, const ALIGN: usize> Add<&Array<T, ALIGN>> for Array<T, ALIGN> {
    type Output = Self;
    fn add(mut self, rhs: &Self) -> Self::Output {
        self.add_assign(rhs);
        self
    }
}
impl<T: Clone, const ALIGN: usize> AddAssign<&Array<T, ALIGN>> for Array<T, ALIGN> {
    fn add_assign(&mut self, _rhs: &Self) {
        todo!()
    }
}
impl<T: Clone, const ALIGN: usize> Clone for Array<T,ALIGN> {
    #[inline(always)]
    fn clone(&self) -> Self {
        let mut out = Self::with_capacity(self.len);
        unsafe {
            ptr::copy_nonoverlapping(self.ptr.as_ptr(), out.ptr.as_ptr(), self.len);
        }
        out
    }
}