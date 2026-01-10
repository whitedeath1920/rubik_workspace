use clap::{ValueEnum, Args};
use std::{io, path::PathBuf};
use anyhow::Result;

pub trait PrettyPrint {
    fn pretty(&self) -> String;
}

pub trait BinaryPrint {
    fn binary(&self) -> Vec<u8>;
}

pub trait VisualPrint {
    fn visual(&self) -> String;
}

#[derive(Clone, Copy, Debug, ValueEnum)]
pub enum OutputFormat {
    Pretty,
    Binary,
    Visual,
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
        T: PrettyPrint + BinaryPrint + VisualPrint,
    {
        let mut w = self.writer()?;
        match self.format {
            OutputFormat::Pretty => {write!(w,"{}", value.pretty())?},
            OutputFormat::Binary => {
                w.write_all(&value.binary())?;
            },
            OutputFormat::Visual => {
                write!(w,"{}",value.visual())?;
            },
        }
        
        Ok(())
    }
}
