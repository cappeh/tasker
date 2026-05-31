use clap::{Parser, Subcommand};

#[derive(Debug, Subcommand)]
pub enum Commands {
    Add(AddCmd),
    List,
    Delete { id: u64 },
}

#[derive(Debug, Parser)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Debug, Parser)]
pub struct AddCmd {
    #[arg(short, long)]
    pub task: String,
    #[arg(short, long)]
    pub desc: Option<String>,
}