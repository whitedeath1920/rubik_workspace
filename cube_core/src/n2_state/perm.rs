const SHIFT: [u8; 24] = {
    let mut arr = [0; 24];
    let mut i = 0;
    while i < 24 {
        arr[i] = (i * 5) as u8;
        i += 1;
    }
    arr
};
const MASK_SET: [u128; 24] = {
    let mut arr = [0; 24];
    let mut i = 0;
    while i < 24 {
        arr[i] = !(0b11111 << (i * 5));
        i += 1;
    }
    arr
};
const NUM_PER_KIND: [usize; 6] = [8, 12, 6, 24, 24, 24];
pub const IDENTITY_PERM: [u128; 6] = [
    247132686368,
    42535295865117307933329727397822564384,
    85070591730234615865843651858114119712,
    128590705839887678326869026826371565600,
    171126001705004986259790852755342592032,
    213661297570122294192712678684313618464,
];

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Perm<const KIND: usize> {
    data: u128,
}
impl<const KIND: usize> Perm<KIND> {
    #[inline]
    pub fn get(&self, i: usize) -> u8 {
        unsafe { ((self.data >> *SHIFT.get_unchecked(i)) & 0b11111) as u8 }
    }
    #[inline]
    pub fn set(&mut self, i: usize, v: u8) {
        unsafe {
            self.data &= *MASK_SET.get_unchecked(i);
            self.data |= (v as u128) << *SHIFT.get_unchecked(i);
        }
    }
    #[inline]
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
                acc | ((v as u128) << *SHIFT.get_unchecked(i))
            });
            Self { data }
        }
    }
    #[inline]
    pub fn new() -> Self {
        unsafe {
            Self {
                data: *IDENTITY_PERM.get_unchecked(KIND),
            }
        }
    }
    #[inline]
    pub fn raw_data(&self) -> u128 {
        self.data
    }    
}
impl<const KIND:usize> Default for Perm<KIND> {
    fn default() -> Self {
        Self::new()
    }
}
