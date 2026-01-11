use anyhow::Result;
use cube_core::{
    cube_moves::{MoveClass, MoveRules, Turn}, cube_perm::{cube_perm::{_to_vec, KIND}, ops::Bit}, CubeMoves, CubePerm
};
use std::io::{Cursor, Read};

use clap::Args;

use crate::{
    input::{BinaryRead, InputArgs},
    output::{BinaryPrint, OutputArgs, PrettyPrint, VisualPrint},
};

// const W: &str = " \x1b[38;2;255;255;255m■";
// const G: &str = " \x1b[38;2;0;255;1m■";
// const R: &str = " \x1b[38;2;254;0;0m■";
// const Y: &str = " \x1b[38;2;255;255;0m■";
// const B: &str = " \x1b[38;2;0;0;254m■";
// const O: &str = " \x1b[38;2;250;200;1m■";
// const K: &str = " \x1b[38;2;0;0;0m■";
// const COLORS: [&str;7] = [W,G,R,Y,B,O,K];

#[derive(Debug, Clone)]
pub struct CubePermWrapper {
    dimension: usize,
    cube: CubePerm,
}

impl CubePermWrapper {
    pub fn try_new(dimension: usize) -> Result<CubePermWrapper> {
        let cube = CubePerm::try_new(dimension)?;
        Ok(CubePermWrapper {
            dimension,
            cube
        })
    }
}

#[derive(Args)]
pub struct CubePermCmd {
    /// Specifies the dimension of the cube
    #[arg(short, long)]
    dimension: Option<usize>,

    /// Sets a custom output for the CubePerm
    #[command(flatten)]
    out: OutputArgs,

    /// Loads from input file
    #[command(flatten)]
    input: InputArgs,

    /// Algorithm for moving the cube in "Big CubePerm" notation
    #[arg(short, long)]
    moves: Option<String>,
}

impl CubePermCmd {
    pub fn run(&self) -> anyhow::Result<()> {
        let mut cube;
        if let Some(dimension) = self.dimension {
            cube = CubePermWrapper::try_new(dimension)?;
            self.out.writer_output(&cube)?;
        } else {
            cube = self.input.read_input::<CubePermWrapper>()?;
            self.out.writer_output(&cube)?;
        }
        if self.moves.is_some() {
            let rule = MoveRules {
                moves: vec![
                    MoveClass::Outer,
                    MoveClass::Rotation,
                    MoveClass::Wide,
                    MoveClass::Slice,
                ],
                turns: vec![Turn::Clockwise, Turn::Double, Turn::Anticlockwise],
            };
            let moveset = CubeMoves::new(cube.dimension, &rule);
            let moves = self.moves.clone().unwrap();
            for mv in moves.split(' ') {
                moveset.make_move_s(mv, &mut cube.cube);
            }

            self.out
                .writer_output(&cube)?;
        }
        Ok(())
    }
}
impl PrettyPrint for CubePermWrapper {
    fn pretty(&self) -> String {
        let mut s = "CubePerm state:\n".to_string();
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
        for a in self.cube.perm.iter() {
            if kind != a.get_kind() {
                kind = a.get_kind();
                cont = 0;
            }
            s = s + &format!(
                "\t\t{} {}: \t{:?}\n",
                KIND[kind],
                cont,
                _to_vec(*a)
            );

            cont += 1;
        }

        s = s + &format!(
            "\tori subgroups:\n\t\t{} {}: \t{:?}\n",
            KIND[0],
            1,
            _to_vec(self.cube.ori[0])
        );

        if self.dimension & 1 == 1 {
            s = s + &format!(
                "\t\t{} {}: \t{:?}\n",
                KIND[1],
                1,
                _to_vec(self.cube.ori[1])
            );
        }
        s
    }
}
impl BinaryPrint for CubePermWrapper {
    fn binary(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(b"STATE");
        buffer.extend_from_slice(&self.dimension.to_le_bytes());
        buffer.extend_from_slice(&self.cube.perm.len().to_le_bytes());
        let tmp = self
            .cube
            .perm
            .iter()
            .flat_map(|&x| x.to_le_bytes())
            .collect::<Vec<_>>();
        buffer.extend_from_slice(&tmp);
        let tmp = self
            .cube
            .ori
            .iter()
            .flat_map(|&x| x.to_le_bytes())
            .collect::<Vec<_>>();
        buffer.extend_from_slice(&tmp);

        buffer
    }
}

impl BinaryRead for CubePermWrapper {
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

        self.cube.perm = vec![0; len];
        let mut tmp = [0u8; 16];
        for i in 0..len {
            cursor.read_exact(&mut tmp)?;
            self.cube.perm[i] = u128::from_le_bytes(tmp);
        }

        let mut tmp = [0u8; 4];
        cursor.read_exact(&mut tmp)?;
        self.cube.ori[0] = u32::from_le_bytes(tmp);

        cursor.read_exact(&mut tmp)?;
        self.cube.ori[1] = u32::from_le_bytes(tmp);

        Ok(())
    }
}

impl Default for CubePermWrapper {
    fn default() -> Self {
        let dimension = 2;
        let cube = CubePerm::new(dimension);
        Self {
            dimension,
            cube 
        }
    }
}
impl VisualPrint for CubePermWrapper {
    fn visual(&self) -> String {
        self.pretty()
    }
}
// impl VisualPrint for CubePermWrapper {
//     fn visual(&self) -> String {
//         let altura = self.dimension as i32 >> 1;
//         let mut lines: Vec<Vec<String>> = vec![
//             (0..self.dimension * 4)
//                 .map(|_| "  ".to_string())
//                 .collect::<Vec<String>>();
//             self.dimension * 3
//         ];

//         let (esquinas, aristas, centros) = get_grupos(self.dimension);

//         let orientacion_8: Vec<[&str; 3]> = vec![
//             [W, R, G],
//             [W, B, R],
//             [W, G, O],
//             [W, O, B],
//             [Y, R, G],
//             [Y, B, R],
//             [Y, G, O],
//             [Y, O, B],
//         ];
//         let orientacion_12: Vec<&str> = vec![R, G, O, B, R, O, R, G, O, B, R, O];
//         let rot_corner: [[[i32; 3]; 3]; 8] = [
//             [[0, 1, 0], [0, 0, 1], [1, 0, 0]],
//             [[0, 0, -1], [1, 0, 0], [0, -1, 0]],
//             [[0, 0, -1], [-1, 0, 0], [0, 1, 0]],
//             [[0, -1, 0], [0, 0, -1], [1, 0, 0]],
//             [[0, -1, 0], [0, 0, -1], [1, 0, 0]],
//             [[0, 0, -1], [-1, 0, 0], [0, 1, 0]],
//             [[0, 0, -1], [1, 0, 0], [0, -1, 0]],
//             [[0, 1, 0], [0, 0, 1], [1, 0, 0]],
//         ];
//         let rot_edge: [[[i32; 3]; 3]; 12] = [
//             [[0, 1, 0], [1, 0, 0], [0, 0, 0]],
//             [[0, 0, 0], [0, 0, 1], [0, 1, 0]],
//             [[0, -1, 0], [-1, 0, 0], [0, 0, 0]],
//             [[0, 0, 0], [0, 0, -1], [0, -1, 0]],
//             [[0, 0, 1], [0, 0, 0], [1, 0, 0]],
//             [[0, 0, -1], [0, 0, 0], [-1, 0, 0]],
//             [[0, -1, 0], [-1, 0, 0], [0, 0, 0]],
//             [[0, 0, 0], [0, 0, -1], [0, -1, 0]],
//             [[0, 1, 0], [1, 0, 0], [0, 0, 0]],
//             [[0, 0, 0], [0, 0, 1], [0, 1, 0]],
//             [[0, 0, -1], [0, 0, 0], [-1, 0, 0]],
//             [[0, 0, 1], [0, 0, 0], [1, 0, 0]],
//         ];
//         let mut cont = 0;

//         for index in esquinas {
//             let pieza = self.piezas[index];
//             let (x, y) = self.get_base_pos(pieza);
//             lines[y][x] = self.colors[index].clone();

//             let mut i = 9;
//             if pieza[0] == [altura, altura, altura] {
//                 i = 0;
//             } else if [altura, altura, -altura] == pieza[0] {
//                 i = 1;
//             } else if [-altura, altura, altura] == pieza[0] {
//                 i = 2;
//             } else if [-altura, altura, -altura] == pieza[0] {
//                 i = 3;
//             } else if pieza[0] == [altura, -altura, altura] {
//                 i = 4;
//             } else if [altura, -altura, -altura] == pieza[0] {
//                 i = 5;
//             } else if [-altura, -altura, altura] == pieza[0] {
//                 i = 6;
//             } else if [-altura, -altura, -altura] == pieza[0] {
//                 i = 7;
//             }
//             let [_, rot1] = self.rotate(rot_corner[i], [[0, 0, 0], pieza[1]]);
//             let (x1, y1) = self.get_base_pos([pieza[0], rot1]);

//             let [_, rot1] = self.rotate(rot_corner[i], [[0, 0, 0], rot1]);
//             let (x, y) = self.get_base_pos([pieza[0], rot1]);

//             if cont > 3 && i < 4 || cont < 4 && i > 3 {
//                 lines[y1][x1] = orientacion_8[cont][2].to_string();
//                 lines[y][x] = orientacion_8[cont][1].to_string();
//             } else {
//                 lines[y1][x1] = orientacion_8[cont][1].to_string();
//                 lines[y][x] = orientacion_8[cont][2].to_string();
//             }
//             cont += 1;
//         }
//         let tmp = self.cube.identity();
//         for index in aristas {
//             let pieza = self.piezas[index];
//             let (x, y) = self.get_base_pos(pieza);
//             lines[y][x] = self.colors[index].clone();

//             let mut i = 13;
//             if pieza[0][1] == altura && pieza[0][0] == altura {
//                 i = 0;
//             } else if pieza[0][1] == altura && pieza[0][2] == altura {
//                 i = 1;
//             } else if pieza[0][1] == altura && pieza[0][0] == -altura {
//                 i = 2;
//             } else if pieza[0][1] == altura && pieza[0][2] == -altura {
//                 i = 3;
//             } else if pieza[0][2] == altura && pieza[0][0] == altura {
//                 i = 4;
//             } else if pieza[0][2] == altura && pieza[0][0] == -altura {
//                 i = 5;
//             } else if pieza[0][1] == -altura && pieza[0][0] == altura {
//                 i = 6;
//             } else if pieza[0][1] == -altura && pieza[0][2] == altura {
//                 i = 7;
//             } else if pieza[0][1] == -altura && pieza[0][0] == -altura {
//                 i = 8;
//             } else if pieza[0][1] == -altura && pieza[0][2] == -altura {
//                 i = 9;
//             } else if pieza[0][2] == -altura && pieza[0][0] == altura {
//                 i = 10;
//             } else if pieza[0][2] == -altura && pieza[0][0] == -altura {
//                 i = 11;
//             }
//             let mut j = 14;
//             let pieza2 = tmp.piezas[index];
//             if pieza2[0][1] == altura && pieza2[0][0] == altura {
//                 j = 0;
//             } else if pieza2[0][1] == altura && pieza2[0][2] == altura {
//                 j = 1;
//             } else if pieza2[0][1] == altura && pieza2[0][0] == -altura {
//                 j = 2;
//             } else if pieza2[0][1] == altura && pieza2[0][2] == -altura {
//                 j = 3;
//             } else if pieza2[0][2] == altura && pieza2[0][0] == altura {
//                 j = 4;
//             } else if pieza2[0][2] == altura && pieza2[0][0] == -altura {
//                 j = 5;
//             } else if pieza2[0][1] == -altura && pieza2[0][0] == altura {
//                 j = 6;
//             } else if pieza2[0][1] == -altura && pieza2[0][2] == altura {
//                 j = 7;
//             } else if pieza2[0][1] == -altura && pieza2[0][0] == -altura {
//                 j = 8;
//             } else if pieza2[0][1] == -altura && pieza2[0][2] == -altura {
//                 j = 9;
//             } else if pieza2[0][2] == -altura && pieza2[0][0] == altura {
//                 j = 10;
//             } else if pieza2[0][2] == -altura && pieza2[0][0] == -altura {
//                 j = 11;
//             }
//             let [_, rot1] = self.rotate(rot_edge[i], [[0, 0, 0], pieza[1]]);
//             let (x1, y1) = self.get_base_pos([pieza[0], rot1]);
//             lines[y1][x1] = orientacion_12[j].to_string();
//         }
//         for index in centros {
//             let (x, y) = self.get_base_pos(self.piezas[index]);
//             lines[y][x] = self.colors[index].clone();
//         }
//         let s = lines
//                 .iter()
//                 .map(|x| x.iter().map(|y| y.clone()).collect::<String>() + "\n")
//                 .collect::<String>() + "\x1b[38;2;255;255;255m";
        
//         s
//     }
// }
// pub fn get_grupos(dimension: usize) -> (usize, Vec<usize>, Vec<usize>) {
//     let mut aristas = vec![];
//     let mut centros = vec![];
    
//     let desfase;
//     if dimension & 1 == 1 {
//         aristas.push(1);
//         centros.push(2);
//         desfase = 1;
//     } else {
//         desfase = 0;
//     }
    
//     (0,aristas,centros)
// }