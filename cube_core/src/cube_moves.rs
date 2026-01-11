use std::{collections::HashMap, ops::AddAssign};

use crate::{error::Result, CubeError, CubeState, CubeVect};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum MoveClass {
    Rotation,
    Outer,
    Slice,
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
    pub fn amount(self) -> i8 {
        match self {
            Turn::Anticlockwise => 3,
            Turn::Clockwise => 1,
            Turn::Double => 2,
        }
    }
}
#[derive(Debug)]
pub struct MoveRules {
    /// Rotation, TopLayer, MiddleLayer, MultipleLayer,
    pub moves: Vec<MoveClass>,

    /// Anticlowise, Clockwise, Double
    pub turns: Vec<Turn>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Layers {
    Outer,
    Slice { index: i32 },
    Wide { width: i32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Axis {
    Y,
    Z,
    X,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Faces {
    U = 0,
    F = 1,
    R = 2,
    D = 3,
    B = 4,
    L = 5,
}
impl Faces {
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
    FaceTurn { face: Faces, layers: Layers },
    Rotation { axis: Axis },
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Move {
    pub kind: MoveKind,
    pub amount: i8, // 3, +1, +2
}

impl Move {
    pub fn to_string(&self) -> String {
        let amount = (self.amount + 4) % 4;
        let mut s = String::new();
        match self.kind {
            MoveKind::FaceTurn { face, layers } => {
                let face = match face {
                    Faces::U => "U",
                    Faces::D => "D",
                    Faces::F => "F",
                    Faces::B => "B",
                    Faces::R => "R",
                    Faces::L => "L",
                };
                match layers {
                    Layers::Outer => {
                        s += face;
                    },
                    Layers::Slice { index } => {
                        if index != 1 {s += &index.to_string()}; 
                        s += face;
                    },
                    Layers::Wide { width } => {
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
        if amount == 0 {
            return "".to_string();
        } else if amount == 2 {
            s += "2";
        } else if amount == 3 {
            s += "'";
        }
        
        s
    }
}
#[derive(Debug, Clone)]
pub struct CubeMoves {
    pub moves: HashMap<Move, CubeState>,
    pub moves_s: HashMap<String, Move>
}

impl CubeMoves {
    pub fn new(dimension: usize, rule: &MoveRules) -> Self {
        let mut moves = HashMap::new();
        let mut moves_s = HashMap::new();
        let cube = CubeVect::new(dimension);
        
        let turns: Vec<i8> = rule.turns.iter().map(|t| t.amount()).collect();

        if rule.moves.contains(&MoveClass::Rotation) {
            let axes = [Axis::X, Axis::Y, Axis::Z];
            for axis in axes {
                for &amount in &turns {
                    let mv = Move {
                        kind: MoveKind::Rotation { axis },
                        amount,
                    };
                    let v = cube.mv(mv);
                    moves.insert(mv, v.into());
                    moves_s.insert(mv.to_string(), mv);
                }
            }
        }
        if rule.moves.contains(&MoveClass::Outer) {
            let faces = [Faces::U, Faces::F, Faces::R, Faces::D, Faces::B, Faces::L];
            for face in faces {
                for &amount in &turns {
                    let mv = Move {
                        kind: MoveKind::FaceTurn {
                            face,
                            layers: Layers::Outer,
                        },
                        amount,
                    };
                    let v = cube.mv(mv);
                    let v: CubeState = v.into();

                    moves.insert(mv, v.clone());
                    moves_s.insert(mv.to_string(), mv);
                }
            }
        }
        if rule.moves.contains(&MoveClass::Slice) {
            let faces = [Faces::U, Faces::F, Faces::R, Faces::D, Faces::B, Faces::L];
            for face in faces {
                for index in 2..=(dimension >> 1) {
                    for &amount in &turns {
                        let mv = Move {
                            kind: MoveKind::FaceTurn {
                                face,
                                layers: Layers::Slice { index: index as i32 },
                            },
                            amount,
                        };
                        let v = cube.mv(mv);
                        moves.insert(mv, v.into());
                        moves_s.insert(mv.to_string(), mv);
                    }
                }
            }
        }
        if rule.moves.contains(&MoveClass::Wide) {
            let faces = [Faces::U, Faces::F, Faces::R, Faces::D, Faces::B, Faces::L];
            for face in faces {
                for width in 2..=(dimension >> 1) {
                    for &amount in &turns {
                        let mv = Move {
                            kind: MoveKind::FaceTurn {
                                face,
                                layers: Layers::Wide { width: width as i32 },
                            },
                            amount,
                        };
                        let v = cube.mv(mv);
                        moves.insert(mv, v.into());
                        moves_s.insert(mv.to_string(), mv);
                    }
                }
            }
        }
        Self { moves , moves_s }
    }

    pub fn make_move_s(&self, mv: &str, cube: &mut CubeState) {
        cube.add_assign(&self.moves[&self.moves_s[mv]]);
    }
    
    pub fn try_make_move_s(&self, mv: &str, cube: &mut CubeState) -> Result<()> {
        if self.moves_s.contains_key(mv) {
            cube.add_assign(&self.moves[&self.moves_s[mv]]);
            return Ok(());
        } else {
            return Err(CubeError::InvalidMoveS {
                got: mv.to_string(),
                expected: self.moves_s.keys().cloned().collect::<Vec<String>>(),
            });
        }
    }
    
    pub fn make_move_m(&self, mv: &Move, cube: &mut CubeState) {
        cube.add_assign(&self.moves[mv]);
    }

    pub fn try_make_move_m(&self, mv: &Move, cube: &mut CubeState) -> Result<()> {
        if self.moves.contains_key(mv) {
            cube.add_assign(&self.moves[mv]);
        } else {
            return Err(CubeError::InvalidMoveM {
                got: *mv,
                expected: self.moves.keys().cloned().collect::<Vec<Move>>(),
            });
        }
        Ok(())
    }
}