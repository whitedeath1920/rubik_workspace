use anyhow::Result;

use clap::Args;

use crate::{
    input::{BinaryRead, InputArgs},
    output::{BinaryPrint, OutputArgs, PrettyPrint},
};
use cube_core::Layout;
use serde::{Deserialize, Serialize};
use std::io::{Cursor, Read};


#[derive(Debug, Serialize, Deserialize)]
struct LayoutWrapper {
    dimension: usize,
    par: bool,
    subgroups: [usize; 7],
    subgroups_offset: [usize; 7],
    groups_len: usize,
}

impl LayoutWrapper {
    pub fn try_new(dimension: usize) -> Result<LayoutWrapper> {
        let l = Layout::try_new(dimension)?;

        Ok(LayoutWrapper {
            dimension: l.dimension(),
            par: l.par(),
            subgroups: l.subgroups(),
            subgroups_offset: l.subgroups_offset(),
            groups_len: l.groups_len(),
        })
    }
}

#[derive(Args)]
pub struct LayoutCmd {
    /// Specifies the dimension of the cube layout
    #[arg(short, long)]
    dimension: Option<usize>,

    /// Sets a custom output for the layout
    #[command(flatten)]
    out: OutputArgs,

    /// Loads from input file
    #[command(flatten)]
    input: InputArgs,
}

impl LayoutCmd {
    pub fn run(&self) -> anyhow::Result<()> {
        if let Some(dimension) = self.dimension {
            let layout: LayoutWrapper = LayoutWrapper::try_new(dimension)?;
            self.out.writer_output(&layout)?;
        } else {
            let layout = self.input.read_input::<LayoutWrapper>()?;
            self.out.writer_output(&layout)?;
        }

        Ok(())
    }
}

impl PrettyPrint for LayoutWrapper {
    fn pretty(&self) -> String {
        let mut s = "Layout:\n".to_string();
        s = s + &format!("\tdimension:\t{}\n", self.dimension);
        if self.dimension < 2642246 {
            s = s + &format!(
                "\ttotal pieces:\t{}\n",
                self.dimension.pow(3) - (self.dimension - 2).pow(3)
            );
        } else {
            s = s + &format!("\ttotal pieces:\tto large to print\n");
        }
        s = s + &format!("\tparity:\t\t{}\n", self.par);
        s = s + &format!("\tsubgroups:\n");
        for i in 0..7 {
            s = s + &format!("\t\t{}:\t{}\n", cube_core::cube_state::KIND[i], self.subgroups[i]);
        }

        s
    }
}

impl BinaryPrint for LayoutWrapper {
    fn binary(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.extend_from_slice(b"LAYOUT");
        buffer.extend_from_slice(&self.dimension.to_le_bytes());
        buffer.extend_from_slice(if self.par { &[1] } else { &[0] });
        let tmp = self.subgroups.iter().map(|x| x.to_le_bytes()).flatten();
        let tmp = tmp.collect::<Vec<_>>();
        buffer.extend(tmp);
        let tmp = self
            .subgroups_offset
            .iter()
            .map(|x| x.to_le_bytes())
            .flatten();
        let tmp = tmp.collect::<Vec<_>>();
        buffer.extend(tmp);
        buffer.extend_from_slice(&self.groups_len.to_le_bytes());
        buffer
    }
}

impl BinaryRead for LayoutWrapper {
    fn binary_r(&mut self, buffer: &[u8]) -> Result<()> {
        let mut cursor = Cursor::new(buffer);
        let mut magic = [0u8; 6];
        cursor.read_exact(&mut magic)?;
        if &magic != b"LAYOUT" {
            return Err(anyhow::anyhow!("Invalid format"));
        }

        let mut tmp = [0u8; (usize::BITS / 8) as usize];
        cursor.read_exact(&mut tmp)?;
        self.dimension = usize::from_le_bytes(tmp);

        let mut tmp = [0u8; 1];
        cursor.read_exact(&mut tmp)?;
        self.par = if tmp[0] == 1 { true } else { false };

        let mut tmp = [0u8; 8];
        for i in 0..7 {
            cursor.read_exact(&mut tmp)?;
            self.subgroups[i] = usize::from_le_bytes(tmp);
        }

        let mut tmp = [0u8; 8];
        for i in 0..7 {
            cursor.read_exact(&mut tmp)?;
            self.subgroups_offset[i] = usize::from_le_bytes(tmp);
        }

        let mut tmp = [0u8; (usize::BITS / 8) as usize];
        cursor.read_exact(&mut tmp)?;
        self.groups_len = usize::from_le_bytes(tmp);
        Ok(())
    }
}

impl Default for LayoutWrapper {
    fn default() -> Self {
        let layout = Layout::new(2);
        Self {
            dimension: layout.dimension(),
            par: layout.par(),
            subgroups: layout.subgroups(),
            subgroups_offset: layout.subgroups_offset(),
            groups_len: layout.groups_len(),
        }
    }
}
