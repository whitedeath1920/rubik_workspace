extern crate alloc;

use alloc::alloc::{alloc, alloc_zeroed, dealloc, handle_alloc_error};

use core::{
    alloc::Layout,
    marker::PhantomData,
    mem::{align_of, size_of},
    ptr::{self, NonNull},
    slice,
};

#[repr(C, align(16))]
#[derive(Debug)]
pub struct Array {
    ptr: NonNull<u128>,
    len: usize,
}

impl Array {
    #[inline(always)]
    fn layout_for(len: usize) -> Layout {
        debug_assert!(len >= 1);
        let align = align_of::<u128>();
        debug_assert!(align.is_power_of_two());
        let size = len.checked_mul(size_of::<u128>()).expect("size overflow");
        unsafe { Layout::from_size_align_unchecked(size, align) }
    }
    #[inline(always)]
    pub fn with_capacity(len: usize) -> Self {
        debug_assert!(len >= 1);
        unsafe {
            let layout = Self::layout_for(len);
            let raw = alloc(layout) as *mut u128;
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
    pub fn write(&mut self, index: usize, value: u128) {
        debug_assert!(index < self.len);
        unsafe {
            self.ptr.add(index).write(value);
        }
    }
    #[inline(always)]
    pub fn zeroed(len: usize) -> Self {
        debug_assert!(len > 1);
        unsafe {
            let layout = Self::layout_for(len);
            let raw = alloc_zeroed(layout) as *mut u128;
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
    pub fn get(&self, index: usize) -> u128 {
        debug_assert!(index < self.len);
        unsafe { *self.ptr.as_ptr().add(index) }
    }
    #[inline(always)]
    pub fn get_mut(&mut self, index: usize) -> &mut u128 {
        debug_assert!(index < self.len);
        unsafe { &mut *self.ptr.as_ptr().add(index) }
    }
    #[inline(always)]
    pub fn iter(&self) -> Iter<'_> {
        let p = self.as_ptr();
        unsafe {
            Iter {
                cur: p,
                end: p.add(self.len),
                _pd: PhantomData,
            }
        }
    }
    #[inline(always)]
    pub fn fill(&mut self, val: u128, start: usize, count: usize) {
        let slc = unsafe { slice::from_raw_parts_mut(self.as_mut_ptr().add(start), count) };
        slc.fill(val);
    }
    #[inline(always)]
    pub fn iter_mut(&mut self) -> IterMut<'_> {
        let p = self.as_mut_ptr();
        unsafe {
            IterMut {
                cur: p,
                end: p.add(self.len),
                _pd: PhantomData,
            }
        }
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

    #[inline(always)]
    pub fn as_slice(&self) -> &[u128] {
        unsafe { slice::from_raw_parts(self.as_ptr(), self.len) }
    }
    #[inline(always)]
    pub fn as_mut_slice(&mut self) -> &mut [u128] {
        unsafe { slice::from_raw_parts_mut(self.as_mut_ptr(), self.len) }
    }
    #[inline(always)]
    pub fn from_slice(src: &[u128]) -> Self {
        let mut out = Self::with_capacity(src.len());
        unsafe {
            ptr::copy_nonoverlapping(src.as_ptr(), out.as_mut_ptr(), src.len());
        }
        out
    }
}

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

impl PartialEq for Array {
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        let nbytes = self.len * size_of::<u128>();
        unsafe {
            let a = slice::from_raw_parts(self.as_ptr() as *const u8, nbytes);
            let b = slice::from_raw_parts(other.as_ptr() as *const u8, nbytes);
            a == b
        }
    }
}

impl Clone for Array {
    #[inline(always)]
    fn clone(&self) -> Self {
        let mut out = Self::with_capacity(self.len());
        unsafe {
            ptr::copy_nonoverlapping(self.as_ptr(), out.as_mut_ptr(), self.len());
        }
        out
    }
}

impl Drop for Array {
    #[inline(always)]
    fn drop(&mut self) {
        unsafe {
            let layout = Self::layout_for(self.len);
            dealloc(self.ptr.as_ptr() as *mut u8, layout);
        }
    }
}
