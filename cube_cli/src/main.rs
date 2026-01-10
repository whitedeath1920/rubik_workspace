use clap::{Parser, Subcommand};


mod commands;
mod input;
mod output;

#[derive(Parser)]
#[command(
    name = "cube_cli",
    version = "0.1.0",
    about = "NxN Rubik Cube research and solving CLI",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Creates a new layout for the cube
    Layout(commands::layout::LayoutCmd),
    CubeState(commands::cube_state::CubeStateCmd),
}

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Layout(cmd) => cmd.run(),
        Commands::CubeState(cmd) => cmd.run(),
    }
}
