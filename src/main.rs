use clap::Parser;

use crate::{cli::{Cli, Commands, AddCmd}};
use store::{TodoStore, JsonStore};
use crate::error::TaskerError;

mod cli;
mod store;
mod todo;
mod error;

fn main() -> Result<(), TaskerError> {
    let cli = Cli::parse();

    let store = JsonStore{
        path: "todos.json".into()
    };

    match cli.command {
        Commands::Add(AddCmd { task, desc }) => {
            store.add(task, desc)?;
        },
      Commands::List => store.list()?,
      Commands::Delete { id } => store.delete(id)?, 
    }
    Ok(())
}
