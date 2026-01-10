use anyhow::Result;
use cube_core::{
    CubeMoves, CubeState,
    cube_moves::{MoveClass, MoveRules, Turn},
    ops::Bit,
};
use serde::{Deserialize, Serialize};
use std::io::{Cursor, Read};

use clap::Args;

use crate::{
    input::{BinaryRead, InputArgs},
    output::{BinaryPrint, OutputArgs, PrettyPrint},
};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CubeStateWrapper {
    dimension: usize,
    perm: Vec<u128>,
    ori: [u32; 2],
}

impl CubeStateWrapper {
    pub fn try_new(dimension: usize) -> Result<CubeStateWrapper> {
        let cube = CubeState::try_new(dimension)?;
        Ok(CubeStateWrapper {
            dimension,
            perm: cube.perm,
            ori: cube.ori,
        })
    }
    pub fn _from_cubestate(cube: CubeState, dimension: usize) -> CubeStateWrapper {
        CubeStateWrapper {
            dimension,
            perm: cube.perm,
            ori: cube.ori,
        }
    }

    pub fn to_cubestate(&self) -> (CubeState, usize) {
        (
            CubeState {
                perm: self.perm.clone(),
                ori: self.ori.clone(),
            },
            self.dimension,
        )
    }
}

#[derive(Args)]
pub struct CubeStateCmd {
    /// Specifies the dimension of the cube
    #[arg(short, long)]
    dimension: Option<usize>,

    /// Sets a custom output for the CubeState
    #[command(flatten)]
    out: OutputArgs,

    /// Loads from input file
    #[command(flatten)]
    input: InputArgs,

    /// Algorithm for moving the cube in "Big Cube" notation
    #[arg(short, long)]
    moves: Option<String>,
}

impl CubeStateCmd {
    pub fn run(&self) -> anyhow::Result<()> {
        let cube;
        if let Some(dimension) = self.dimension {
            cube = CubeStateWrapper::try_new(dimension)?;
            self.out.writer_output(&cube)?;
        } else {
            cube = self.input.read_input::<CubeStateWrapper>()?;
            self.out.writer_output(&cube)?;
        }
        if self.moves.is_some() {
            let (mut c, _) = cube.to_cubestate();
            let rule = MoveRules {
                moves: vec![MoveClass::Outer, MoveClass::Rotation, MoveClass::Wide, MoveClass::Slice ],
                turns: vec![Turn::Clockwise, Turn::Double, Turn::Anticlockwise],
            };
            let moveset = CubeMoves::new(cube.dimension, &rule);
            let moves = self.moves.clone().unwrap();
            for mv in moves.split(' ') {
                moveset.make_move_s(mv, &mut c);
            }
            
            self.out
                .writer_output(&CubeStateWrapper::_from_cubestate(c, cube.dimension))?;
        }
        Ok(())
    }
}
impl PrettyPrint for CubeStateWrapper {
    fn pretty(&self) -> String {
        let mut s = "Cube state:\n".to_string();
        s = s + &format!("\tdimension:\t{}\n", self.dimension);
        if self.dimension < 2642246 {
            s = s + &format!(
                "\ttotal pieces:\t{}\n",
                self.dimension.pow(3) - (self.dimension - 2).pow(3)
            );
        } else {
            s = s + &format!("\ttotal pieces:\tto large to print\n");
        }

        s = s + &format!("\tperm subgroups:\n");
        let mut kind = 0;
        let mut cont = 0;
        for a in self.perm.iter() {
            if kind != a.get_kind() {
                kind = a.get_kind();
                cont = 0;
            }
            s = s + &format!(
                "\t\t{} {}: \t{:?}\n",
                cube_core::cube_state::KIND[kind],
                cont,
                cube_core::cube_state::_to_vec(*a)
            );

            cont += 1;
        }

        s = s + &format!(
            "\tori subgroups:\n\t\t{} {}: \t{:?}\n",
            cube_core::cube_state::KIND[0],
            1,
            cube_core::cube_state::_to_vec(self.ori[0])
        );

        if self.dimension & 1 == 1 {
            s = s + &format!(
                "\t\t{} {}: \t{:?}\n",
                cube_core::cube_state::KIND[1],
                1,
                cube_core::cube_state::_to_vec(self.ori[1])
            );
        }
        s
    }
}
impl BinaryPrint for CubeStateWrapper {
    fn binary(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(b"STATE");
        buffer.extend_from_slice(&self.dimension.to_le_bytes());
        buffer.extend_from_slice(&self.perm.len().to_le_bytes());
        let tmp = self
            .perm
            .iter()
            .flat_map(|&x| x.to_le_bytes())
            .collect::<Vec<_>>();
        buffer.extend_from_slice(&tmp);
        let tmp = self
            .ori
            .iter()
            .flat_map(|&x| x.to_le_bytes())
            .collect::<Vec<_>>();
        buffer.extend_from_slice(&tmp);

        buffer
    }
}

impl BinaryRead for CubeStateWrapper {
    fn binary_r(&mut self, buffer: &[u8]) -> anyhow::Result<()> {
        let mut cursor = Cursor::new(buffer);
        let mut magic = [0u8; 5];
        cursor.read_exact(&mut magic)?;
        if &magic != b"STATE" {
            return Err(anyhow::anyhow!(
                "Invalid format: got {}, expected {}",
                str::from_utf8(&magic)?,
                "STATE"
            ));
        }

        let mut tmp = [0u8; (usize::BITS / 8) as usize];
        cursor.read_exact(&mut tmp)?;
        self.dimension = usize::from_le_bytes(tmp);

        let mut tmp = [0u8; (usize::BITS / 8) as usize];
        cursor.read_exact(&mut tmp)?;
        let len = usize::from_le_bytes(tmp);

        self.perm = vec![0; len];
        let mut tmp = [0u8; 16];
        for i in 0..len {
            cursor.read_exact(&mut tmp)?;
            self.perm[i] = u128::from_le_bytes(tmp);
        }

        let mut tmp = [0u8; 4];
        cursor.read_exact(&mut tmp)?;
        self.ori[0] = u32::from_le_bytes(tmp);

        cursor.read_exact(&mut tmp)?;
        self.ori[1] = u32::from_le_bytes(tmp);

        Ok(())
    }
}

impl Default for CubeStateWrapper {
    fn default() -> Self {
        let dimension = 2;
        let cube = CubeState::new(dimension);
        Self {
            dimension,
            perm: cube.perm,
            ori: cube.ori,
        }
    }
}
