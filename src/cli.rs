use clap::{Parser, Subcommand};
use crate::todo::Status;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Add(AddCmd),
    List,
    Delete { id: u64 },
    Update { id: u64, status: Status }
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