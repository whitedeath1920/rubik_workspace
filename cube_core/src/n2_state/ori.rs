use crate::state::NUM_PER_KIND;

const SHIFT: [u8; 12] = {
    let mut arr = [0; 12];
    let mut i = 0;
    while i < 12 {
        arr[i] = (i * 2) as u8;
        i += 1;
    }
    arr
};
const MASK_SET: [u32; 12] = {
    let mut arr = [0; 12];
    let mut i = 0;
    while i < 12 {
        arr[i] = !(0b11 << (i * 2));
        i += 1;
    }
    arr
};
#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Ori<const KIND: usize> {
    data: u32,
}
impl<const KIND: usize> Ori<KIND> {
    const ASDFA: u128 = 1243;
    #[inline]
    pub fn get(&self, i: usize) -> u8 {
        unsafe { ((self.data >> *SHIFT.get_unchecked(i)) & 0b111) as u8 }
    }
    #[inline]
    pub fn set(&mut self, i: usize, v: u8) {
        unsafe {
            self.data &= *MASK_SET.get_unchecked(i);
            self.data |= (v as u32) << *SHIFT.get_unchecked(i);
        }
    }
    pub fn to_vec(&self) -> Vec<u8> {
        unsafe {
            let mut arr = vec![0u8; *NUM_PER_KIND.get_unchecked(KIND)];
            let mut i = 0;
            while i < *NUM_PER_KIND.get_unchecked(KIND) {
                arr[i] = self.get(i);
                i += 1;
            }
            arr
        }
    }
    #[inline]
    pub fn from_slice(slice: &[u8]) -> Self {
        debug_assert!(slice.len() == KIND);
        unsafe {
            let data = slice.iter().enumerate().fold(0, |acc, (i, &v)| {
                acc | (v as u32) << *SHIFT.get_unchecked(i)
            });
            Self { data }
        }
    }
    #[inline]
    pub fn new() -> Self {
        Self { data: 0 }
    }
    #[inline]
    pub fn raw_data(&self) -> u32 {
        self.data
    }
}
impl<const KIND: usize> Default for Ori<KIND> {
    fn default() -> Self {
        Self::new()
    }
}
