use std::{collections::HashMap};

use crate::{CubeArena, cube_vect::CubeVect, error::Result};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MoveFamily {
    Rotation,
    Outer,
    Inner,
    Wide,
}
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Turn {
    Anticlockwise,
    Clockwise,
    Double,
}
impl Turn {
    #[inline]
    pub fn qturns(self) -> i8 {
        match self {
            Turn::Anticlockwise => 3,
            Turn::Clockwise => 1,
            Turn::Double => 2,
        }
    }
    pub fn from_usize(i: usize) -> Self {
        match i {
            0 => Turn::Anticlockwise,
            1 => Turn::Clockwise,
            2 => Turn::Double,
            _ => unreachable!(),
        }
    }
}
#[derive(Debug)]
pub struct MoveSet {
    // Rotation, TopLayer, MiddleLayer, MultipleLayer,
    pub moves: usize,

    // Anticlowise, Clockwise, Double
    pub turns: usize,
}
impl MoveSet {
    pub fn _new(moves: usize, turns: usize) -> Self {
        Self { moves, turns }
    }
    pub fn new(v_moves: Vec<MoveFamily>, v_turns: Vec<Turn>) -> Self {
        let moves = v_moves.into_iter().fold(0usize, |acc, m| acc | (1 << m as usize));
        let turns = v_turns.into_iter().fold(0usize, |acc, t| acc | (1 << t as usize));
        Self { moves, turns }
    }
    pub fn get_moves(&self, n: usize) -> usize {
        let sums = [
            3,
            6,
            6*(n-2),
            6*(n-1),
        ];
        let mut sum = 0;
        for a in 0..4 {
            sum += sums[a] * ((self.moves >> a) & 1)
        }
        sum
    }
    pub fn get_turns(&self) -> usize {
        self.turns.trailing_ones() as usize
    }
    pub fn total_moves(&self, n: usize) -> usize {
        self.get_turns() * self.get_moves(n)
    }
    pub fn contains_move(&self, mv: MoveFamily) -> bool {
        (self.moves >> mv as usize) & 1 == 1
    }
    pub fn contains_turn(&self, t: Turn) -> bool {
        (self.turns >> t as usize) & 1 == 1
    }
    pub fn create_turns(&self) -> Vec<Turn> {
        (0..self.turns.trailing_ones() as usize)
            .map(|i| Turn::from_usize(i))
            .collect()
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LayerSpec {
    Outer,
    Inner { depth: i32 },
    Wide { width: i32 },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Axis {
    Y,
    Z,
    X,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Face {
    U = 0,
    F = 1,
    R = 2,
    D = 3,
    B = 4,
    L = 5,
}
impl Face {
    #[inline]
    pub fn to_axis(&self) -> Axis {
        match self {
            Self::U | Self::D => Axis::X,
            Self::F | Self::B => Axis::Y,
            Self::R | Self::L => Axis::Z,
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MoveKind {
    FaceTurn { face: Face, layer: LayerSpec },
    Rotation { axis: Axis },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move {
    pub kind: MoveKind,
    pub qturns: i8, // 3, +1, +2
}
impl Move {
    pub fn invert(&self) -> Self {
        let mut qturns = (self.qturns + 2) % 4;
        if qturns == 0 {
           qturns = 2;
        }
       Self {
           kind: self.kind,
           qturns
       } 
    }
    pub fn to_string(&self) -> String {
        let qturns = (self.qturns + 4) % 4;
        let mut s = String::new();
        match self.kind {
            MoveKind::FaceTurn { face, layer } => {
                let face = match face {
                    Face::U => "U",
                    Face::D => "D",
                    Face::F => "F",
                    Face::B => "B",
                    Face::R => "R",
                    Face::L => "L",
                };
                match layer {
                    LayerSpec::Outer => {
                        s += face;
                    },
                    LayerSpec::Inner { depth } => {
                        if depth != 1 {s += &depth.to_string()}; 
                        s += face;
                    },
                    LayerSpec::Wide { width } => {
                        if width != 2 {s+= &width.to_string();}
                        s += face;
                        s += "w";
                    },
                }
            },
            MoveKind::Rotation { axis } => {
                s+= match axis {
                    Axis::X => "x",
                    Axis::Y => "y",
                    Axis::Z => "z",
                };
            }
        };
        if qturns == 0 {
            return "".to_string();
        } else if qturns == 2 {
            s += "2";
        } else if qturns == 3 {
            s += "'";
        }
        
        s
    }
}
#[derive(Debug, Clone)]
pub struct MoveTable {
    arena: CubeArena,
    moves: HashMap<Move, usize>,
    moves_s: HashMap<String, Move>
}
impl MoveTable {
    pub fn new(n: usize, rule: &MoveSet) -> Result<Self> {
        let mut arena = CubeArena::new_arena(n as u8, rule.total_moves(n));
        let mut moves = HashMap::new();
        let mut moves_s = HashMap::new();
        let mut cube = CubeVect::new(n);
        cube.print_cube();
        let mut cont = 0;
        let turns: Vec<i8> = rule.create_turns().iter().map(|t| t.qturns()).collect();

        if rule.contains_move(MoveFamily::Rotation) {
            let axes = [Axis::X, Axis::Y, Axis::Z];
            for axis in axes {
                for &qturns in &turns {
                    let mv = Move {
                        kind: MoveKind::Rotation { axis },
                        qturns,
                    };
                    eprintln!("{}",mv.to_string());
                    cube.mv(mv);
                    let c: Vec<u128> = cube.into_packed();
                    arena.cube_from_slice(cont, &c);
                    // match  arena.is_solvable(cont) {
                    //     Ok(_) => {}
                    //     Err(e) => {
                    //         cube.print_cube();
                    //         arena.print_cube(cont);
                    //         eprintln!("asdflaksjfdlaskdjfalskdfj \n\n\n\n{:?}", e);
                    //         return Err(e);
                    //     }
                    // };
                    moves.insert(mv, cont);
                    moves_s.insert(mv.to_string(), mv);
                    cont += 1;
                    cube.mv(mv.invert());
                }
            }
        }
        if rule.contains_move(MoveFamily::Outer) {
            let faces = [Face::U, Face::F, Face::R, Face::D, Face::B, Face::L];
            for face in faces {
                for &qturns in &turns {
                    let mv = Move {
                        kind: MoveKind::FaceTurn {
                            face,
                            layer: LayerSpec::Outer,
                        },
                        qturns,
                    };
                    eprintln!("{}",mv.to_string());
                    cube.mv(mv);
                    // cube.check()?;
                    let c: Vec<u128> = cube.into_packed();
                    arena.cube_from_slice(cont, &c);
                    // arena.is_solvable(cont)?;
                    moves.insert(mv, cont);
                    moves_s.insert(mv.to_string(), mv);
                    cont += 1;
                    cube.mv(mv.invert());
                }
            }
        }
        if rule.contains_move(MoveFamily::Inner) {
            let faces = [Face::U, Face::F, Face::R, Face::D, Face::B, Face::L];
            for face in faces {
                for index in 2..=(n >> 1) {
                    for &qturns in &turns {
                        let mv = Move {
                            kind: MoveKind::FaceTurn {
                                face,
                                layer: LayerSpec::Inner { depth: index as i32 },
                            },
                            qturns,
                        };
                        eprintln!("{}",mv.to_string());
                        cube.mv(mv);
                        // cube.check()?;
                        let c: Vec<u128> = cube.into_packed();
                        arena.cube_from_slice(cont, &c);
                        // arena.is_solvable(cont)?;
                        moves.insert(mv, cont);
                        moves_s.insert(mv.to_string(), mv);
                        cont += 1;
                        cube.mv(mv.invert());
                    }
                }
            }
        }
        if rule.contains_move(MoveFamily::Wide) {
            let faces = [Face::U, Face::F, Face::R, Face::D, Face::B, Face::L];
            for face in faces {
                for width in 2..=(n >> 1) {
                    for &qturns in &turns {
                        let mv = Move {
                            kind: MoveKind::FaceTurn {
                                face,
                                layer: LayerSpec::Wide { width: width as i32 },
                            },
                            qturns,
                        };
                        eprintln!("{}",mv.to_string());
                        cube.mv(mv);
                        cube.print_cube();
                        // cube.check().unwrap();
                        let c: Vec<u128> = cube.into_packed();
                        arena.cube_from_slice(cont, &c);
                        arena.print_cube(cont);
                        arena.is_solvable(cont).unwrap();
                        moves.insert(mv, cont);
                        moves_s.insert(mv.to_string(), mv);
                        cont += 1;
                        cube.mv(mv.invert());
                    }
                }
            }
        }
        Ok(Self { arena, moves , moves_s })
    }
    pub fn arena(&self) -> &CubeArena {
        &self.arena
    }
    pub fn moves(&self) -> &HashMap<Move, usize> {
        &self.moves
    }
    pub fn moves_s(&self) -> &HashMap<String, Move> {
        &self.moves_s
    }
    pub fn get_move_s(&self, mv: &str) -> &[u128] {
        let index  = self.moves[&self.moves_s[mv]];
        self.arena.get_cube(index)
    }
    
    // pub fn try_make_move_s(&self, mv: &str, cube: &mut CubeState) -> Result<()> {
    //     if self.moves_s.contains_key(mv) {
    //         cube.add_assign(&self.moves[&self.moves_s[mv]]);
    //         return Ok(());
    //     } else {
    //         return Err(CubeError::InvalidMoveS {
    //             got: mv.to_string(),
    //             expected: self.moves_s.keys().cloned().collect::<Vec<String>>(),
    //         });
    //     }
    // }
    
    pub fn get_move_m(&self, mv: &Move) -> &[u128] {
        let index = self.moves[mv];
        self.arena.get_cube(index)
    }

    // pub fn try_make_move_m(&self, mv: &Move, cube: &mut CubeState) -> Result<()> {
    //     if self.moves.contains_key(mv) {
    //         cube.add_assign(&self.moves[mv]);
    //     } else {
    //         return Err(CubeError::InvalidMoveM {
    //             got: *mv,
    //             expected: self.moves.keys().cloned().collect::<Vec<Move>>(),
    //         });
    //     }
    //     Ok(())
    // }
}