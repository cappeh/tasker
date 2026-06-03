use clap::Parser;

use crate::{cli::{Cli, Commands, AddCmd}};
use store::{TodoStore, JsonStore};
use crate::error::TaskerError;
use crate::output::print_todos;

mod cli;
mod store;
mod todo;
mod error;
mod output;

fn main() -> Result<(), TaskerError> {
    let cli = Cli::parse();

    let store = JsonStore{
        path: "todos.json".into()
    };

    match cli.command {
        Commands::Add(AddCmd { task, desc }) => {
            store.add(task, desc)?;
        },
      Commands::List => {
          let todos = store.list()?;
          print_todos(&todos);
      },
      Commands::Delete { id } => store.delete(id)?,
      Commands::Update { id, status } => store.update_status(id, status)?  
    }
    Ok(())
}
