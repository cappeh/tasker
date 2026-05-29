use crate::error::TaskerError;
use crate::todo::Todo;
use comfy_table::{Cell, Table, presets::ASCII_FULL};
use std::path::PathBuf;
use std::{fs, io};

pub trait TodoStore {
    fn save(&self, todos: &[Todo]) -> Result<(), TaskerError>;
    fn load(&self) -> Result<Vec<Todo>, TaskerError>;
    fn add(&self, task: String) -> Result<(), TaskerError>;
    fn list(&self) -> Result<(), TaskerError>;
    fn delete(&self, id: u64) -> Result<(), TaskerError>;
}

pub struct JsonStore {
    pub path: PathBuf,
}

impl TodoStore for JsonStore {
    fn save(&self, todos: &[Todo]) -> Result<(), TaskerError> {
        let contents = serde_json::to_string_pretty(todos)
            .map_err(|_| TaskerError::JsonError("Error Serializing Todos to JSON".into()))?;

        fs::write(&self.path, contents)
            .map_err(|_| TaskerError::WriteError("Error Writing Todos to JSON".into()))?;
        Ok(())
    }

    fn load(&self) -> Result<Vec<Todo>, TaskerError> {
        if !self.path.exists() {
            return Ok(vec![]);
        }
        let data = fs::read_to_string(&self.path).map_err(|_| TaskerError::ReadError("Error Reading JSON".into()))?;
        let todos = serde_json::from_str(&data).map_err(|_| TaskerError::JsonError("Error Deserializing JSON".into()))?;
        Ok(todos)
    }

    fn add(&self, task: String) -> Result<(), TaskerError> {
        let mut todos = self.load()?;

        let id = Todo::next_id(&todos);
        let todo = Todo::new(id, task);

        todos.push(todo);
        self.save(&todos)?;
        Ok(())
    }

    fn list(&self) -> Result<(), TaskerError> {
        let todos = self.load()?;
        let mut table = Table::new();
        table.load_preset(ASCII_FULL).set_header(vec![
            Cell::new("Id"),
            Cell::new("Task"),
            Cell::new("Status"),
            Cell::new("Created_At"),
        ]);
        for todo in &todos {
            table.add_row(vec![
                Cell::new(todo.id),
                Cell::new(&todo.task),
                Cell::new(&todo.status),
                Cell::new(todo.human_friendly_datetime()),
            ]);
        }
        println!("{table}");
        Ok(())
    }

    fn delete(&self, id: u64) -> Result<(), TaskerError> {
        let mut todos = self.load()?;

        match todos.iter().position(|t| t.id == id) {
            Some(pos) => {
                todos.remove(pos);
                self.save(&todos)?;
                Ok(())
            }
            None => Err(TaskerError::InvalidId(id)),
        }
    }
}
