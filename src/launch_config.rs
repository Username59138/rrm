use clap::Parser;
use std::{error::Error, path::PathBuf};

#[derive(Parser)]
#[command(author, version, about)]
pub struct LaunchConfig {
    #[arg(short, long)]
    pub verbose: bool,

    #[arg(long)]
    pub no_files_confirm: bool,

    #[arg(long)]
    pub no_confirm: bool,
    #[arg(long, conflicts_with = "no_confirm")]
    pub confirm: bool,

    #[arg(short, long)]
    pub include: Option<String>,
    #[arg(short, long)]
    pub exclude: Option<String>,
    #[arg(long, conflicts_with = "no_files_confirm")]
    pub confirm_files: Option<String>,

    pub files_path: Vec<PathBuf>,
}

impl LaunchConfig {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(LaunchConfig::parse())
    }
}
