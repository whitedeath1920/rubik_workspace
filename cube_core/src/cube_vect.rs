use std::fmt::Display;

use crate::{
    CubeArena, CubeError,
    cube_moves::{LayerSpec, Move, MoveKind},
    error::Result,
};

const MAP_PERM_VECT: [fn(u8, usize, &mut [Point3]); 6] = [
    orbit_slice_0,
    orbit_slice_1,
    orbit_slice_2,
    orbit_slice_3,
    orbit_slice_4,
    orbit_slice_5,
];

#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Point3 {
    pub x: i8,
    pub y: i8,
    pub z: i8,
}
impl Point3 {
    pub fn new(x: i8, y: i8, z: i8) -> Self {
        Self { x, y, z }
    }
    #[inline]
    pub fn hash(&self, n: u8) -> usize {
        let x = (self.x as i16 + n as i16) as usize;
        let y = (self.y as i16 + n as i16) as usize;
        let z = (self.z as i16 + n as i16) as usize;
        x + (y << 8) + (z << 16)
    }
    pub fn get(&self, index: usize) -> i8 {
        match index {
            0 => self.x,
            1 => self.y,
            2 => self.z,
            _ => unreachable!(),
        }
    }
    pub fn set(&mut self, index: usize, value: i8) {
        match index {
            0 => self.x = value,
            1 => self.y = value,
            2 => self.z = value,
            _ => unreachable!(),
        }
    }
}
impl Display for Point3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CubeVect {
    /// Position data of cubies
    data: Vec<Point3>,
    /// Orientation data of corners and edges
    ori: Vec<Point3>,
    /// (start, length, number of groups), of the cubies orbits
    orbit: [(usize, usize, usize); 6],
    /// Dimensions of the cube.
    n: u8,
}
impl CubeVect {
    pub fn data(&self) -> &[Point3] {
        &self.data
    }
    pub fn new(n: usize) -> Self {
        assert!(n < 128, "CubeVect dimension must be < 128, got {n}");
        let n_mod_2 = n & 1;
        let cubies = n.pow(3) - (n - 2).pow(3);
        let mut data: Vec<Point3> = Vec::with_capacity(cubies as usize);
        let mut ori = Vec::with_capacity((8 + 12 * n_mod_2) as usize);
        unsafe {
            data.set_len(cubies as usize);
            ori.set_len((8 + 12 * n_mod_2) as usize);
        }
        let tmp = (n - 2 - n_mod_2) / 2;
        let orbits: [(usize, usize); 6] = [
            (1, 8),
            (n_mod_2, 12),
            (n_mod_2, 6),
            (tmp.pow(2), 24),
            (tmp, 24),
            (tmp * n_mod_2, 24),
        ];
        let mut orbit = [(0, 0, 0); 6];
        let mut start = 0;
        for (i, &(g, len)) in orbits.iter().enumerate() {
            orbit[i] = (start as usize, len as usize, g as usize);
            for index in 0..g {
                MAP_PERM_VECT[i](n as u8, index, &mut data[start..start + len]);
                start += len;
            }
        }
        let caras = [
            Point3::new(0, 1, 0),
            Point3::new(0, 0, 1),
            Point3::new(1, 0, 0),
            Point3::new(0, -1, 0),
            Point3::new(0, 0, -1),
            Point3::new(-1, 0, 0),
        ];
        for i in 0..4 {
            ori[i] = caras[3];
            ori[i + 4] = caras[0];
        }
        if n & 1 == 1 {
            for i in 8..12 {
                ori[i] = caras[3];
                ori[i + 8] = caras[0];
            }
            ori[4 + 8] = caras[5];
            ori[5 + 8] = caras[5];
            ori[6 + 8] = caras[2];
            ori[7 + 8] = caras[2];
        }

        Self {
            data,
            ori,
            orbit,
            n: n as u8,
        }
    }
    pub fn ori(&self) -> &[Point3] {
        &self.ori
    }
    /// Validate that `_slice` does not return duplicate indices.
    /// Catches cases where the layer-height calculation yields the same
    /// coordinate for two different layers (e.g. middle-layer skip bug
    /// on odd dimensions), which would apply the rotation twice to the
    /// same piece while skipping another entirely.
    fn _slice(&self, axis: usize, layer: i32, rotation: bool) -> Vec<usize> {
        let mut slice = Vec::new();
        let cara = [(1, 1), (1, 2), (1, 0), (-1, 1), (-1, 2), (-1, 0)];
        let mut altura = (self.n >> 1) as i32 - layer;
        if !rotation && altura <= 0 && self.n & 1 == 1 {
            altura -= 1;
        }
        altura *= cara[axis].0;
        for (i, piece) in self.data.iter().enumerate() {
            if piece.get(cara[axis].1) as i32 == altura {
                // Guard: the same piece must not appear in multiple layers
                // (unique per call to _slice, not globally).
                debug_assert!(
                    slice.last().map_or(true, |&last| last != i),
                    "_slice: duplicate piece {} at axis={} layer={} altura={}",
                    i,
                    axis,
                    layer,
                    altura
                );
                slice.push(i);
            }
        }
        slice
    }
    fn _mv(&mut self, mv: usize, layer: i32, rotation: bool) {
        let matrix = [
            [[0, 0, -1], [0, 1, 0], [1, 0, 0]],
            [[0, 1, 0], [-1, 0, 0], [0, 0, 1]],
            [[1, 0, 0], [0, 0, 1], [0, -1, 0]],
            [[0, 0, 1], [0, 1, 0], [-1, 0, 0]],
            [[0, -1, 0], [1, 0, 0], [0, 0, 1]],
            [[1, 0, 0], [0, 0, -1], [0, 1, 0]],
        ];
        let tmp1 = self.ori.clone();

        for piece in self._slice(mv, layer, rotation) {
            // println!("piece {piece}");
            let tmp = self.data[piece];
            for (i, values) in matrix[mv].iter().enumerate() {
                self.data[piece].set(
                    i,
                    tmp.get(0) * values[0] + tmp.get(1) * values[1] + tmp.get(2) * values[2],
                );
                if piece < self.ori.len() {
                    self.ori[piece].set(
                        i,
                        tmp1[piece].get(0) * values[0]
                            + tmp1[piece].get(1) * values[1]
                            + tmp1[piece].get(2) * values[2],
                    );
                }
            }
        }
    }

    /// Validate the CubeVect invariants:
    /// - No two pieces share the same 3D position
    /// - All coordinates are within valid bounds
    /// - The total number of cubies matches expectations
    pub fn check(&self) -> Result<()> {
        let half = (self.n >> 1) as i8;
        let n = self.n;

        // Expected cubie count for an N×N×N cube
        let expected = (n as usize).pow(3) - (n as usize - 2).max(0).pow(3);
        if self.data.len() != expected {
            return Err(CubeError::InvalidLength {
                expected,
                got: self.data.len(),
            });
        }

        // Check orbit boundaries and position uniqueness
        let mut seen: std::collections::HashMap<usize, usize> = std::collections::HashMap::new();
        for (i, &p) in self.data.iter().enumerate() {
            // Bounds check
            if p.x.abs() > half || p.y.abs() > half || p.z.abs() > half {
                return Err(CubeError::InvalidVectorDimension { i, p, half });
            }
            // Uniqueness check — this is the critical invariant:
            // after any move, every piece must have a unique 3D position.
            let hash = p.hash(n);
            if let Some(&other) = seen.get(&hash) {
                let other_p = self.data[other];
                return Err(CubeError::DuplicateVector {
                    i,
                    p,
                    hash,
                    other,
                    other_p,
                });
            }
            seen.insert(hash, i);
        }

        Ok(())
    }

    pub fn mv(&mut self, mv: Move) {
        let qturns = mv.qturns;
        // let (modifier, qturns) = if mv.qturns == 3 { (3, 1) } else { (0, 1) };
        // println!("mv {mv:?}\t{modifier}");

        match mv.kind {
            MoveKind::FaceTurn { face, layer } => {
                let (start, finish) = match layer {
                    LayerSpec::Outer {} => (0, 0),
                    LayerSpec::Inner { depth } => (depth, depth),
                    LayerSpec::Wide { width } => (0, width),
                };
                let mv = face as usize;
                for a in 0..(qturns + 4) % 4 {
                    println!("xd {a}");
                    for layer in start..=finish {
                        self._mv(mv, layer, false);
                        // self.print_cube();
                    }
                }
            }
            MoveKind::Rotation { axis } => {
                // Rotate all layers including the middle —_slice handles
                // the middle-layer skip via the `rotation` flag.
                let axis = axis as usize;
                let finish = (self.n - (self.n & 1)) as i32;
                for _ in 0..(qturns + 4) % 4 {
                    for layer in 0..=finish {
                        self._mv(axis, layer, true);
                    }
                }
            }
        }
    }

    /// Print the cube state with orbit labels and position data.
    /// Each orbit is labeled (Corners, Edges, Centers, EdgeWings,
    /// CenterWings, EdgeCenterWings) and shows the (x,y,z) position
    /// for each piece along with its orientation vector.
    pub fn print_cube(&self) {
        let orbit_names = [
            "Corners",
            "Edges",
            "Centers",
            "CenterWings",
            "EdgeWings",
            "EdgeCenterWings",
        ];
        println!("CubeVect n={}:", self.n);
        println!("  Cubies: {} total\n", self.data.len());
        let mut count = 0i32;
        let mut offset = 0;
        for (oi, &(_start, len, g)) in self.orbit.iter().enumerate() {
            if g == 0 {
                println!("  {}: (none)\n", orbit_names[oi]);
                continue;
            }
            println!(
                "  {}: {g} groups -> {len} pieces = {} pieces",
                orbit_names[oi],
                g * len
            );

            for gi in 0..g {
                let group_start = offset + gi * len;
                let group_end = group_start + len;
                println!("    group {gi} [{group_start}..{group_end}):");

                for (j, pi) in (group_start..group_end).enumerate() {
                    let p = &self.data[pi];
                    if pi < self.ori.len() {
                        let o = &self.ori[pi];
                        println!(
                            "{count}:{j}:({},{},{})->({},{},{})   ",
                            p.x, p.y, p.z, o.x, o.y, o.z
                        );
                    } else {
                        // if orbit_names[oi] == "EdgeWings" && ((count % 120 < 8) || (count % 144 < 8)){
                        println!("{count}:{j}:({},{},{})   ", p.x, p.y, p.z);
                        // }
                    }
                    count += 1;
                }
                println!();
            }
            offset += g * len;
            println!();
        }

        if offset < self.ori.len() {
            println!("  Extra orientation pieces (unmapped):");
            for pi in offset..self.ori.len() {
                let o = &self.ori[pi];
                print!("({},{},{})  ", o.x, o.y, o.z);
            }
            println!("\n");
        }
    }

    pub fn into_packed(&self) -> Vec<u128> {
        let mut lut = vec![0u8; 1 << 24];
        let mut slice = [Point3::new(0, 0, 0); 24];
        let n_mod_2 = (self.n & 1) as usize;
        let tmp = (self.n as usize - 2 - n_mod_2) >> 1;
        let stride = tmp.pow(2) + tmp + tmp * n_mod_2 + 1;
        let mut vec = vec![0; stride];

        let mut count = 0;
        for (i, &(start, len, g)) in self.orbit.iter().enumerate() {
            let mut start = start as usize;

            for index in 0..g {
                MAP_PERM_VECT[i](self.n, index, &mut slice[0..len]);
                for (j, p) in slice[0..len].iter().enumerate() {
                    lut[p.hash(self.n)] = j as u8;
                }
                for (j, p) in self.data[start..start + len].iter().enumerate() {
                    let std_pos = lut[p.hash(self.n)] as u128;
                    println!("{}\t{}\t{}", count, j, std_pos);
                    if i == 1 {
                        vec[count] |= (j as u128) << ((std_pos * 5) + 40);
                    } else if i == 2 {
                        vec[count] |= (j as u128) << ((std_pos * 5) + 100);
                    } else {
                        vec[count] |= (j as u128) << (std_pos * 5);
                    }
                }
                if i < 2 {
                    continue;
                }
                start += len;
                count += 1;
            }
            if i == 2 {
                count = 1;
            }
        }
        let caras = [
            Point3::new(0, 1, 0),
            Point3::new(0, 0, 1),
            Point3::new(1, 0, 0),
            Point3::new(0, -1, 0),
            Point3::new(0, 0, -1),
            Point3::new(-1, 0, 0),
        ];
        MAP_PERM_VECT[0](self.n, 0, &mut slice[0..8]);
        for (j, p) in slice[0..8].iter().enumerate() {
            lut[p.hash(self.n)] = j as u8;
        }
        for (o, piece) in self.ori[0..8].iter().zip(self.data[0..8].iter()) {
            let index = lut[piece.hash(self.n)];

            let (idx1, idx2, idx3) = if index < 4 {
                if index & 1 == 1 { (3, 5, 2) } else { (3, 1, 4) }
            } else {
                if index & 1 == 1 { (0, 1, 4) } else { (0, 5, 2) }
            };
            let mod_ = if *o == caras[idx1] {
                0
            } else if *o == caras[idx2] || *o == caras[idx3] {
                1
            } else {
                2
            };
            vec[0] |= mod_ << ((index * 5) + 3);
        }
        if self.n & 1 == 1 {
            MAP_PERM_VECT[1](self.n, 0, &mut slice[8..20]);
            for (j, p) in slice[8..20].iter().enumerate() {
                lut[p.hash(self.n)] = j as u8;
            }
            let edges: [Point3; 12] = [
                Point3::new(0, -1, 0),
                Point3::new(0, -1, 0),
                Point3::new(0, -1, 0),
                Point3::new(0, -1, 0),
                Point3::new(-1, 0, 0),
                Point3::new(-1, 0, 0),
                Point3::new(1, 0, 0),
                Point3::new(1, 0, 0),
                Point3::new(0, 1, 0),
                Point3::new(0, 1, 0),
                Point3::new(0, 1, 0),
                Point3::new(0, 1, 0),
            ];
            for (o, p) in self.ori[8..20].iter().zip(self.data[8..20].iter()) {
                let tmp = lut[p.hash(self.n)] as usize;
                if edges[tmp] != *o {
                    vec[0] |= 1 << ((tmp * 5) + 44);
                }
            }
        }

        vec
    }
}
fn pattern_4(a: i8, b: i8, c: i8, slice: &mut [Point3; 4]) {
    slice[0] = Point3::new(-a, b, -c);
    slice[1] = Point3::new(-c, b, a);
    slice[2] = Point3::new(a, b, c);
    slice[3] = Point3::new(c, b, -a);
}
fn pattern_8(a: i8, b: i8, c: i8, slice: &mut [Point3; 8]) {
    slice[0] = Point3::new(-a, b, -c);
    slice[1] = Point3::new(-c, b, -a);
    slice[2] = Point3::new(-c, b, a);
    slice[3] = Point3::new(-a, b, c);
    slice[4] = Point3::new(a, b, c);
    slice[5] = Point3::new(c, b, a);
    slice[6] = Point3::new(c, b, -a);
    slice[7] = Point3::new(a, b, -c);
}
fn orbit_slice_0(n: u8, _: usize, slice: &mut [Point3]) {
    let n = n as i8 >> 1;
    pattern_4(n, -n, n, (&mut slice[0..4]).try_into().unwrap());
    pattern_4(n, n, n, (&mut slice[4..8]).try_into().unwrap());
}
fn orbit_slice_1(n: u8, _: usize, slice: &mut [Point3]) {
    let n = (n >> 1) as i8;
    pattern_4(0, -n, n, (&mut slice[0..4]).try_into().unwrap());
    pattern_4(n, 0, n, (&mut slice[4..8]).try_into().unwrap());
    pattern_4(0, n, n, (&mut slice[8..12]).try_into().unwrap());
}
fn orbit_slice_2(n: u8, _: usize, slice: &mut [Point3]) {
    let n = (n >> 1) as i8;
    slice[0] = Point3::new(0, -n, 0);
    slice[1] = Point3::new(0, 0, -n);
    slice[2] = Point3::new(-n, 0, 0);
    slice[3] = Point3::new(0, 0, n);
    slice[4] = Point3::new(n, 0, 0);
    slice[5] = Point3::new(0, n, 0);
}
fn orbit_slice_3(n: u8, idx: usize, slice: &mut [Point3]) {
    let half = (n >> 1) as i8; // half of cube dimension
    let tmp = half as usize - 1; // number of distinct (x,z) offsets
    let x = half - (idx % tmp) as i8 - 1; // x offset: half-1 .. 1
    let z = half - 1 - ((idx - (idx % tmp)) / tmp) as i8; // z offset: half-1 .. 1

    pattern_4(x, -half, z, (&mut slice[0..4]).try_into().unwrap());
    if z == x {
        pattern_8(z, -z, half, (&mut slice[4..12]).try_into().unwrap());
        pattern_8(z, z, half, (&mut slice[12..20]).try_into().unwrap());
    } else if z > x {
        pattern_4(half, -z, x, (&mut slice[4..8]).try_into().unwrap());
        pattern_4(z, -x, half, (&mut slice[8..12]).try_into().unwrap());
        pattern_4(half, x, z, (&mut slice[12..16]).try_into().unwrap());
        pattern_4(x, z, half, (&mut slice[16..20]).try_into().unwrap());
    } else {
        pattern_4(z, -x, half, (&mut slice[4..8]).try_into().unwrap());
        pattern_4(half, -z, x, (&mut slice[8..12]).try_into().unwrap());
        pattern_4(x, z, half, (&mut slice[12..16]).try_into().unwrap());
        pattern_4(half, x, z, (&mut slice[16..20]).try_into().unwrap());
    }

    pattern_4(x, half, z, (&mut slice[20..24]).try_into().unwrap());
}
fn orbit_slice_4(n: u8, idx: usize, slice: &mut [Point3]) {
    let n = (n >> 1) as usize;
    let ni = (n - 1 - idx) as i8;
    let n = n as i8;
    pattern_8(ni, -n, n, (&mut slice[0..8]).try_into().unwrap());
    pattern_4(n, -ni, n, (&mut slice[8..12]).try_into().unwrap());
    pattern_4(n, ni, n, (&mut slice[12..16]).try_into().unwrap());
    pattern_8(ni, n, n, (&mut slice[16..24]).try_into().unwrap());
}
fn orbit_slice_5(n: u8, idx: usize, slice: &mut [Point3]) {
    let half = (n >> 1) as i8;
    let a = idx as i8 + 1;

    // Edge-centre wings: exactly one coord at ±half, one at 0,
    // one at ±a.  Uses a non-overlapping coordinate set vs orbit_3.
    // y = ±half: (0, half, ±a) and (±a, half, 0)
    pattern_4(0, half, a, (&mut slice[0..4]).try_into().unwrap());
    pattern_4(0, -half, a, (&mut slice[4..8]).try_into().unwrap());
    // y = ±a: (±half, a, 0) and (0, a, ±half)
    pattern_4(half, a, 0, (&mut slice[8..12]).try_into().unwrap());
    pattern_4(half, -a, 0, (&mut slice[12..16]).try_into().unwrap());
    // y = 0: (±half, 0, ±a) and (±a, 0, ±half)
    pattern_4(half, 0, a, (&mut slice[16..20]).try_into().unwrap());
    pattern_4(half, 0, -a, (&mut slice[20..24]).try_into().unwrap());
}
pub fn get_dim_from_len(len: usize) -> u8 {
    (1.0 + ((6 * len - 12) as f64).sqrt() / 6.0).round() as u8
}

impl CubeArena {
    pub fn into_cubevect(&self, index: usize) -> CubeVect {
        debug_assert!(index < self.len() + 2);
        let n = self.n() as usize;
        let vect = self.cube_to_vec(index);
        let mut cubevect = CubeVect::new(n);
        let data = cubevect.data.clone();
        let caras = [
            Point3::new(0, 1, 0),
            Point3::new(0, 0, 1),
            Point3::new(1, 0, 0),
            Point3::new(0, -1, 0),
            Point3::new(0, 0, -1),
            Point3::new(-1, 0, 0),
        ];
        let corners = [
            [3, 4, 5],
            [3, 5, 1],
            [3, 1, 2],
            [3, 2, 4],
            [0, 5, 4],
            [0, 1, 5],
            [0, 2, 1],
            [0, 4, 2],
        ];
        for (i, &(pos, ori)) in vect[0].iter().enumerate() {
            cubevect.data[pos as usize] = data[i];
            cubevect.ori[pos as usize] = caras[corners[i][ori as usize]];
        }
        let (mut a, b);
        if self.n() & 1 == 1 {
            let edges = [
                [3, 4],
                [3, 5],
                [3, 1],
                [3, 2],
                [5, 4],
                [5, 1],
                [2, 1],
                [2, 4],
                [0, 4],
                [0, 5],
                [0, 1],
                [0, 2],
            ];
            for (i, &(pos, ori)) in vect[1].iter().enumerate() {
                cubevect.data[pos as usize + 8] = data[i + 8];
                cubevect.ori[pos as usize + 8] = caras[edges[i][ori as usize]];
            }
            (a, b) = (20, 2);
        } else {
            (a, b) = (8, 1);
        }
        for orbit in &vect[b..] {
            for (i, &(pos, _)) in orbit.iter().enumerate() {
                cubevect.data[pos as usize + a] = data[i as usize + a];
            }
            a += orbit.len();
        }
        cubevect
    }
}
