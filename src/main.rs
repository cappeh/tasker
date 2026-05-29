use clap::Parser;

use crate::{cli::{Cli, Commands}};
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
      Commands::Add { task } => store.add(task)?,
      Commands::List => store.list()?,
      Commands::Delete { id } => store.delete(id)?, 
    }
    Ok(())
}
