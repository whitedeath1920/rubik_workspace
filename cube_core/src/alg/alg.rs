#![allow(dead_code, unused_variables)]
use crate::{error::Result, moves::{Move, MoveKind}, CubeError};

pub struct Alg {
    moves: Vec<Move>
}

impl Alg {
    fn new() -> Self {
        Self { moves: Vec::new() }
    }
    pub fn moves(&self) -> &[Move] {
        &self.moves
    }
    fn from_moves(moves: &[Move]) -> Self {
        Self { moves: moves.into() }
    } 
    pub fn invert(&self) -> Self {
        Self { moves: self.moves.iter().rev().map(|m| m.invert()).collect() }
    }
    pub fn append(&mut self, other: &Self) {
        self.moves.extend(other.moves.iter().cloned());
    }
    pub fn push(&mut self, move_: Move) {
        self.moves.push(move_);
    }
    pub fn len(&self) -> usize {
        self.moves.len()
    }
    pub fn conjugate(&self, rotation: Move) -> Result<Self> {
        let mut moves: Vec<Move> =self.moves().into();
        let axis = match rotation.kind {
            MoveKind::FaceTurn { face, layer } => {
                return Err(CubeError::InvalidMoveConjugate { got: rotation });
            },
            MoveKind::Rotation { axis } => {axis},
        };
        for mv in moves.iter_mut() {
            
        }
        Ok(Self { moves })
    }
}