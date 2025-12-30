use std::io;

use clap::Parser;

use crate::{cli::{Cli, Commands}, 
    storage::{json_store::JsonStore, store::TodoStore}
};

mod models;
mod storage;
mod cli;

fn main() -> Result<(), io::Error> {
    let cli = Cli::parse();

    let store = JsonStore{
        path: "todos.json".into()
    };

    match cli.command {
      Commands::Add { task } => store.add(task)?,
      Commands::List => store.list()?,
    }
    Ok(())
}
