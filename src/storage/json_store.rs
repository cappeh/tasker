use std::{fs, io};
use std::path::PathBuf;

use crate::storage::store::TodoStore;
use crate::models::todo::Todo;

pub struct JsonStore {
    path: PathBuf,
}

impl TodoStore for JsonStore {
    pub fn load(&self) -> Result<Vec<Todo>, io::Error> {
        if !self.path.exists() {
            return Ok(vec![]);
        }
        let data = fs::read_to_string(&self.path)?;
        let todos = serde_json::from_str(&data)?;
        Ok(todos)
    }

    pub fn add(&self, todo: Todo) -> Result<(), io::Error> {
        let mut todos = self.load()?;
        todos.push(todo);
        fs::write(&self.path, serde_json::to_string_pretty(&todos)?)?;
        Ok(())
    }
}
