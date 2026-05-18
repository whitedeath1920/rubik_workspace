use std::{alloc::{Layout, alloc, handle_alloc_error}, fmt::{Debug, Display}, ptr::NonNull};

extern crate alloc;

#[repr(C)]
#[derive(Debug)]
pub struct Array<T, const ALIGN: usize = 16> {
    ptr: NonNull<T>,
    len: usize,
}

impl<T, const ALIGN: usize> Array<T, ALIGN> {
    #[inline(always)]
    fn layout(n: usize) -> Layout {
        let align = ALIGN.max(align_of::<T>());
        debug_assert!(align.is_power_of_two());
        let size = n.checked_mul(size_of::<T>()).expect("size overflow");
        unsafe  { Layout::from_size_align_unchecked(size, align) }
    }
    
    #[inline(always)]
    pub fn with_capacity(n: usize) -> Self {
        debug_assert!(n > 0);
        let layout = Self::layout(n);
        unsafe {
            let raw = alloc(layout) as *mut T;
            if raw.is_null() {
                handle_alloc_error(layout);
            }
            Self { 
                ptr: NonNull::new_unchecked(raw),
                len: n 
            }
        }
    }
    #[inline(always)]
    pub fn write(&mut self, index: usize, value: T) {
        unsafe { self.ptr.add(index).write(value); }
    }
    #[inline(always)]
    pub fn len(&self) -> usize {
        self.len
    }
}
impl<T: Debug, const ALIGN: usize> Display for Array<T, ALIGN> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f,"[")?;
        for i in 0..self.len {
            unsafe { writeln!(f,"\t[{:?}],",*self.ptr.as_ptr().add(i))?; }
        }
        writeln!(f,"]")?;
        Ok(())
    }
}