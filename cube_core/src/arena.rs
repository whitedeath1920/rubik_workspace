use std::ptr;

use rand::RngExt;

use crate::{CubeError, error::Result};

/// The 15-bit permutation lookup table used by `add_perm`/`sub_perm`.
/// Each entry `[a, b, c]` decomposes a 15-bit index into three 5-bit
/// piece indices, enabling fast piece extraction from packed permutations.
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
/// Rotation-move sentinel for the X axis, used by `normalize_cube`.
pub const X: u32 = 4;
/// Rotation-move sentinel for the Y axis, used by `normalize_cube`.
pub const Y: u32 = 4;
/// Rotation-move sentinel for the Z axis, used by `normalize_cube`.
pub const Z: u32 = 4;
/// Neutral/identity rotation sentinel for `normalize_cube`.
pub const ZERO: u32 = 4;

/// Trait abstracting over piece types (corners, edges, centers, generic
/// 24-piece orbits). Associated constants define the bit-level layout
/// within a packed u128 word:
///
/// - `ORI` — bitmask for the orientation field of a single piece.
/// - `PERM` — bitmask for the permutation/index field of a single piece.
/// - `LEN` — total number of pieces of this kind.
/// - `SHIFT` — bit offset of the orientation sub-field within the piece.
/// - `MOD` — modulus for orientation addition (e.g. 3 for corners, 2 for
///   edges, 1 when orientation is absent).
pub trait PieceKind {
    const ORI: u128;
    const PERM: u128;
    const LEN: usize;
    const SHIFT: u32;
    const MOD: u128;
}

/// Piece kind representing the 8 corners of a Rubik's cube.
/// Each corner uses a 3-bit permutation index (values 0-7) and a 2-bit
/// orientation field, with orientation arithmetic modulo 3.
pub struct Corner;
impl PieceKind for Corner {
    const ORI: u128 = 3;
    const PERM: u128 = 7;
    const LEN: usize = 8;
    const SHIFT: u32 = 3;
    const MOD: u128 = 3;
}

/// Piece kind representing the 12 single edges of a Rubik's cube.
/// Each edge uses a 4-bit permutation index (values 0-11) and a 1-bit
/// orientation field, with orientation arithmetic modulo 2.
pub struct Edge;
impl PieceKind for Edge {
    const ORI: u128 = 1;
    const PERM: u128 = 15;
    const LEN: usize = 12;
    const SHIFT: u32 = 4;
    const MOD: u128 = 2;
}

/// Piece kind representing the 6 fixed face centers.
/// Each center uses a 5-bit permutation index (values 0-5) and has no
/// orientation component (`ORI = 0`, `MOD = 1` to avoid division by
/// zero).
pub struct Center;
impl PieceKind for Center {
    const ORI: u128 = 0;
    const PERM: u128 = 31;
    const LEN: usize = 6;
    const SHIFT: u32 = 0;
    const MOD: u128 = 1; // to avoid any probable division by 0
}

/// Generic piece kind for a 24-piece orbit group (center-corners,
/// center-edges, coupled edges, center-edge-wings). Uses a 5-bit
/// permutation index with no orientation component.
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
    /// Stores all cubes contiguously in packed u128 format. Each cube
    /// occupies `stride` words; scratch slots live at the tail.
    data: Vec<u128>,
    /// Number of user-visible cubes in the arena (excluding scratch
    /// slots).
    len: usize,
    /// Maps (start_index, count) for the three 24-piece orbit families
    /// (center-corners, center-edges, coupled edges) in the packed
    /// layout.
    orbit: [(u16, u16); 3],
    /// Number of u128 words per cube in the packed `data` layout.
    stride: u16,
    /// Equals `stride + 2` when `n` is odd (edges + centers in cube[0]
    /// get two extra orbits), otherwise equals `stride`.
    len_orbits: u16,
    /// Dimension of the cube (e.g. 3 for a 3x3x3, 4 for a 4x4x4).
    n: u8,
}

impl CubeArena {
    /// Creates an arena holding `len` user cubes of dimension `n`.
    ///
    /// Allocates `len + 2` slots: the last two (indices `len` and `len + 1`)
    /// are scratch slots used for intermediate computations. All cubes are
    /// initialized to the solved (identity) state.
    pub fn new_arena(n: u8, len: usize) -> Self {
        assert!(len > 0);
        let n_mod_2 = (n & 1) as u16;
        let k_max = (n as u16 - 2 - n_mod_2) >> 1; // Temporal value represents de number of pieces between the corner en the edge
        let stride = k_max.pow(2) + k_max + k_max * n_mod_2 + 1;
        let orbit = [
            (1, k_max.pow(2)),
            (1 + k_max.pow(2), k_max),
            ((1 + k_max.pow(2) + k_max) * n_mod_2, k_max * n_mod_2),
        ];
        let mut data = vec![984818244535754528103549039458486304; (len + 2) * stride as usize];
        for i in 0..(len + 2) {
            data[i * stride as usize] = 247132686368
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
    /// Returns a shared reference to the packed `[u128]` slice for the cube
    /// at `index`.
    ///
    /// Panics in debug builds if `index >= self.len`.
    #[inline(always)]
    pub fn get_cube(&self, index: usize) -> &[u128] {
        debug_assert!(index < self.len);
        let start = index * self.stride as usize;
        &self.data[start..start + self.stride as usize]
    }
    /// Mutable variant of [`get_cube`](Self::get_cube).
    #[inline(always)]
    pub fn get_cube_mut(&mut self, index: usize) -> &mut [u128] {
        debug_assert!(index < self.len);
        let start = index * self.stride as usize;
        &mut self.data[start..start + self.stride as usize]
    }
    /// Copies an external packed `slice` into the arena at `index`.
    ///
    /// `slice` must have length equal to `stride`.
    #[inline(always)]
    pub fn cube_from_slice(&mut self, index: usize, slice: &[u128]) {
        debug_assert!(index < self.total_len());
        debug_assert!(slice.len() == self.stride as usize);

        let start = index * self.stride as usize;
        self.data[start..start + slice.len()].copy_from_slice(slice);
    }
    /// Unpacks the cube at `index` into a `Vec<Vec<(u8, u8)>>` where each
    /// inner `Vec` is an orbit group of `(perm, ori)` pairs.
    ///
    /// Useful for debugging and [`print_cube`](Self::print_cube).
    #[inline]
    pub fn cube_to_vec(&self, index: usize) -> Vec<Vec<(u8, u8)>> {
        let mut result: Vec<Vec<(u8, u8)>> = Vec::with_capacity(self.len_orbits as usize);
        unsafe {
            let cube_ptr = self.cube_ptr(index);

            ptr::write(
                result.as_mut_ptr().add(0),
                unpack_u128::<Corner>(get_corner(*cube_ptr.add(0)) as u128),
            );
            let mut offset = 0;
            if self.n & 1 == 1 {
                ptr::write(
                    result.as_mut_ptr().add(1),
                    unpack_u128::<Edge>(get_edge(*cube_ptr.add(0)) as u128),
                );
                ptr::write(
                    result.as_mut_ptr().add(2),
                    unpack_u128::<Center>(get_center(*cube_ptr.add(0)) as u128),
                );
                offset = 2;
            }
            for i in 1..self.stride as usize {
                ptr::write(
                    result.as_mut_ptr().add(i + offset),
                    unpack_u128::<Piece24>(*cube_ptr.add(i)),
                );
            }

            result.set_len(self.len_orbits as usize);
        }
        result
    }
    /// Packs a `Vec<Vec<(u8, u8)>>` back into the arena at `index`.
    ///
    /// Inverse of [`cube_to_vec`](Self::cube_to_vec).
    #[inline]
    pub fn cube_from_vec(&mut self, index: usize, cube: &[Vec<(u8, u8)>]) {
        debug_assert!(cube.len() == self.len_orbits as usize);
        debug_assert!(index < self.len);
        let n = self.n;

        unsafe {
            let c = self.cube_mut_ptr(index);
            let cube_ptr = cube.as_ptr();
            let mut offset = 0;
            let mut block = pack_u128::<Corner>(&*cube_ptr.add(0));
            if n & 1 == 1 {
                block |= pack_u128::<Edge>(&*cube_ptr.add(1)) << 40;
                block |= pack_u128::<Center>(&*cube_ptr.add(2)) << 100;
                offset = 2;
            }
            *c.add(0) = block;

            for i in 1..self.stride as usize {
                *c.add(i) = pack_u128::<Piece24>(&*cube_ptr.add(i + offset));
            }
        }
    }
    /// Returns a raw `*const u128` pointer to the start of cube `index`'s
    /// packed data.
    ///
    /// `index` can go up to `total_len() - 1` (includes scratch slots).
    #[inline(always)]
    pub fn cube_ptr(&self, index: usize) -> *const u128 {
        debug_assert!(index < self.total_len());
        unsafe { self.data.as_ptr().add(index * self.stride as usize) }
    }
    /// Mutable variant of [`cube_ptr`](Self::cube_ptr).
    #[inline(always)]
    pub fn cube_mut_ptr(&mut self, index: usize) -> *mut u128 {
        debug_assert!(index < self.total_len());
        unsafe { self.data.as_mut_ptr().add(index * self.stride as usize) }
    }
    /// c = a + b (group composition).
    ///
    /// Composes cubes `a` and `b` and stores the result in `c`. Uses
    /// `add_ori` for corners and edges (orientation-aware composition) and
    /// `add_perm` for 24-piece orbits. Handles aliasing automatically by
    /// copying to a scratch slot when needed.
    #[inline]
    pub fn add(&mut self, mut a: usize, mut b: usize, c: usize) {
        debug_assert!(a < self.total_len() && b < self.total_len() && c < self.total_len());
        if a == c && b == c {
            self.clone_cube(a, self.len);
            a = self.len;
            b = self.len;
        } else if a == c {
            self.clone_cube(a, self.len);
            a = self.len;
        } else if b == c {
            self.clone_cube(b, self.len);
            b = self.len;
        }
        unsafe {
            let c_a = self.cube_ptr(a);
            let c_b = self.cube_ptr(b);
            let c_c = self.cube_mut_ptr(c);

            let mut block = add_ori::<Corner>(get_corner(*c_a), get_corner(*c_b));
            if self.n & 1 == 1 {
                block |= add_ori::<Edge>(get_edge(*c_a), get_edge(*c_b)) << 40;
                block |= add_perm::<Center>(get_center(*c_a), get_center(*c_b)) << 100;
            }
            *c_c = block;

            for a in 1..self.stride as usize {
                *c_c.add(a) = add_perm::<Piece24>(*c_a.add(a), *c_b.add(a));
            }
        }
    }
    /// c = a + b where `b` is an external `&[u128]` slice instead of an arena
    /// index.
    ///
    /// Same semantics as [`add`](Self::add).
    #[inline]
    pub fn add_slice(&mut self, mut a: usize, b: &[u128], c: usize) {
        debug_assert!(
            a < self.total_len() && b.len() == self.stride as usize && c < self.total_len()
        );
        if a == c {
            self.clone_cube(a, self.len);
            a = self.len;
        }
        unsafe {
            let c_a = self.cube_ptr(a);
            let c_b = b.as_ptr();
            let c_c = self.cube_mut_ptr(c);

            let mut block = add_ori::<Corner>(get_corner(*c_a), get_corner(*c_b));
            if self.n & 1 == 1 {
                block |= add_ori::<Edge>(get_edge(*c_a), get_edge(*c_b)) << 40;
                block |= add_perm::<Center>(get_center(*c_a), get_center(*c_b)) << 100;
            }
            *c_c = block;

            for a in 1..self.stride as usize {
                *c_c.add(a) = add_perm::<Piece24>(*c_a.add(a), *c_b.add(a));
            }
        }
    }
    /// c = a - b (group inverse composition) where `b` is an external
    /// `&[u128]` slice.
    ///
    /// Inverse of [`add_slice`](Self::add_slice).
    #[inline]
    pub fn sub_slice(&mut self, mut a: usize, b: &[u128], c: usize) {
        debug_assert!(
            a < self.total_len() && b.len() == self.stride as usize && c < self.total_len()
        );
        if a == c {
            self.clone_cube(a, self.len);
            a = self.len;
        }
        unsafe {
            let c_a = self.cube_ptr(a);
            let c_b = b.as_ptr();
            let c_c = self.cube_mut_ptr(c);

            let mut block = sub_ori::<Corner>(get_corner(*c_a), get_corner(*c_b));
            if self.n & 1 == 1 {
                block |= sub_ori::<Edge>(get_edge(*c_a), get_edge(*c_b)) << 40;
                block |= sub_perm::<Center>(get_center(*c_a), get_center(*c_b)) << 100;
            }
            *c_c = block;

            for a in 1..self.stride as usize {
                *c_c.add(a) = sub_perm::<Piece24>(*c_a.add(a), *c_b.add(a));
            }
        }
    }
    /// c = a - b (group inverse composition).
    ///
    /// Inverse of [`add`](Self::add). Handles aliasing automatically.
    #[inline]
    pub fn sub(&mut self, mut a: usize, mut b: usize, c: usize) {
        debug_assert!(a < self.total_len() && b < self.total_len() && c < self.total_len());
        if a == c && b == c {
            self.clone_cube(a, self.len);
            a = self.len;
            b = self.len;
        } else if a == c {
            self.clone_cube(a, self.len);
            a = self.len;
        } else if b == c {
            self.clone_cube(b, self.len);
            b = self.len;
        }
        unsafe {
            let c_a = self.cube_ptr(a);
            let c_b = self.cube_ptr(b);
            let c_c = self.cube_mut_ptr(c);

            let mut block = sub_ori::<Corner>(get_corner(*c_a), get_corner(*c_b));
            if self.n & 1 == 1 {
                block |= sub_ori::<Edge>(get_edge(*c_a), get_edge(*c_b)) << 40;
                block |= sub_perm::<Center>(get_center(*c_a), get_center(*c_b)) << 100;
            }
            *c_c = block;

            for a in 1..self.stride as usize {
                *c_c.add(a) = sub_perm::<Piece24>(*c_a.add(a), *c_b.add(a));
            }
        }
    }
    /// Copies cube `a` to cube `b`.
    ///
    /// No-op if `a == b`.
    pub fn clone_cube(&mut self, a: usize, b: usize) {
        debug_assert!(a < self.total_len() && b < self.total_len());
        if a == b {
            return;
        }
        let c_a = self.cube_ptr(a);
        let c_b = self.cube_mut_ptr(b);
        unsafe {
            std::ptr::copy_nonoverlapping(c_a, c_b, self.stride as usize);
        }
    }
    /// c = -a (group inverse).
    ///
    /// Composes with the identity to produce the inverse. Equivalent to
    /// `sub(identity, a, c)`.
    #[inline]
    pub fn neg(&mut self, a: usize, c: usize) {
        debug_assert!(a < self.len && c < self.len);
        unsafe {
            let c_a = self.cube_ptr(a);
            let c_c = self.cube_mut_ptr(c);

            let mut block = sub_ori::<Corner>(247132686368, get_corner(*c_a));
            if self.n & 1 == 1 {
                block |= sub_ori::<Edge>(407901468851537952, get_edge(*c_a)) << 40;
                block |= sub_perm::<Center>(172066848, get_center(*c_a)) << 100;
            }
            *c_c = block;

            for a in 1..self.stride as usize {
                *c_c.add(a) =
                    sub_perm::<Piece24>(984818244535754528103549039458486304, *c_a.add(a));
            }
        }
    }
    /// c = n * a (scalar multiplication via repeated squaring).
    ///
    /// For `n == 0` sets `c` to the identity. For negative `n`, negates
    /// first then multiplies by `|n|`.
    #[inline]
    pub fn mul(&mut self, a: usize, n: isize, c: usize) {
        if n == 0 {
            self.identity(c);
            return;
        }
        self.clone_cube(a, c);
        if n == 1 {
            return;
        }
        if n < 0 {
            self.neg(c, c);
        }

        let acc = self.aux_mul();
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
        self.clone_cube(acc, c);
    }
    /// Sets cube `index` to the solved (identity) configuration.
    #[inline(always)]
    pub fn identity(&mut self, index: usize) {
        debug_assert!(index < self.total_len());
        let start = index * self.stride as usize;

        self.data[start] =
            247132686368 | (218120643595071921725640057825106329600 * (self.n as u128 & 1));

        if self.n > 3 {
            self.data[start + 1..start + self.stride as usize]
                .fill(984818244535754528103549039458486304);
        }
    }
    /// Pretty-prints the cube at `index` to stdout, showing each orbit
    /// group as (perm, ori) pairs.
    pub fn print_cube(&self, index: usize) {
        debug_assert!(index < self.len);
        let cube = self.get_cube(index);

        println!("Cube {}:", index);

        println!(
            "Corner: {:?}",
            unpack_u128::<Corner>(get_corner(cube[0]) as u128)
        );
        if self.n & 1 == 1 {
            println!("Edge: {:?}", unpack_u128::<Edge>(get_edge(cube[0]) as u128));
            println!(
                "Center: {:?}",
                unpack_u128::<Center>(get_center(cube[0]) as u128)
            );
        }
        let name = ["Par Center", "Par Edge", "Par Corner"];
        for (&(start, len), n) in self.orbit.iter().zip(name.iter()) {
            for (i, &val) in &mut cube[start as usize..(start + len) as usize]
                .iter()
                .enumerate()
            {
                println!("{} {}: {:?}", n, i, unpack_u128::<Piece24>(val));
            }
        }
    }
    /// Generates a uniformly random cube at `index` that is guaranteed
    /// solvable per Theorem 14 (even n) and Theorem 20 (odd n) of
    /// Bonzio, Loi, Peruzzi (2017, arXiv:1708.05598).
    ///
    /// Uses a two-pass approach: first generates all 24-piece orbits with
    /// per-orbit parity fixing (center-corners match corner parity,
    /// center-edges even for even n, coupled edges use paired 24-encoding),
    /// then in a second pass fixes coupled-edge parities for odd n to
    /// satisfy `sgn(tau_k) = sgn(sigma) XOR sgn(rho_ek)`.
    pub fn random_cube(&mut self, index: usize, rng: &mut impl rand::Rng) {
        debug_assert!(index < self.total_len());
        let mut _6: [(u8, u8); 6] = std::array::from_fn(|i| (i as u8, 0));
        let mut _8: [(u8, u8); 8] = std::array::from_fn(|i| (i as u8, 0));
        let mut _12: [(u8, u8); 12] = std::array::from_fn(|i| (i as u8, 0));
        let mut _24: [(u8, u8); 24] = std::array::from_fn(|i| (i as u8, 0));

        let stride = self.stride;
        let n = self.n as usize;
        let n_mod_2 = n & 1;
        let k_max = (n - 2 - n_mod_2) >> 1; // K
        let ew_base: usize = 1 + k_max * k_max;
        let cew_base: usize = ew_base + k_max;
        let cube = self.cube_mut_ptr(index);

        /// Parity of a (u8,u8) slice (cheap, no packing needed).
        fn slice_parity(slice: &[(u8, u8)]) -> bool {
            let mut visited: u64 = 0;
            let mut cycles = 0usize;
            let len = slice.len();
            for i in 0..len {
                if (visited >> i) & 1 == 0 {
                    cycles += 1;
                    let mut j = i;
                    loop {
                        if (visited >> j) & 1 != 0 {
                            break;
                        }
                        visited |= 1u64 << j;
                        j = slice[j].0 as usize;
                    }
                }
            }
            ((len - cycles) & 1) != 0
        }

        /// Swap positions 0<->1 in slice (toggles parity).
        fn fix_parity(slice: &mut [(u8, u8)], target_odd: bool) {
            if slice_parity(slice) != target_odd {
                slice.swap(0, 1);
            }
        }

        // ---- corners --------------------------------------------------------
        shuffle(&mut _8, rng);
        // orientation sum ≡ 0 mod 3  (Theorem 14/20 cond. 2/3)
        let mut ori_sum: usize = 0;
        for i in 0..7 {
            let ori = rng.random_range(0..3);
            _8[i].1 = ori;
            ori_sum += ori as usize;
        }
        _8[7].1 = ((3 - (ori_sum % 3)) % 3) as u8;

        let mut block = pack_u128::<Corner>(&_8);
        let sigma_odd = parity::<Corner>(block);

        // ---- single edges (odd n only) -------------------------------------
        if n & 1 == 1 {
            shuffle(&mut _12, rng);
            // sgn(σ) = sgn(τ)  (Theorem 20  cond. 1)
            fix_parity(&mut _12, sigma_odd);
            // orientation sum ≡ 0 mod 2  (Theorem 20  cond. 4)
            ori_sum = 0;
            for i in 0..11 {
                let ori = rng.random::<bool>() as u8;
                _12[i].1 = ori;
                ori_sum ^= ori as usize;
            }
            _12[11].1 = ori_sum as u8;

            block |= pack_u128::<Edge>(&_12) << 40;
            block |= pack_u128::<Center>(&_6) << 100; // fixed centres = identity
        }

        unsafe {
            *cube = block;

            // ---- 24‑piece orbits (first pass: generate & fix centre-corners) -
            for i in 1..stride as usize {
                let is_ew = i >= ew_base && i < cew_base; // coupled-edge wing

                if is_ew {
                    // ---- coupled-edge wing: generate with pair constraint ----
                    // Pieces come in 12 consecutive pairs (2j,2j+1).
                    // Condition 3/5 requires perm[i^1] == perm[i] ^ 1.
                    // Generate a random permutation of the 12 pairs, each
                    // optionally flipped.
                    let mut pairs: [usize; 12] = std::array::from_fn(|j| j);
                    for pi in (1..12).rev() {
                        pairs.swap(pi, rng.random_range(0..=pi));
                    }
                    for dst in 0..12 {
                        let src = pairs[dst];
                        if rng.random::<bool>() {
                            // flipped: low sticker goes to high slot, high to low
                            _24[2 * dst] = (2 * src as u8 + 1, 0);
                            _24[2 * dst + 1] = (2 * src as u8, 0);
                        } else {
                            _24[2 * dst] = (2 * src as u8, 0);
                            _24[2 * dst + 1] = (2 * src as u8 + 1, 0);
                        }
                    }
                } else {
                    // ---- centre / centre-edge / centre-edge-wing: standard ----
                    for j in 0..24 {
                        _24[j] = (j as u8, 0);
                    }
                    shuffle(&mut _24, rng);

                    let is_cc = i < (1 + k_max * k_max)     // inside orbit_slice_3
                             && ((i - 1) % (k_max + 1)) == 0; // x == z → centre-corner

                    if is_cc {
                        // Theorem 14/20  cond. 1: sgn(σ) = sgn(ρck)
                        fix_parity(&mut _24, sigma_odd);
                    } else if n & 1 == 0 && i < (1 + k_max * k_max) {
                        // Even n, orbit_slice_3 x≠z group  = centre-edge.
                        // Theorem 14  cond. 4: sgn(ρek) = +1  (k ≥ 2; k=1 has
                        // no centre-edge groups, but the loop body is harmless)
                        fix_parity(&mut _24, false);
                    }
                    // Odd n: centre-edge & centre-edge-wing groups have *no*
                    // standalone parity constraint — only the combined ρek sign
                    // matters (handled below via τk).
                }

                *cube.add(i) = pack_u128::<Piece24>(&_24);
            }

            // ---- second pass: fix coupled‑edge parities (odd n only) --------
            if n & 1 == 1 {
                let cc_base: usize = 1;

                // Pre‑compute sgn(ρek) for every k :
                //   ρek = XOR over orbit[3] x≠z groups for k
                //         XOR orbit[5] group k−1
                let mut rho_ek_odd = vec![false; k_max]; // rho_ek_odd[k-1]
                for k in 1..=k_max {
                    let mut odd = false;
                    // orbit_slice_3 centre-edge groups (empty when k = 1)
                    for x in 1..k {
                        let idx_a = (k_max - k) * k_max + (k_max - x);
                        odd ^= parity::<Piece24>(*cube.add(cc_base + idx_a));
                        let idx_b = (k_max - x) * k_max + (k_max - k);
                        odd ^= parity::<Piece24>(*cube.add(cc_base + idx_b));
                    }
                    // orbit_slice_5 centre-edge-wing group
                    odd ^= parity::<Piece24>(*cube.add(cew_base + k - 1));
                    rho_ek_odd[k - 1] = odd;
                }

                // Fix τk: sgn(τk) = sgn(σ) · sgn(ρek)
                for k in 1..=k_max {
                    let target = sigma_odd ^ rho_ek_odd[k - 1];
                    let idx = ew_base + k_max - k; // τk in orbit_slice_4
                    let mut arr = unpack_u128::<Piece24>(*cube.add(idx));
                    fix_parity(&mut arr, target);
                    *cube.add(idx) = pack_u128::<Piece24>(&arr);
                }
            }
        }
    }
    /// Returns the cycle decomposition of every orbit group in the cube
    /// at `index`.
    ///
    /// Returns `Vec<Vec<Vec<usize>>>`: outer vec = orbit groups, middle
    /// vec = cycles, inner vec = elements in each cycle.
    pub fn cycle_decomposition_cube(&self, index: usize) -> Vec<Vec<Vec<usize>>> {
        debug_assert!(index < self.total_len());
        let mut vect = Vec::with_capacity((self.stride as u8 + 2 * (self.n & 1)) as usize);
        let cube = self.get_cube(index);
        vect.push(cycle_decomp::<Corner>(get_corner(cube[0]) as u128));

        if self.n & 1 == 1 {
            vect.push(cycle_decomp::<Edge>(get_edge(cube[0]) as u128));
            vect.push(cycle_decomp::<Center>(get_center(cube[0]) as u128));
        }
        for i in 1..self.stride as usize {
            vect.push(cycle_decomp::<Piece24>(cube[i]));
        }
        vect
    }
    /// Validate that every 5-bit block in the cube slice contains
    /// valid piece indices (no out-of-range values) and that no
    /// permutation repeats within each orbit group.
    pub fn check_slice(&self, cube: &[u128]) -> Result<()> {
        // Helper: verify orbit has unique perms in [0, max_piece)
        fn check_orbit<const MAX: u8>(
            data: u128,
            count: usize,
            shift: u32,
            ori_mask: u8,
            ori_mod: u8,
        ) -> Result<()> {
            let mut seen = 0u32;
            for i in 0..count {
                let block = ((data >> (i * 5)) & 31) as u8;
                let perm = block & ((1 << shift) - 1);
                let _ori = (block >> shift) & ori_mask;
                if perm >= MAX {
                    return Err(CubeError::InvalidPermutation {
                        got: unpack_u128::<Piece24>(data),
                    });
                }
                if _ori >= ori_mod && ori_mod > 0 {
                    return Err(CubeError::InvalidOrientation {
                        got: vec![(perm, _ori)],
                        mod_: ori_mod as usize,
                    });
                }
                if (seen >> perm) & 1 == 1 {
                    return Err(CubeError::InvalidPermutation {
                        got: unpack_u128::<Piece24>(data),
                    });
                }
                seen |= 1 << perm;
            }
            Ok(())
        }

        // Check corners: 8 pieces, perm 0..7, ori 0..2 at bit 3
        {
            let corners = get_corner(cube[0]);
            check_orbit::<8>(corners as u128, 8, 3, 3, 3)?;
        }

        if self.n & 1 == 1 {
            // Edges: 12 pieces, perm 0..11, ori 0..1 at bit 4
            let edges = get_edge(cube[0]);
            check_orbit::<12>(edges as u128, 12, 4, 1, 2)?;

            // Centers: 6 pieces, value 0..5, no orientation (use shift=5 → full 5-bit)
            let centers = get_center(cube[0]);
            check_orbit::<6>(centers, 6, 5, 0, 0)?;
        }

        // Check 24-piece orbits: 24 pieces, value 0..23
        for i in 1..self.stride as usize {
            check_orbit::<24>(cube[i], 24, 5, 0, 0)?;
        }

        orientation_check::<Corner>(get_corner(cube[0]) as u128)?;
        orientation_check::<Edge>(get_edge(cube[0]) as u128)?;
        Ok(())
    }
    /// Attempts to normalize the cube orientation by reading the corner
    /// position (even n) or fixed-center permutation (odd n) to determine
    /// which whole-cube rotation was applied, then applying the inverse.
    /// Currently a skeleton.
    pub fn normalize_cube(&mut self, index: usize) {
        debug_assert!(index < self.total_len());
        if self.n & 1 != 1 {
            let cube = self.get_cube(index);
            let corner = get_corner(cube[0]);
            // solve CORNER 0
            let _mv = match corner & 0b111 {
                0 => ZERO,
                1 => Y,
                2 => Y * Y,
                3 => Y * Y * Y,
                4 => X,
                5 => X * X,
                6 => Y * X * X,
                7 => Z * Z,
                _ => unreachable!("invalid corner identity"),
            };
            // solve ORIENTATION
            let _mv = match (corner >> 3) & 0b11 {
                0 => ZERO,
                1 => X * Y,
                2 => Z * Z * Z * Y * Y * Y,
                _ => unreachable!("invalid corner identity"),
            };
        } else {
            let cube = self.get_cube(index);
            let center = get_center(cube[0]);
            // solve D face
            let _mv = match center & 0b111 {
                0 => ZERO,
                1 => X,
                2 => Z * Z * Z,
                3 => X * X * X,
                4 => Z,
                5 => Y * Y,
                _ => unreachable!("invalid center identity"),
            };
            // solve B face
            let _mv = match center >> 5 & 0b111 {
                // 0 => {ZERO}, UNREACHABLE
                1 => ZERO,
                2 => Y * Y * Y,
                3 => Y * Y,
                4 => Y,
                // 5 => {Y*Y}, UNREACHABLE
                _ => unreachable!("invalid center identity"),
            };
        }
    }
    /// Checks whether an arbitrary packed cube configuration is solvable
    /// (reachable from the solved state via face moves).
    ///
    /// Implements Theorem 14 for even n and Theorem 20 for odd n.
    /// Returns `Ok(())` if solvable, otherwise
    /// `Err(CubeError::InvalidPermutation)` or
    /// `Err(CubeError::InvalidOrientation)`. See file-level docs for the
    /// packed layout assumed.
    pub fn is_solvable_slice(&self, cube: &[u128]) -> Result<()> {
        let n = self.n as usize;

        // K — number of concentric circles (= tmp in new_arena)
        let k_max = (n - 2 - (n & 1)) >> 1;

        // Base indices into cube[] for each orbit family
        let cc_base: usize = 1; // center-corners  (orbit_slice_3)
        let ew_base: usize = 1 + k_max * k_max; // coupled-edges   (orbit_slice_4)
        let cew_base: usize = ew_base + k_max; // center-edge-wings (orbit_slice_5, odd n)

        // sgn(σ) — corner permutation parity
        let sigma_odd = parity::<Corner>(get_corner(cube[0]) as u128);

        if self.n & 1 == 0 {
            // ── Theorem 14  (even n) ──────────────────────────────────────────

            // Condition 1: sgn(σ) = sgn(ρck)  for every k = 1..K
            // ρck are the centre-corner groups: orbit_slice_3 groups with x=z,
            // which sit at indices (K−k)·(K+1) ≡ 0 mod (K+1).
            for k in 1..=k_max {
                let idx_cc = (k_max - k) * (k_max + 1);
                if parity::<Piece24>(cube[cc_base + idx_cc]) != sigma_odd {
                    return Err(CubeError::InvalidPermutation {
                        got: unpack_u128::<Piece24>(cube[cc_base + idx_cc]),
                    });
                }
            }

            // Condition 2: Σ corner orientations ≡ 0 (mod 3)
            if orientation_check::<Corner>(get_corner(cube[0]) as u128).is_err() {
                return Err(CubeError::InvalidOrientation {
                    got: unpack_u128::<Corner>(get_corner(cube[0]) as u128),
                    mod_: 3,
                });
            }

            // Condition 3: yk orientation — coupled-edge pairs must stay
            // assembled: for every pair of home positions (i,i^1), the
            // two pieces occupying them must also form a consecutive
            // pair (j,j^1).  In an arbitrary assembly this can be
            // violated by swapping individual stickers.
            for &orbit in cube[ew_base..cew_base].iter() {
                if !edge_check::<Piece24>(orbit) {
                    return Err(CubeError::InvalidOrientation {
                        got: unpack_u128::<Piece24>(orbit),
                        mod_: 1,
                    });
                }
            }
            // Condition 4: sgn(ρek) = +1  for every k = 2..K
            //   ρek is block-diagonal; sign = XOR of each sub-group's parity
            //   Sub-groups are orbit_slice_3 groups with x ≠ z and max(x,z) = k
            for k in 2..=k_max {
                let mut rho_ek_odd = false;
                for x in 1..k {
                    // group with (x, z = k)
                    let idx_a = (k_max - k) * k_max + (k_max - x);
                    rho_ek_odd ^= parity::<Piece24>(cube[cc_base + idx_a]);
                    // group with (x = k, z)
                    let idx_b = (k_max - x) * k_max + (k_max - k);
                    rho_ek_odd ^= parity::<Piece24>(cube[cc_base + idx_b]);
                }
                if rho_ek_odd {
                    return Err(CubeError::InvalidPermutation {
                        got: unpack_u128::<Piece24>(cube[cc_base]),
                    });
                }
            }
        } else {
            // ── Theorem 20  (odd n) ───────────────────────────────────────────

            // sgn(τ) — single-edge permutation parity
            let tau_odd = parity::<Edge>(get_edge(cube[0]) as u128);

            // Condition 1a: sgn(σ) = sgn(τ)
            if sigma_odd != tau_odd {
                return Err(CubeError::InvalidPermutation {
                    got: unpack_u128::<Edge>(get_edge(cube[0]) as u128),
                });
            }

            // Condition 1b: sgn(σ) = sgn(ρck)  for every k = 1..K
            // ρck are the centre-corner groups: orbit_slice_3 groups with x=z,
            // which sit at indices (K−k)·(K+1) ≡ 0 mod (K+1).
            for k in 1..=k_max {
                let idx_cc = (k_max - k) * (k_max + 1);
                if parity::<Piece24>(cube[cc_base + idx_cc]) != sigma_odd {
                    return Err(CubeError::InvalidPermutation {
                        got: unpack_u128::<Piece24>(cube[cc_base + idx_cc]),
                    });
                }
            }

            let center_odd = parity::<Center>(get_center(cube[0]));
            // Condition 2: sgn(τk) = sgn(σ) · sgn(ρek)  for every k = 1..K
            //   Boolean equivalent: odd(τk) = σ_odd XOR odd(ρek)
            //   ρek = (orbit_slice_3 x≠z groups for circle k) ⊕ (orbit_slice_5 group k−1)
            if center_odd {
                return Err(CubeError::InvalidPermutation {
                    got: unpack_u128::<Center>(get_center(cube[0])),
                });
            }
            for k in 1..=k_max {
                // sgn(τk): coupled-edge group for circle k
                let tau_k_odd = parity::<Piece24>(cube[ew_base + k_max - k]);

                // sgn(ρek): combined parity over all center-edge groups for circle k
                let mut rho_ek_odd = false;
                // orbit_slice_3 center-edge groups (empty loop when k = 1)
                for x in 1..k {
                    let idx_a = (k_max - k) * k_max + (k_max - x);
                    rho_ek_odd ^= parity::<Piece24>(cube[cc_base + idx_a]);
                    let idx_b = (k_max - x) * k_max + (k_max - k);
                    rho_ek_odd ^= parity::<Piece24>(cube[cc_base + idx_b]);
                }
                // orbit_slice_5 center-edge-wing group for this circle
                rho_ek_odd ^= parity::<Piece24>(cube[cew_base + k - 1]);
                if tau_k_odd != (sigma_odd ^ rho_ek_odd) {
                    return Err(CubeError::InvalidPermutation {
                        got: unpack_u128::<Piece24>(cube[ew_base + k_max - k]),
                    });
                }
            }

            // Condition 3: Σ corner orientations ≡ 0 (mod 3)
            if orientation_check::<Corner>(get_corner(cube[0]) as u128).is_err() {
                return Err(CubeError::InvalidOrientation {
                    got: unpack_u128::<Corner>(get_corner(cube[0]) as u128),
                    mod_: 3,
                });
            }

            // Condition 4: Σ single-edge orientations ≡ 0 (mod 2)
            if orientation_check::<Edge>(get_edge(cube[0]) as u128).is_err() {
                return Err(CubeError::InvalidOrientation {
                    got: unpack_u128::<Edge>(get_edge(cube[0]) as u128),
                    mod_: 2,
                });
            }

            // Condition 5: yk orientation — coupled-edge pairs must stay
            // assembled (same as Condition 3 for even n).
            for &orbit in cube[ew_base..cew_base].iter() {
                if !edge_check::<Piece24>(orbit) {
                    return Err(CubeError::InvalidOrientation {
                        got: unpack_u128::<Piece24>(orbit),
                        mod_: 1,
                    });
                }
            }
        }

        Ok(())
    }
    /// Returns true if cubes `a` and `b` have identical packed
    /// representations.
    pub fn eq(&self, a: usize, b: usize) -> bool {
        debug_assert!(a < self.total_len() && b < self.total_len());
        self.get_cube(a) == self.get_cube(b)
    }
    /// Convenience wrapper: calls
    /// [`is_solvable_slice`](Self::is_solvable_slice) on the cube at
    /// `index`.
    pub fn is_solvable(&self, index: usize) -> Result<()> {
        debug_assert!(index < self.total_len());
        self.is_solvable_slice(self.get_cube(index))
    }

    pub fn check_cube(&mut self, index: usize) -> Result<()> {
        debug_assert!(index < self.total_len());
        let cube = self.get_cube(index);
        match self.check_slice(cube) {
            Ok(_) => Ok(()),
            Err(e) => {
                self.print_cube(index);
                Err(e)
            }
        }
    }
    /// Number of u128 words per cube.
    #[inline]
    pub fn stride(&self) -> u16 {
        self.stride
    }
    /// Number of user cubes (not counting scratch slots).
    #[inline]
    pub fn len(&self) -> usize {
        self.len
    }
    /// Total number of cube slots including 2 scratch slots.
    /// Equal to `len + 2`.
    #[inline]
    pub fn total_len(&self) -> usize {
        self.len + 2
    }
    /// Returns index `len + 1`, the second scratch slot used by `mul`.
    #[inline]
    pub fn aux_mul(&self) -> usize {
        self.len + 1
    }
    /// Cube dimension.
    #[inline]
    pub fn n(&self) -> u8 {
        self.n
    }
    /// Computes the order (period) of the group element at `index`:
    /// the smallest positive integer k such that the cube composed with
    /// itself k times returns to identity.
    #[inline]
    pub fn cube_order(&self, index: usize) -> usize {
        debug_assert!(index < self.total_len());
        let cube = self.get_cube(index);
        let mut order = 1;
        order = mcm(order, orbit_order::<Corner>(get_corner(cube[0]) as u128));
        if self.n & 1 == 1 {
            order = mcm(order, orbit_order::<Edge>(get_edge(cube[0]) as u128));
            order = mcm(order, orbit_order::<Center>(get_center(cube[0]) as u128));
        }
        for i in 1..self.stride as usize {
            order = mcm(order, orbit_order::<Piece24>(cube[i]));
        }
        order
    }
    /// c = b^(-1) * a * b  (group conjugation).
    /// Computes c = -b + a + b.
    #[inline]
    pub fn conjugate(&mut self, a: usize, b: usize, c: usize) {
        debug_assert!(a < self.total_len() || b < self.total_len() || c < self.total_len());
        self.neg(a, c);
        self.add(c, b, c);
        self.add(c, a, c);
    }
    /// c = a * b * a^(-1) * b^(-1)  (group commutator).
    /// Uses a scratch slot to avoid aliasing.
    #[inline]
    pub fn conmutator(&mut self, a: usize, b: usize, c: usize) {
        debug_assert!(a < self.total_len() || b < self.total_len() || c < self.total_len());
        let tmp = self.len;

        self.clone_cube(a, tmp);
        self.add(tmp, b, tmp);
        self.neg(a, c);
        self.add(tmp, c, tmp);
        self.neg(b, c);
        self.add(tmp, c, c);
    }
}

/// Unpacks a u128 word into `P::LEN` (perm, ori) pairs. Each pair
/// occupies 5 bits; `P::SHIFT` determines the bit offset of the
/// orientation sub-field.
#[inline(always)]
pub fn unpack_u128<P: PieceKind>(value: u128) -> Vec<(u8, u8)> {
    let mut vect: Vec<(u8, u8)> = Vec::with_capacity(P::LEN);

    for i in 0..P::LEN {
        let block = (value >> (i * 5)) & 31;
        let p = (block & P::PERM) as u8;
        let o = ((block >> P::SHIFT) & P::ORI) as u8;

        unsafe {
            ptr::write(vect.as_mut_ptr().add(i), (p, o));
        }
    }
    unsafe {
        vect.set_len(P::LEN);
    }
    vect
}
/// Packs a slice of (perm, ori) pairs into a single u128.
/// Inverse of [`unpack_u128`].
#[inline(always)]
pub fn pack_u128<P: PieceKind>(slice: &[(u8, u8)]) -> u128 {
    debug_assert!(slice.len() == P::LEN);
    let mut value = 0u128;
    unsafe {
        let slice_ptr = slice.as_ptr();
        for i in 0..P::LEN {
            let (p, o) = *slice_ptr.add(i);
            let block = (p as u128) | ((o as u128) << P::SHIFT);
            value |= block << (i * 5);
        }
    }
    value
}
/// Extracts the corner block (lower 40 bits) from cube[0].
#[inline(always)]
fn get_corner(a: u128) -> u64 {
    (a & ((1 << 40) - 1)) as u64
}
/// Extracts the single-edge block (bits 40-99) from cube[0].
#[inline(always)]
fn get_edge(a: u128) -> u64 {
    ((a >> 40) & ((1 << 60) - 1)) as u64
}
/// Extracts the fixed-center block (bits 100+) from cube[0].
#[inline(always)]
fn get_center(a: u128) -> u128 {
    a >> 100
}
/// Orientation-aware composition of two packed piece arrays.
/// For each piece, composes permutations and adds orientations modulo
/// `P::MOD`. Used by `add`/`add_slice`.
#[inline(always)]
pub fn add_ori<P: PieceKind>(a: u64, b: u64) -> u128 {
    let mut out = 0;
    for shift in (0..P::LEN * 5).step_by(5) {
        let b_block = (b >> shift) & 31;
        let b_perm = (b_block & (P::PERM as u64)) * 5;
        let b_ori = b_block >> P::SHIFT;

        let a_block = (a >> b_perm) & 31;
        let a_perm = a_block & (P::PERM as u64);
        let a_ori = a_block >> P::SHIFT;

        let new_ori = ((b_ori + a_ori) % (P::MOD as u64)) << P::SHIFT;
        out |= (a_perm | new_ori) << shift;
    }

    out as u128
}
/// Permutation composition for a packed 24-piece orbit.
/// Uses the 15-bit LUT for fast 3-piece composition.
/// Used by `add`/`add_slice`.
// #[unsafe(no_mangle)]
#[inline(always)]
pub fn add_perm<P: PieceKind>(a: u128, mut b: u128) -> u128 {
    let mut p = 0;
    for i in (0..P::LEN).step_by(3) {
        let idx = (b & ((1 << (15)) - 1)) as usize;
        unsafe {
            let v = &LUT.get_unchecked(idx);
            let p1 = ((a >> (*v.get_unchecked(0) as u32 * 5)) & 31) << (i * 5);
            let p2 = ((a >> (*v.get_unchecked(1) as u32 * 5)) & 31) << ((i + 1) * 5);
            let p3 = ((a >> (*v.get_unchecked(2) as u32 * 5)) & 31) << ((i + 2) * 5);

            b >>= 15;
            p |= p1 | p2 | p3;
        }
    }
    p
}
/// Inverse permutation composition. Inverse of [`add_perm`].
#[inline(always)]
// #[unsafe(no_mangle)]
pub fn sub_perm<P: PieceKind>(mut a: u128, mut b: u128) -> u128 {
    let mask = (1 << (15)) - 1;

    let mut p = 0;

    for _ in (0..P::LEN).step_by(3) {
        let idx = (b & mask) as usize;
        let val = (a & mask) as usize;
        unsafe {
            let v = &LUT.get_unchecked(idx);
            let w = &LUT.get_unchecked(val);
            let p1 = (*w.get_unchecked(0) as u128) << (*v.get_unchecked(0) as u128 * 5);
            let p2 = (*w.get_unchecked(1) as u128) << (*v.get_unchecked(1) as u128 * 5);
            let p3 = (*w.get_unchecked(2) as u128) << (*v.get_unchecked(2) as u128 * 5);
            b >>= 15;
            a >>= 15;
            p |= p1 | p2 | p3;
        }
    }
    p
}
/// Inverse orientation-aware composition. Inverse of [`add_ori`].
#[inline(always)]
pub fn sub_ori<P: PieceKind>(a: u64, b: u64) -> u128 {
    let mut out = 0;
    for shift in (0..P::LEN * 5).step_by(5) {
        let b_block = (b >> shift) & 31;
        let b_perm = (b_block & (P::PERM as u64)) * 5;
        let b_ori = b_block >> P::SHIFT;

        let a_block = (a >> shift) & 31;
        let a_perm = a_block & (P::PERM as u64);
        let a_ori = a_block >> P::SHIFT;

        let new_ori = ((a_ori + (P::MOD as u64) - b_ori) % (P::MOD as u64)) << P::SHIFT;

        out |= (a_perm | new_ori) << b_perm;
    }
    out as u128
}
/// Decomposes a packed permutation into its cycles.
/// Returns a vec of cycles, each cycle is a vec of position indices.
#[inline]
pub fn cycle_decomp<P: PieceKind>(value: u128) -> Vec<Vec<usize>> {
    let mut visited: u64 = 0;
    let mut out: Vec<Vec<usize>> = Vec::with_capacity(P::LEN >> 1);
    let mut cycle: Vec<usize> = Vec::with_capacity(P::LEN >> 1);
    for start in 0..P::LEN {
        if ((visited >> start) & 1) == 1 {
            continue;
        }

        cycle.clear();
        let mut j = start;
        loop {
            if ((visited >> j) & 1) == 1 {
                break;
            }

            visited |= 1 << j;
            cycle.push(j);

            j = ((value >> (j * 5)) & P::PERM) as usize;
        }
        if cycle.len() > 1 {
            out.push(std::mem::replace(
                &mut cycle,
                Vec::with_capacity(P::LEN >> 1),
            ));
        }
    }
    out
}
/// Computes the order of a packed permutation including orientation.
/// Returns the smallest k > 0 such that applying the permutation k
/// times returns to identity (with all orientations zero).
#[inline]
pub fn orbit_order<P: PieceKind>(value: u128) -> usize {
    let mut visited: u64 = 0;
    let mut order = 1;
    for i in 0..P::LEN {
        if (visited & (1 << i)) == 0 {
            let mut j = i;
            let mut tmp = 0;
            loop {
                if (visited & (1 << j)) != 0 {
                    order = mcm(order, tmp);
                    break;
                }
                tmp += 1;
                visited |= 1 << j;
                let block = value >> (j * 5);
                j = (block & P::PERM) as usize;
                let block = (block >> P::SHIFT) & P::ORI;
                visited |= (1u64 << 63) * ((block | block.wrapping_neg()) >> 127) as u64;
            }
        }
    }
    if visited >> 63 & 1 == 1 {
        order *= P::MOD as usize;
    }
    order
}
/// Returns true if the packed permutation is odd (sign -1).
/// Counts cycles and returns `(LEN - cycles) % 2 != 0`.
#[inline]
pub fn parity<P: PieceKind>(value: u128) -> bool {
    let mut visited: u64 = 0;
    let mut cycles = 0;

    for i in 0..P::LEN {
        if ((visited >> i) & 1) == 0 {
            cycles += 1;

            let mut j = i;

            loop {
                if ((visited >> j) & 1) != 0 {
                    break;
                }

                visited |= 1 << j;
                j = ((value >> (j * 5)) & P::PERM) as usize;
            }
        }
    }

    ((P::LEN - cycles) & 1) != 0
}
/// Verifies that the sum of all orientation values modulo `P::MOD`
/// is zero. Returns `Err(CubeError::InvalidOrientation)` on failure.
#[inline]
pub fn orientation_check<P: PieceKind>(value: u128) -> Result<()> {
    if (0..P::LEN as u128)
        .map(|i| {
            let o = (value >> (i * 5 + P::SHIFT as u128)) & P::ORI;
            o
        })
        .sum::<u128>()
        % P::MOD
        != 0
    {
        return Err(CubeError::InvalidOrientation {
            got: unpack_u128::<P>(value),
            mod_: P::MOD as usize,
        });
    }
    Ok(())
}
/// Verifies the coupled-edge pair constraint (Theorem 14 cond. 3 /
/// Theorem 20 cond. 5): for every adjacent pair of positions (i, i^1),
/// the pieces occupying them must also form a consecutive pair (j, j^1).
/// Returns false if any pair is "disassembled" (individual stickers
/// swapped).
#[inline]
pub fn edge_check<P: PieceKind>(value: u128) -> bool {
    let perm = (0..P::LEN)
        .map(|i| ((value >> (i * 5)) & P::PERM) as u8)
        .collect::<Vec<_>>();

    for i in 0..P::LEN {
        if perm[i ^ 1] != (perm[i] ^ 1) {
            return false;
        }
    }
    true
}
/// Binary GCD algorithm.
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
/// Least common multiple via `(u * v) / gcd(u, v)`.
#[inline]
pub fn mcm(u: usize, v: usize) -> usize {
    (u * v) / gcd(u, v)
}
/// Fisher-Yates shuffle of a (perm, ori) slice.
/// Returns the parity of the induced permutation (true if odd).
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
