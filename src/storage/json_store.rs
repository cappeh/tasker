use std::{fs, io};
use std::path::PathBuf;
use comfy_table::{Table, Cell, presets::ASCII_FULL};

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

    pub fn add(&self, mut todo: Todo) -> Result<(), io::Error> {
        let mut todos = self.load()?;
        let id = Todo::next_id(&todos);
        todo.id = id;
        todos.push(todo);
        fs::write(&self.path, serde_json::to_string_pretty(&todos)?)?;
        Ok(())
    }

    pub fn list(&self) -> Result<(), io::Error> {
        let todos = self.load()?;
        let mut table = Table::new();
        table.load_preset(ASCII_FULL)
            .set_header(vec![
                Cell::new("Id"),
                Cell::new("Task"),
            ]);
        for todo in &todos {
            table.add_row(vec![
                Cell::new(todo.id),
                Cell::new(&todo.task),
            ]);
        }
        println!("{table}");
        Ok(())
    }
}
