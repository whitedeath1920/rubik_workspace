/// This implements the "Sparse Set" in a way that is efficient for Rubik's Cube permutations. \
/// Encoding the permutation of each orbit of the cube, each cubie es a 5 bit packed with the id
/// and the orientation, the permutation is defined as the position in said bitpacked array.
// #[repr(C)]
pub struct CubeArena {
    /// This correspond to the one Cube
    /// [  0  |  1  |...|n_orbit]
    /// [orbit|orbit|...| orbit]
    /// each orbit is a 5 bit packed number that contains the permutation \
    /// and for the 12 and 8 variant it also encodes the orientation
    orbits: Vec<u128>,
    /// Contains the Ids to the orbits
    sparse: Vec<u32>,
    /// Contains the dense array
    dense_id: Vec<u32>,
    /// Number of orbits per cube
    n_orbits: usize,
    /// Number of layer per cube
    n: usize,
}
/// This struct represents a mutable state of a Rubik's Cube.
#[repr(C)]
#[derive(Debug)]
pub struct CubeStateMut<'a> {
    orbits: &'a mut [u128],
}
impl CubeStateMut<'_> {
    pub fn zero(&mut self, n: usize) {
        let dim_mod_2 = n & 1;
        let tmp1 = (n - 2 - dim_mod_2) >> 1;
        let orbits = [
            (1, 247132686368),                                   // Corner Packed permutation
            (dim_mod_2, 407901468851537952),                     // Edge Packed permutation
            (dim_mod_2, 172066848),                              // Center packed permutation
            (tmp1.pow(2), 984818244535754528103549039458486304), // 24-pieces orbit packed permutation
            (tmp1, 984818244535754528103549039458486304),
            (tmp1 * dim_mod_2, 984818244535754528103549039458486304),
        ];
        let len = (n.pow(2) + 5 * (n & 1) - 2 * n + 4) / 4;
        debug_assert!(len == self.orbits.len());

        let mut idx = 0;
        orbits.iter().for_each(|(len, val)| {
            for _ in 0..*len {
                self.orbits[idx] = *val;
                idx += 1;
            }
        });
    }
}
/// This struct represents an immutable state of a Rubik's Cube.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct CubeStateRef<'a> {
    orbits: &'a [u128],
}
impl CubeArena {
    #[inline(always)]
    pub fn n_orbits(&self) -> usize {
        self.n_orbits
    }
    #[inline(always)]
    pub fn n(&self) -> usize {
        self.n
    }
    /// Initialize the cube arena with a given capacity and populated with all zeros.
    #[inline]
    pub fn with_capacity(n: usize, max_ids: usize) -> Self {
        debug_assert!(n > 1);
        debug_assert!(max_ids > 1);

        let n_orbits = (n.pow(2) + 5 * (n & 1) - 2 * n + 4) / 4;

        Self {
            orbits: vec![0; max_ids * n_orbits],
            sparse: vec![0; max_ids],
            dense_id: Vec::with_capacity(max_ids),
            n_orbits,
            n,
        }
    }
    #[inline]
    pub fn zeros(n: usize, max_ids: usize) -> Self {
        debug_assert!(n > 1);
        debug_assert!(max_ids > 1);

        let dim_mod_2 = n & 1;
        let tmp1 = (n - 2 - dim_mod_2) >> 1;
        let orbits: [(usize, u128); 6] = [
            (1, 247132686368),                                   // Corner Packed permutation
            (dim_mod_2, 407901468851537952),                     // Edge Packed permutation
            (dim_mod_2, 172066848),                              // Center packed permutation
            (tmp1.pow(2), 984818244535754528103549039458486304), // 24-pieces orbit packed permutation
            (tmp1, 984818244535754528103549039458486304),
            (tmp1 * dim_mod_2, 984818244535754528103549039458486304),
        ];
        let n_orbits = (n.pow(2) + 5 * (n & 1) - 2 * n + 4) / 4;

        let mut template = Vec::with_capacity(n_orbits);
        orbits.iter().for_each(|(len, val)| {
            for _ in 0..*len {
                template.push(*val);
            }
        });
        let mut orbits: Vec<u128> = Vec::with_capacity(n_orbits * max_ids);
        for _ in 0..max_ids {
            orbits.extend_from_slice(&template);
        }
        let sparse = Vec::from_iter(0..max_ids as u32);
        let dense_id = Vec::from_iter(0..max_ids as u32);
        Self {
            orbits,
            sparse,
            dense_id,
            n_orbits,
            n,
        }
    }
    #[inline]
    pub fn contains(&self, id: usize) -> bool {
        if id >= self.sparse.len() {
            return false;
        }
        let dense_index = self.sparse[id] as usize;

        dense_index < self.dense_id.len() && self.dense_id[dense_index] == id as u32
    }
    /// Get a reference to a cube state.
    #[inline]
    pub fn get(&self, id: usize) -> CubeStateRef<'_> {
        debug_assert!(self.contains(id));

        let n_orbits = self.n_orbits;
        let start = self.sparse[id] as usize * n_orbits;
        let end = start + n_orbits;
        let orbits = &self.orbits[start..end];
        CubeStateRef { orbits }
    }
    /// Get a mutable reference to a cube state.
    #[inline]
    pub fn get_mut(&mut self, id: usize) -> CubeStateMut<'_> {
        debug_assert!(self.contains(id));

        let n_orbits = self.n_orbits;
        let start = self.sparse[id] as usize * n_orbits;
        let end = start + n_orbits;
        let orbits = &mut self.orbits[start..end];
        CubeStateMut { orbits }
    }
    #[inline]
    pub fn insert(&mut self, id: usize, state: &[u128]) {
        debug_assert!(!self.contains(id));
        debug_assert!(state.len() == self.n_orbits);

        let dense_id = self.dense_id.len();
        let n_orbits = self.n_orbits;
        let start = dense_id * n_orbits;
        self.dense_id.push(id as u32);
        self.sparse[id] = dense_id as u32;

        unsafe {
            let ptr = self.orbits.as_mut_ptr().add(start);
            ptr.copy_from_nonoverlapping(state.as_ptr(), n_orbits);
        }
    }
    #[inline]
    pub fn remove(&mut self, id: usize) {
        debug_assert!(self.contains(id));
        let dense_index = self.sparse[id] as usize;
        let last_index = self.dense_id.len() - 1;

        self.dense_id.swap(dense_index, last_index);

        let swapped_id = self.dense_id[dense_index] as usize;
        self.sparse[swapped_id] = dense_index as u32;

        self.dense_id.pop();

        let n = self.n_orbits;

        let a = dense_index * n;
        let b = last_index * n;

        unsafe {
            std::ptr::swap_nonoverlapping(
                self.orbits.as_mut_ptr().add(a),
                self.orbits.as_mut_ptr().add(b),
                n,
            );
        }
    }
    #[inline]
    pub fn add_assign(&mut self, a: usize, b: usize) {
        debug_assert!(self.contains(a));
        debug_assert!(self.contains(b));
        let ptr_orbits = self.orbits.as_mut_ptr();
        let n_orbits = self.n_orbits;
        let a_start = self.sparse[a] as usize * n_orbits;
        let b_start = self.sparse[b] as usize * n_orbits;

        unsafe {
            add_8(&mut *ptr_orbits.add(a_start), *ptr_orbits.add(b_start));

            let mut idx = 1;
            if self.n & 1 == 0 {
                add_12(
                    &mut *ptr_orbits.add(a_start + 1),
                    *ptr_orbits.add(b_start + 1),
                );
                add_6(
                    &mut *ptr_orbits.add(a_start + 2),
                    *ptr_orbits.add(b_start + 2),
                );
                idx = 3;
            }
            for i in idx..n_orbits {
                add_24(
                    &mut *ptr_orbits.add(a_start + i),
                    *ptr_orbits.add(b_start + i),
                );
            }
        }
    }
    // #[inline(always)]
    // pub fn add_assign(&mut self, a: usize, b: usize) {
    //     debug_assert!(self.contains(a));
    //     debug_assert!(self.contains(b));
    
    //     let n_orbits = self.n_orbits;
    //     let even = self.n & 1 == 0;
    
    //     let base = self.orbits.as_mut_ptr();
    
    //     let a_ptr = unsafe { base.add(self.sparse[a] as usize * n_orbits) };
    //     let b_ptr = unsafe { base.add(self.sparse[b] as usize * n_orbits) };
    
    //     unsafe {
    //         // orbit 0 always exists
    //         add_8(&mut *a_ptr, *b_ptr);
    
    //         if n_orbits == 1 {
    //             return;
    //         }
    
    //         let mut offset = 1;
    
    //         if even {
    //             add_12(&mut *a_ptr.add(1), *b_ptr.add(1));
    //             add_6(&mut *a_ptr.add(2), *b_ptr.add(2));
    //             offset = 3;
    //         }
    
    //         let mut ap = a_ptr.add(offset);
    //         let mut bp = b_ptr.add(offset);
    
    //         let remaining = n_orbits - offset;
    
    //         for _ in 0..remaining {
    //             add_24(&mut *ap, *bp);
    //             ap = ap.add(1);
    //             bp = bp.add(1);
    //         }
    //     }
    // }
    #[inline]
    pub fn sub_assign(&mut self, a: usize, b: usize) {
        debug_assert!(self.contains(a));
        debug_assert!(self.contains(b));
        let ptr_orbits = self.orbits.as_mut_ptr();
        let n_orbits = self.n_orbits;
        let a_start = self.sparse[a] as usize * n_orbits;
        let b_start = self.sparse[b] as usize * n_orbits;

        unsafe {
            sub_8(&mut *ptr_orbits.add(a_start), *ptr_orbits.add(b_start));

            if n_orbits > 1 {
                let mut idx = 1;
                if self.n & 1 == 0 {
                    sub_12(
                        &mut *ptr_orbits.add(a_start + 1),
                        *ptr_orbits.add(b_start + 1),
                    );
                    sub_6(
                        &mut *ptr_orbits.add(a_start + 2),
                        *ptr_orbits.add(b_start + 2),
                    );
                    idx = 3;
                }
                for i in idx..n_orbits {
                    sub_24(
                        &mut *ptr_orbits.add(a_start + i),
                        *ptr_orbits.add(b_start + i),
                    );
                }
            }
        }
    }
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
pub fn add_8(a: &mut u128, b: u128) {
    let p = *a;
    let mut out: u128 = 0;
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
pub fn sub_8(a: &mut u128, b: u128) {
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
