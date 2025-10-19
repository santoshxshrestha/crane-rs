use clap::Parser;
use std::path::PathBuf;

#[derive(Debug, Parser)]
#[command(name = "crane", author = "santoshxshrestha", version, about)]
pub struct Args {
    /// Port to run the server on
    #[arg(short = 'p', long, default_value = "8080")]
    pub port: u16,

    /// File(s) to be shared
    #[arg(short = 'f', long)]
    pub file: Vec<PathBuf>,
}

impl Args {
    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub fn get_files(&self) -> Vec<PathBuf> {
        self.file.clone()
    }
}
