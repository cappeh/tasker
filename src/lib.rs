use clap::Parser;

use crate::cli::Cli;
use store::JsonStore;
use crate::error::TaskerError;

mod cli;
mod store;
mod todo;
pub mod error;
mod output;

pub fn run() -> Result<(), TaskerError> {
    let cli = Cli::parse();

    let store = JsonStore::new();

    cli.run(store)?;
    Ok(())
}