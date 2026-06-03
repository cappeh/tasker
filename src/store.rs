use crate::error::TaskerError;
use crate::todo::{Status, Todo};
use comfy_table::{Cell, Table, presets::ASCII_FULL};
use std::path::PathBuf;
use std::fs;

pub trait TodoStore {
    fn save(&self, todos: &[Todo]) -> Result<(), TaskerError>;
    fn load(&self) -> Result<Vec<Todo>, TaskerError>;
    fn add(&self, task: String, desc: Option<String>) -> Result<(), TaskerError>;
    fn list(&self) -> Result<(), TaskerError>;
    fn delete(&self, id: u64) -> Result<(), TaskerError>;
    fn update_status(&self, id: u64, status: Status) -> Result<(), TaskerError>;
}

pub struct JsonStore {
    pub path: PathBuf,
}

impl TodoStore for JsonStore {
    fn save(&self, todos: &[Todo]) -> Result<(), TaskerError> {
        let file = fs::File::create(&self.path)
            .map_err(|e| TaskerError::ReadError(e.to_string()))?;

        serde_json::to_writer_pretty(file, todos)
            .map_err(|e| TaskerError::WriteError(e.to_string()))?;
        Ok(())
    }

    fn load(&self) -> Result<Vec<Todo>, TaskerError> {
        if !self.path.exists() {
            return Ok(vec![]);
        }

        let file = fs::File::open(&self.path)
            .map_err(|e| TaskerError::ReadError(e.to_string()))?;

        let todos = serde_json::from_reader(file)
            .map_err(|e| TaskerError::JsonError(e.to_string()))?;

        Ok(todos)
    }

    fn add(&self, task: String, desc: Option<String>) -> Result<(), TaskerError> {
        let mut todos = self.load()?;

        let id = Todo::next_id(&todos);
        let todo = Todo::new(id, task, desc.unwrap_or_default());

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
            Cell::new("Description"),
            Cell::new("Status"),
            Cell::new("Created_At"),
        ]);
        for todo in &todos {
            table.add_row(vec![
                Cell::new(todo.id),
                Cell::new(&todo.task),
                Cell::new(&todo.description.clone().unwrap_or("".into())),
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

    fn update_status(&self, id: u64, status: Status) -> Result<(), TaskerError> {
        let mut todos = self.load()?;

        match todos.iter().position(|t| t.id == id) {
            Some(pos) => {
                todos[pos].status = status;
                self.save(&todos)?;
                Ok(())
            }
            None => Err(TaskerError::InvalidId(id))
        }
    }
}
