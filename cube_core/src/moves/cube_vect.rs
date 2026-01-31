use crate::{
    moves::{LayerSpec, Move, MoveKind},
    state::{Bit, CubeState},
};

fn map_perm_vect(dimension: usize, kind: usize, index: i32, mut vect: &mut Vec<[i32; 3]>) {
    vect.clear();
    match kind {
        0 => _sub_0(dimension, index, &mut vect),
        1 => _sub_1(dimension, index, &mut vect),
        2 => _sub_2(dimension, index, &mut vect),
        3 => _sub_3(dimension, index, &mut vect),
        4 => _sub_4(dimension, index, &mut vect),
        5 => _sub_5(dimension, index, &mut vect),
        _ => panic!("Invalid kind"),
    }
}

#[derive(Debug, Clone)]
pub struct CubeVect {
    // orbits(piece(x,y,z),kind)
    pub perm: Vec<(Vec<[i32; 3]>, u8)>,
    pub dimension: usize,
    pub ori: [Vec<[i32; 3]>; 2],
}

impl CubeVect {
    pub fn new(dimension: usize) -> Self {
        let dim_mod_2 = dimension % 2;
        let tmp1 = (dimension - 2 - dim_mod_2) / 2;
        let orbits = [1, dim_mod_2, dim_mod_2, tmp1.pow(2), tmp1, tmp1 * dim_mod_2];
        let mut perm = vec![(Vec::with_capacity(24), 0); orbits.iter().sum()];

        let mut cont = 0;
        let mut vect = Vec::with_capacity(24);
        for (i, &g) in orbits.iter().enumerate() {
            for index in 0..g {
                map_perm_vect(dimension, i, index as i32, &mut vect);
                // perm[cont].1 = i as u8;
                // std::mem::swap(&mut vect, &mut perm[cont].0);
                perm[cont] = (vect.clone(), i as u8);
                // perm[cont] = (std::mem::replace(&mut vect, Vec::with_capacity(24)), i as u8);
                cont += 1;
            }
        }
        let caras = [
            [0, 1, 0],
            [0, 0, 1],
            [1, 0, 0],
            [0, -1, 0],
            [0, 0, -1],
            [-1, 0, 0],
        ];
        let mut tmp1 = vec![[0; 3]; 8];
        for i in 0..4 {
            tmp1[i] = caras[3];
            tmp1[i + 4] = caras[0];
        }
        let mut tmp2 = vec![[0; 3]; 12];
        for i in 0..4 {
            tmp2[i] = caras[3];
            tmp2[i + 8] = caras[0];
        }
        tmp2[4] = caras[5];
        tmp2[5] = caras[5];
        tmp2[6] = caras[2];
        tmp2[7] = caras[2];
        Self {
            perm,
            dimension,
            ori: [tmp1, tmp2],
        }
    }
    fn _mv(&mut self, mv: usize, layer: i32) {
        let matrix = [
            [[0, 0, -1], [0, 1, 0], [1, 0, 0]],
            [[0, 1, 0], [-1, 0, 0], [0, 0, 1]],
            [[1, 0, 0], [0, 0, 1], [0, -1, 0]],
            [[0, 0, 1], [0, 1, 0], [-1, 0, 0]],
            [[0, -1, 0], [1, 0, 0], [0, 0, 1]],
            [[1, 0, 0], [0, 0, -1], [0, 1, 0]],
        ];
        for (kind, piece) in self._slice(mv, layer) {
            let tmp = self.perm[kind].0[piece];
            let tmp1 = if kind == 0 {
                self.ori[kind][piece]
            } else if self.dimension & 1 == 1 && kind == 1 {
                self.ori[kind][piece]
            } else {
                [0; 3]
            };
            let mut tmp3 = [0; 3];
            for (i, values) in matrix[mv].iter().enumerate() {
                self.perm[kind].0[piece][i] =
                    tmp[0] * values[0] + tmp[1] * values[1] + tmp[2] * values[2];
                tmp3[i] = tmp1[0] * values[0] + tmp1[1] * values[1] + tmp1[2] * values[2];
            }
            if kind == 0 || (kind == 1 && self.dimension & 1 == 1) {
                self.ori[kind][piece] = tmp3;
            }
        }
    }
    fn _slice(&self, axis: usize, layer: i32) -> Vec<(usize, usize)> {
        let mut slice = Vec::new();
        let cara = [(1, 1), (1, 2), (1, 0), (-1, 1), (-1, 2), (-1, 0)];
        let mut altura = (self.dimension >> 1) as i32 - layer;
        if altura <= 0 && self.dimension & 1 == 1 {
            altura -= 1;
        }
        altura *= cara[axis].0;
        for (i, (orbit, _)) in self.perm.iter().enumerate() {
            for (j, piece) in orbit.iter().enumerate() {
                if piece[cara[axis].1] == altura {
                    slice.push((i, j));
                }
            }
        }
        slice
    }
    pub fn mv(&self, mv: Move) -> Self {
        let mut cube = self.clone();
        let (_mv, qturns, start, finish) = match mv.kind {
            MoveKind::FaceTurn { face, layer } => {
                let (start, finish) = match layer {
                    LayerSpec::Outer {} => (0, 0),
                    LayerSpec::Inner { depth } => (depth, depth),
                    LayerSpec::Wide { width } => (0, width),
                };
                (face as usize, mv.qturns, start, finish)
            }
            MoveKind::Rotation { axis } => (
                axis as usize,
                mv.qturns,
                0,
                (self.dimension - (self.dimension & 1)) as i32,
            ),
        };
        for _ in 0..(qturns + 4) % 4 {
            for layer in start..=finish {
                cube._mv(_mv, layer);
            }
        }
        cube
    }
}

impl Into<CubeState> for CubeVect {
    fn into(self) -> CubeState {
        let mut perm = vec![0; self.perm.len()];
        let mut ori = [0, 1 << 29];
        let mut vect = Vec::with_capacity(24);
        let mut cont = 0;
        let dim_mod_2 = self.dimension & 1;
        let tmp1 = (self.dimension - 2 - dim_mod_2) / 2;
        let orbits = [1, dim_mod_2, dim_mod_2, tmp1.pow(2), tmp1, tmp1 * dim_mod_2];
        for (i, (v, kind)) in self.perm.iter().enumerate() {
            let kind = *kind as usize;
            perm[i].set_kind(kind);
            map_perm_vect(self.dimension, kind, cont, &mut vect);
            for (j, piece) in v.iter().enumerate() {
                let mut index = 0;
                for (i, p) in vect.iter().enumerate() {
                    if p == piece {
                        index = i;
                        break;
                    }
                }
                perm[i].set(index, j as u8);
            }
            cont += 1;
            if cont as usize >= orbits[kind] {
                cont = 0;
            }
        }
        let dims = self.dimension as i32 >> 1;
        let caras = [
            [0, 1, 0],
            [0, 0, 1],
            [1, 0, 0],
            [0, -1, 0],
            [0, 0, -1],
            [-1, 0, 0],
        ];
        for (o, piece) in self.ori[0].clone().iter().zip(self.perm[0].0.clone()) {
            let mut index = 0;

            map_perm_vect(self.dimension, 0, 0, &mut vect);
            for (i, p) in vect.iter().enumerate() {
                if *p == piece {
                    index = i;
                    break;
                }
            }
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
            ori[0].set(index, mod_);
        }
        if self.dimension & 1 == 1 {
            let layer = [[0, -1, 0], [-1, 0, 0], [1, 0, 0], [0, 1, 0]];
            let mut cont = 0;
            for (o, p) in self.ori[1].clone().iter().zip(self.perm[1].0.clone()) {
                let idx = if p[1] == -dims {
                    0
                } else if p[1] == dims {
                    3
                } else if p[1] == 0 && p[0] == -dims {
                    1
                } else {
                    2
                };
                if layer[idx] == *o {
                    ori[1].set(cont, 0);
                } else {
                    ori[1].set(cont, 1);
                }
                cont += 1;
            }
        }
        CubeState { perm, ori }
    }
}
fn _get_len_from_dim(dimension: usize) -> usize {
    (dimension.pow(2) + 5 * (dimension & 1) - 2 * dimension + 4) / 4
}
pub fn get_dim_from_len(len: usize, par: bool) -> usize {
    let len = len as f64;
    let dim0 = 1.0 + (4.0 * len - 3.0).sqrt();
    let dim1 = 1.0 + (4.0 * len - 8.0).sqrt();
    if dim1 - dim1.floor() < 0.0001 && !par {
        return dim1 as usize;
    } else if dim0 - dim0.floor() < 0.0001 && par {
        return dim0 as usize;
    } else {
        panic!("Invalid dimension for length {}", len);
    }
}
fn pattern_1(a: i32, b: i32, c: i32) -> [[i32; 3]; 4] {
    [[-a, b, -c], [-c, b, a], [a, b, c], [c, b, -a]]
}
fn pattern_2(a: i32, b: i32, c: i32) -> [[i32; 3]; 8] {
    [
        [-a, b, -c],
        [-c, b, -a],
        [-c, b, a],
        [-a, b, c],
        [a, b, c],
        [c, b, a],
        [c, b, -a],
        [a, b, -c],
    ]
}
fn _sub_0(dimension: usize, _: i32, vect: &mut Vec<[i32; 3]>) {
    let n = dimension as i32 >> 1;
    vect.extend_from_slice(&pattern_1(n, -n, n));
    vect.extend_from_slice(&pattern_1(n, n, n));
}
fn _sub_1(dimension: usize, _: i32, vect: &mut Vec<[i32; 3]>) {
    let n = dimension as i32 >> 1;
    vect.extend_from_slice(&pattern_1(0, -n, n));
    vect.extend_from_slice(&pattern_1(n, 0, n));
    vect.extend_from_slice(&pattern_1(0, n, n));
}
fn _sub_2(dimension: usize, _: i32, vect: &mut Vec<[i32; 3]>) {
    let n = dimension as i32 >> 1;
    vect.extend_from_slice(&[
        //x   y   z
        [0, -n, 0],
        [0, 0, -n],
        [-n, 0, 0],
        [0, 0, n],
        [n, 0, 0],
        [0, n, 0],
    ]);
}
fn _sub_3(dimension: usize, idx: i32, vect: &mut Vec<[i32; 3]>) {
    let n = dimension as i32 >> 1;
    let tmp = ((dimension - 2 - (dimension % 2)) / 2) as i32;
    let x = n - (idx) % tmp - 1;
    let z = n - 1 - (idx - idx % tmp) / tmp;

    vect.extend_from_slice(&pattern_1(x, -n, z));
    if z == x {
        vect.extend_from_slice(&pattern_2(z, -z, n));
        vect.extend_from_slice(&pattern_2(z, z, n));
    } else if z > x {
        vect.extend_from_slice(&pattern_1(n, -z, x));
        vect.extend_from_slice(&pattern_1(z, -x, n));
        vect.extend_from_slice(&pattern_1(n, x, z));
        vect.extend_from_slice(&pattern_1(x, z, n));
    } else {
        vect.extend_from_slice(&pattern_1(z, -x, n));
        vect.extend_from_slice(&pattern_1(n, -z, x));
        vect.extend_from_slice(&pattern_1(x, z, n));
        vect.extend_from_slice(&pattern_1(n, x, z));
    }

    vect.extend_from_slice(&pattern_1(x, n, z));
}
fn _sub_4(dimension: usize, idx: i32, vect: &mut Vec<[i32; 3]>) {
    let n = dimension as i32 >> 1;
    let ni = n - 1 - idx;
    vect.extend_from_slice(&pattern_2(ni, -n, n));
    vect.extend_from_slice(&pattern_1(n, -ni, n));
    vect.extend_from_slice(&pattern_1(n, ni, n));
    vect.extend_from_slice(&pattern_2(ni, n, n));
}
fn _sub_5(dimension: usize, idx: i32, vect: &mut Vec<[i32; 3]>) {
    let n = dimension as i32 >> 1;
    let a = idx + 1;
    let ni = n - a;
    vect.extend_from_slice(&pattern_1(a, -n, ni));
    vect.extend_from_slice(&pattern_2(ni, -ni, n));
    vect.extend_from_slice(&pattern_2(ni, ni, n));
    vect.extend_from_slice(&pattern_1(a, n, ni));
}
