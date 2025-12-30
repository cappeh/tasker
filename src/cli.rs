use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Commands {
    Add { task: String },
    List,
    Delete { id: u64 },
}

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}
