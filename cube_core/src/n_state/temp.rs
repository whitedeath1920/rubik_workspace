#![allow(dead_code)]
extern crate alloc;

use alloc::alloc::{alloc, alloc_zeroed, dealloc, handle_alloc_error};
use core::{
    alloc::Layout,
    marker::PhantomData,
    mem::{align_of, size_of, MaybeUninit},
    ops::{Index, IndexMut},
    ptr::{self, NonNull},
    slice,
};

/// Fast fixed-length buffer of u128 with explicit (ptr,len).
/// ALIGN is optional over-alignment (default 64). Must be power-of-two.
pub struct U128Buf<const ALIGN: usize = 64> {
    ptr: NonNull<u128>,
    len: usize,
}

pub struct U128BufUninit<const ALIGN: usize = 64> {
    ptr: NonNull<MaybeUninit<u128>>,
    len: usize,
}

impl<const ALIGN: usize> U128Buf<ALIGN> {
    #[inline(always)]
    fn layout_for(len: usize) -> Layout {
        debug_assert!(len > 1);
        let a = ALIGN.max(align_of::<u128>());
        debug_assert!(a.is_power_of_two());
        let size = len.checked_mul(size_of::<u128>()).expect("size overflow");
        Layout::from_size_align(size, a).unwrap()
    }

    /// Allocate uninitialized (fast). You should init immediately.
    #[inline(always)]
    pub fn with_capacity(len: usize) -> U128BufUninit<ALIGN> {
        U128BufUninit::with_capacity(len)
    }

    /// Allocate and zero.
    #[inline(always)]
    pub fn zeroed(len: usize) -> Self {
        assert!(len > 1);
        unsafe {
            let layout = Self::layout_for(len);
            let raw = alloc_zeroed(layout) as *mut u128;
            if raw.is_null() {
                handle_alloc_error(layout);
            }
            Self { ptr: NonNull::new_unchecked(raw), len }
        }
    }

    /// Allocate and memcpy from slice.
    #[inline(always)]
    pub fn from_slice(src: &[u128]) -> Self {
        assert!(src.len() > 1);
        U128BufUninit::<ALIGN>::with_capacity(src.len()).init_from_slice(src)
    }

    /// Allocate and initialize sequentially.
    #[inline(always)]
    pub fn from_fn(len: usize, f: impl FnMut(usize) -> u128) -> Self {
        assert!(len > 1);
        U128BufUninit::<ALIGN>::with_capacity(len).init_with(f)
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline(always)]
    pub fn as_ptr(&self) -> *const u128 {
        self.ptr.as_ptr()
    }

    #[inline(always)]
    pub fn as_mut_ptr(&mut self) -> *mut u128 {
        self.ptr.as_ptr()
    }

    /// Best for hot sequential loops (hoist once).
    #[inline(always)]
    pub fn as_ptr_len(&self) -> (*const u128, usize) {
        (self.as_ptr(), self.len)
    }

    /// Best for hot sequential loops (hoist once).
    #[inline(always)]
    pub fn as_mut_ptr_len(&mut self) -> (*mut u128, usize) {
        (self.as_mut_ptr(), self.len)
    }

    /// Only build slices when you actually want slice APIs.
    #[inline(always)]
    pub fn as_slice(&self) -> &[u128] {
        unsafe { slice::from_raw_parts(self.as_ptr(), self.len) }
    }

    #[inline(always)]
    pub fn as_mut_slice(&mut self) -> &mut [u128] {
        unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), self.len) }
    }

    #[inline(always)]
    pub fn iter(&self) -> Iter<'_> {
        let p = self.as_ptr();
        unsafe { Iter { cur: p, end: p.add(self.len), _pd: PhantomData } }
    }

    #[inline(always)]
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        let p = self.as_mut_ptr();
        unsafe { IterMut { cur: p, end: p.add(self.len), _pd: PhantomData } }
    }

    // ---- getters / setters ----

    #[inline(always)]
    pub fn get(&self, i: usize) -> Option<u128> {
        if i < self.len { unsafe { Some(self.get_unchecked(i)) } } else { None }
    }

    #[inline(always)]
    pub fn get_ref(&self, i: usize) -> Option<&u128> {
        if i < self.len {
            unsafe { Some(&*self.as_ptr().add(i)) }
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn get_mut(&mut self, i: usize) -> Option<&mut u128> {
        if i < self.len {
            unsafe { Some(&mut *self.as_mut_ptr().add(i)) }
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn set(&mut self, i: usize, v: u128) -> bool {
        if i < self.len {
            unsafe { self.set_unchecked(i, v) }
            true
        } else {
            false
        }
    }

    /// # Safety: i < len
    #[inline(always)]
    pub unsafe fn get_unchecked(&self, i: usize) -> u128 {
        *self.as_ptr().add(i)
    }

    /// # Safety: i < len
    #[inline(always)]
    pub unsafe fn set_unchecked(&mut self, i: usize, v: u128) {
        *self.as_mut_ptr().add(i) = v;
    }

    #[inline(always)]
    pub fn copy_from_slice(&mut self, src: &[u128]) {
        assert!(src.len() == self.len);
        unsafe {
            ptr::copy_nonoverlapping(src.as_ptr(), self.as_mut_ptr(), self.len);
        }
    }
}

impl<const ALIGN: usize> U128BufUninit<ALIGN> {
    #[inline(always)]
    pub fn with_capacity(len: usize) -> Self {
        assert!(len > 1);
        unsafe {
            let layout = U128Buf::<ALIGN>::layout_for(len);
            let raw = alloc(layout) as *mut MaybeUninit<u128>;
            if raw.is_null() {
                handle_alloc_error(layout);
            }
            Self { ptr: NonNull::new_unchecked(raw), len }
        }
    }

    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
    }

    #[inline(always)]
    pub fn as_mut_ptr(&mut self) -> *mut MaybeUninit<u128> {
        self.ptr.as_ptr()
    }

    /// # Safety: i < len
    #[inline(always)]
    pub unsafe fn write_unchecked(&mut self, i: usize, v: u128) {
        self.as_mut_ptr().add(i).write(MaybeUninit::new(v));
    }

    /// Safe: guarantees full initialization.
    #[inline(always)]
    pub fn init_with(mut self, mut f: impl FnMut(usize) -> u128) -> U128Buf<ALIGN> {
        for i in 0..self.len {
            unsafe { self.write_unchecked(i, f(i)); }
        }
        unsafe { self.assume_init() }
    }

    /// Safe: memcpy then seals as initialized.
    #[inline(always)]
    pub fn init_from_slice(mut self, src: &[u128]) -> U128Buf<ALIGN> {
        assert!(src.len() == self.len);
        unsafe {
            ptr::copy_nonoverlapping(src.as_ptr(), self.as_mut_ptr() as *mut u128, self.len);
            self.assume_init()
        }
    }

    /// # Safety: all elements must be initialized.
    pub unsafe fn assume_init(self) -> U128Buf<ALIGN> {
        let out = U128Buf {
            ptr: NonNull::new_unchecked(self.ptr.as_ptr() as *mut u128),
            len: self.len,
        };
        core::mem::forget(self);
        out
    }
}

// ---- iterators: pointer-walk, minimal overhead ----

pub struct Iter<'a> {
    cur: *const u128,
    end: *const u128,
    _pd: PhantomData<&'a u128>,
}
impl<'a> Iterator for Iter<'a> {
    type Item = &'a u128;
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur == self.end {
            None
        } else {
            unsafe {
                let p = self.cur;
                self.cur = self.cur.add(1);
                Some(&*p)
            }
        }
    }
}

pub struct IterMut<'a> {
    cur: *mut u128,
    end: *mut u128,
    _pd: PhantomData<&'a mut u128>,
}
impl<'a> Iterator for IterMut<'a> {
    type Item = &'a mut u128;
    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur == self.end {
            None
        } else {
            unsafe {
                let p = self.cur;
                self.cur = self.cur.add(1);
                Some(&mut *p)
            }
        }
    }
}

// ---- traits: Index, Clone, Eq, Drop ----

impl<const ALIGN: usize> Index<usize> for U128Buf<ALIGN> {
    type Output = u128;
    #[inline(always)]
    fn index(&self, i: usize) -> &Self::Output {
        assert!(i < self.len);
        unsafe { &*self.as_ptr().add(i) }
    }
}
impl<const ALIGN: usize> IndexMut<usize> for U128Buf<ALIGN> {
    #[inline(always)]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        assert!(i < self.len);
        unsafe { &mut *self.as_mut_ptr().add(i) }
    }
}

impl<const ALIGN: usize> Clone for U128Buf<ALIGN> {
    fn clone(&self) -> Self {
        let mut uninit = U128BufUninit::<ALIGN>::with_capacity(self.len);
        unsafe {
            ptr::copy_nonoverlapping(self.as_ptr(), uninit.as_mut_ptr() as *mut u128, self.len);
            uninit.assume_init()
        }
    }
}

impl<const ALIGN: usize> PartialEq for U128Buf<ALIGN> {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        if self.len != other.len { return false; }
        // Fast path: compare as bytes (valid for integers like u128).
        let nbytes = self.len * size_of::<u128>();
        unsafe {
            let a = slice::from_raw_parts(self.as_ptr() as *const u8, nbytes);
            let b = slice::from_raw_parts(other.as_ptr() as *const u8, nbytes);
            a == b
        }
    }
}
impl<const ALIGN: usize> Eq for U128Buf<ALIGN> {}

impl<const ALIGN: usize> Drop for U128Buf<ALIGN> {
    fn drop(&mut self) {
        unsafe {
            let layout = Self::layout_for(self.len);
            dealloc(self.as_mut_ptr() as *mut u8, layout);
        }
    }
}
impl<const ALIGN: usize> Drop for U128BufUninit<ALIGN> {
    fn drop(&mut self) {
        unsafe {
            let layout = U128Buf::<ALIGN>::layout_for(self.len);
            dealloc(self.as_mut_ptr() as *mut u8, layout);
        }
    }
}
