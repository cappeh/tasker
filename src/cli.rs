use clap::{Parser, Subcommand};
use crate::error::TaskerError;
use crate::output::print_todos;
use crate::store::TodoStore;
use crate::todo::Status;

#[derive(Debug, Subcommand)]
pub enum Commands {
    Add(AddCmd),
    List {
        #[arg(short, long)]
        status: Option<Status>
    },
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

impl Cli {
    pub fn run<S: TodoStore>(self, store: S) -> Result<(), TaskerError> {
        match self.command {
            Commands::Add(AddCmd { task, desc }) => {
                store.add(task, desc)?;
            },
            Commands::List { status } => {
                let todos = store.list(status)?;
                print_todos(&todos);
            },
            Commands::Delete { id } => store.delete(id)?,
            Commands::Update { id, status } => store.update_status(id, status)?
        }
        Ok(())
    }
}