use clap::Parser;

#[derive(Debug, Parser)]
#[command(name = "crane", author = "santoshxshrestha", version, about)]
pub struct Args {
    #[arg(short = 'p', long, default_value = "8080")]
    pub port: u16,
}

impl Args {
    pub fn get_port(&self) -> u16 {
        self.port
    }
}
