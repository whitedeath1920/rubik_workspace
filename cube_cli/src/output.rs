use clap::{ValueEnum, Args};
use std::{io, path::PathBuf};
use serde_json;
use anyhow::Result;

pub trait PrettyPrint {
    fn pretty(&self) -> String;
}

pub trait BinaryPrint {
    fn binary(&self) -> Vec<u8>;
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum OutputFormat {
    Pretty,
    Json,
    Binary
}

#[derive(Args, Clone, Debug)]
pub struct OutputArgs {
    /// Output format
    #[arg(short, long, value_enum, default_value_t = OutputFormat::Pretty)]
    pub format: OutputFormat,

    /// Output file path (default: stdout)
    #[arg(short, long)]
    pub out: Option<PathBuf>,
}

impl OutputArgs {
    pub fn writer(&self) -> io::Result<Box<dyn io::Write>> {
        match &self.out {
            Some(path) => Ok(Box::new(std::fs::File::create(path)?)),
            None => Ok(Box::new(io::BufWriter::new(io::stdout().lock()))),
        }
    }

    pub fn writer_output<T>(&self, value: &T) -> Result<()> 
    where 
        T: serde::Serialize + PrettyPrint + BinaryPrint,
    {
        let mut w = self.writer()?;
        match self.format {
            OutputFormat::Pretty => {write!(w,"{}", value.pretty())?}
            OutputFormat::Json => {
                serde_json::to_writer(&mut w, value)?;
            }
            OutputFormat::Binary => {
                w.write_all(&value.binary())?;
            }
        }
        
        Ok(())
    }
}
