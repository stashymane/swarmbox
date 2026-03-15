use crate::commands::build::BuildArgs;
use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// Path to project (defaults to current directory)
    #[arg(short, long)]
    pub project: Option<PathBuf>,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Subcommand)]
pub enum Commands {
    /// Builds your project
    Build(BuildArgs),
}
