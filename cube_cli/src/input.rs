use std::path::PathBuf;

use clap::Args;
use anyhow::Result;
pub trait BinaryRead {
    fn binary_r(&mut self, buffer: &[u8]) -> Result<()>;
}

#[derive(Args, Clone, Debug)]
pub struct InputArgs {
    /// Input file path
    #[arg(short, long)]
    pub input: Option<PathBuf>,
}

impl InputArgs {
    pub fn read_input<'a, T>(&self) -> Result<T>
    where
        T: BinaryRead + Default,
    {
        if let Some(input) = &self.input {
        let bytes = std::fs::read(input)?;
        
        let mut value = T::default();
        value.binary_r(&bytes)?;
        Ok(value)
            
        } else {
            return Err(anyhow::Error::msg("Input file path is required"));
        }
    }
}