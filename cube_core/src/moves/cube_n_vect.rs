use std::{collections::HashMap, ops::Range};

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Orbit {
    Corners(i32) = 0,
    Edges(i32) = 1,
    Centers(i32) = 2,
    ParCenters(i32) = 3,
    ParEdges(i32) = 4,
    EdgeCenters(i32) = 5,
}
impl Orbit {
    #[inline(always)]
    const fn idx(self) -> usize {
        match self {
            Orbit::Corners(_) => 0,
            Orbit::Edges(_) => 1,
            Orbit::Centers(_) => 2,
            Orbit::ParCenters(_) => 3,
            Orbit::ParEdges(_) => 4,
            Orbit::EdgeCenters(_) => 5,
        }
    }
    pub fn to_string(&self) -> String {
        match self {
            Orbit::Corners(i) => format!("Corners:{}", *i),
            Orbit::Edges(i) => format!("Edges:{}", *i),
            Orbit::Centers(i) => format!("Centers:{}", *i),
            Orbit::ParCenters(i) => format!("ParCenters:{}", *i),
            Orbit::ParEdges(i) => format!("ParEdges:{}", *i),
            Orbit::EdgeCenters(i) => format!("EdgeCenters:{}", *i),
        }
    }
    fn from_idx(kind: usize, idx: i32) -> Self {
        match kind {
            0 => Orbit::Corners(idx),
            1 => Orbit::Edges(idx),
            2 => Orbit::Centers(idx),
            3 => Orbit::ParCenters(idx),
            4 => Orbit::ParEdges(idx),
            5 => Orbit::EdgeCenters(idx),
            _ => panic!("Invalid orbit index"),
        }
    }
}
#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Layer {
    X(i32) = 0,
    Y(i32) = 1,
    Z(i32) = 2,
}
impl Layer {
    fn get_range(&self, len: usize) -> (Range<i32>, Range<i32>, Range<i32>) {
        let len = len as i32;
        let n = (len as i32) >> 1;
        let ran = -n..n + 1;
        match self {
            Layer::X(i) => {
                assert!(i.abs() < len);
                (*i..*i + 1, ran.clone(), ran.clone())
            }
            Layer::Y(i) => {
                assert!(i.abs() < len);
                (ran.clone(), *i..*i + 1, ran.clone())
            }
            Layer::Z(i) => {
                assert!(i.abs() < len);
                (ran.clone(), ran.clone(), *i..*i + 1)
            }
        }
    }
    fn to_string(&self) -> String {
        match self {
            Layer::X(i) => format!("X{}", i),
            Layer::Y(i) => format!("Y{}", i),
            Layer::Z(i) => format!("Z{}", i),
        }
    }
}
/// x, y, z
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(i32, i32, i32);

/// Orbit kind(suborbit number), index
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State(Orbit, i32);

pub struct CubeVect(HashMap<Point, State>);
impl CubeVect {
    pub fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|(p, s)| {
                format!(
                    "({}, {}, {}):({}, {})\n",
                    p.0,
                    p.1,
                    p.2,
                    s.0.to_string(),
                    s.1
                )
            })
            .collect::<Vec<_>>()
            .join(", ")
    }
}
pub struct CubePerm(HashMap<State, Point>);
impl CubePerm {
    pub fn to_string(&self) -> String {
        self.0
            .iter()
            .map(|(s, p)| {
                format!(
                    "({}, {}):({}, {}, {})\n",
                    s.0.to_string(),
                    s.1,
                    p.0,
                    p.1,
                    p.2
                )
            })
            .collect::<Vec<_>>()
            .join(", ")
    }
}
pub struct CubePair {
    pub vect: CubeVect,
    pub perm: CubePerm,
}
impl CubePair {
    pub fn new(dimension: usize) -> Self {
        let capacity = number_of_cubies(dimension);
        let mut vect = CubeVect(HashMap::with_capacity(capacity));
        let mut perm = CubePerm(HashMap::with_capacity(capacity));

        let dim_mod_2 = dimension % 2;
        let tmp1 = (dimension - 2 - dim_mod_2) / 2;
        let orbits = [1, dim_mod_2, dim_mod_2, tmp1.pow(2), tmp1, tmp1 * dim_mod_2];

        let mut arr: Vec<(Point, State)> = Vec::with_capacity(24);
        for (kind, &g) in orbits.iter().enumerate() {
            for index in 0..g {
                map_perm_vect(dimension, kind, index, &mut arr);
                for (p, s) in &arr {
                    vect.0.insert(*p, *s);
                    perm.0.insert(*s, *p);
                }
            }
        }

        Self { vect, perm }
    }
    pub fn print(&self) {
        println!("Vector: {}", self.vect.to_string());
        println!("Permutation: {}", self.perm.to_string());
    }
}
fn map_perm_vect(dimension: usize, kind: usize, index: usize, arr: &mut Vec<(Point, State)>) {
    arr.clear();
    let dimension = dimension as i32;
    let index = index as i32;
    match kind {
        // 0 => _sub_0(dimension, kind, index, arr),
        // 1 => _sub_1(dimension, kind, index, arr),
        // 2 => _sub_2(dimension, kind, index, arr),
        3 => _sub_3(dimension, kind, index, arr),
        4 => (),
        5 => (),
        _ => (),
        // _ => unreachable!("Invalid kind"),
    };
}
fn pattern_1(x: i32, y: i32, z: i32, kind: usize, idx: i32, start: i32) -> [(Point, State); 4] {
    [
        (Point(-x, y, -z), State(Orbit::from_idx(kind, idx), start)),
        (
            Point(-z, y, x),
            State(Orbit::from_idx(kind, idx), start + 1),
        ),
        (Point(x, y, z), State(Orbit::from_idx(kind, idx), start + 2)),
        (
            Point(z, y, -x),
            State(Orbit::from_idx(kind, idx), start + 3),
        ),
    ]
}
fn pattern_2(x: i32, y: i32, z: i32, kind: usize, idx: i32, start: i32) -> [(Point, State); 8] {
    [
        (Point(-x, y, -z), State(Orbit::from_idx(kind, idx), start)),
        (
            Point(-z, y, -x),
            State(Orbit::from_idx(kind, idx), start + 1),
        ),
        (
            Point(-z, y, x),
            State(Orbit::from_idx(kind, idx), start + 2),
        ),
        (
            Point(-x, y, z),
            State(Orbit::from_idx(kind, idx), start + 3),
        ),
        (Point(x, y, z), State(Orbit::from_idx(kind, idx), start + 4)),
        (Point(z, y, x), State(Orbit::from_idx(kind, idx), start + 5)),
        (
            Point(z, y, -x),
            State(Orbit::from_idx(kind, idx), start + 6),
        ),
        (
            Point(x, y, -z),
            State(Orbit::from_idx(kind, idx), start + 7),
        ),
    ]
}
fn _sub_0(dimension: i32, kind: usize, idx: i32, arr: &mut Vec<(Point, State)>) {
    let n = dimension >> 1;
    arr.extend_from_slice(&pattern_1(n, -n, n, kind, idx, 0));
    arr.extend_from_slice(&pattern_1(n, n, n, kind, idx, 4));
}
fn _sub_1(dimension: i32, kind: usize, idx: i32, arr: &mut Vec<(Point, State)>) {
    let n = dimension >> 1;
    arr.extend_from_slice(&pattern_1(0, -n, n, kind, idx, 0));
    arr.extend_from_slice(&pattern_1(n, 0, n, kind, idx, 4));
    arr.extend_from_slice(&pattern_1(0, n, n, kind, idx, 8));
}
fn _sub_2(dimension: i32, kind: usize, idx: i32, arr: &mut Vec<(Point, State)>) {
    let n = dimension >> 1;
    arr.extend_from_slice(&[
        (Point(0, -n, 0), State(Orbit::from_idx(kind, idx), 0)),
        (Point(0, 0, -n), State(Orbit::from_idx(kind, idx), 1)),
        (Point(-n, 0, 0), State(Orbit::from_idx(kind, idx), 2)),
        (Point(0, 0, n), State(Orbit::from_idx(kind, idx), 3)),
        (Point(n, 0, 0), State(Orbit::from_idx(kind, idx), 4)),
        (Point(0, n, 0), State(Orbit::from_idx(kind, idx), 5)),
    ]);
}
fn _sub_3(dimension: i32, kind: usize, idx: i32, arr: &mut Vec<(Point, State)>) {
    let n = dimension >> 1;
    let tmp = (dimension - 2 - (dimension % 2)) / 2;
    let x = n - (idx) % tmp - 1;
    let z = n - 1 - (idx - idx % tmp) / tmp;

    // arr.extend_from_slice(&pattern_1(x, -n, z, kind, idx, 0));
    if z == x {
        arr.extend_from_slice(&pattern_2(z, -z, n, kind, idx, 4));
        arr.extend_from_slice(&pattern_2(z, z, n, kind, idx, 12));
    } else if z > x {
        arr.extend_from_slice(&pattern_1(n, -z, x, kind, idx, 1));
        arr.extend_from_slice(&pattern_1(z, -x, n, kind, idx, 1));
        arr.extend_from_slice(&pattern_1(n, x, z, kind, idx, 1));
        arr.extend_from_slice(&pattern_1(x, z, n, kind, idx, 1));
    } else {
        arr.extend_from_slice(&pattern_1(z, -x, n, kind, idx, 1));
        arr.extend_from_slice(&pattern_1(n, -z, x, kind, idx, 1));
        arr.extend_from_slice(&pattern_1(x, z, n, kind, idx, 1));
        arr.extend_from_slice(&pattern_1(n, x, z, kind, idx, 1));
    } 
    // arr.extend_from_slice(&pattern_1(x, n, z,kind,idx,20));

}
fn number_of_cubies(dimension: usize) -> usize {
    dimension.pow(3) - (dimension - 2).pow(3)
}
