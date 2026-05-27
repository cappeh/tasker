use std::io;

use clap::Parser;

use crate::{cli::{Cli, Commands}};
use store::{TodoStore, JsonStore};

mod cli;
mod store;
mod todo;

fn main() -> Result<(), io::Error> {
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
