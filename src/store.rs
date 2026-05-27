use std::{fs, io};
use std::path::PathBuf;
use comfy_table::{Table, Cell, presets::ASCII_FULL};
use crate::todo::Todo;

pub trait TodoStore {
    fn save(&self, todos: &[Todo]) -> Result<(), io::Error>;
    fn load(&self) -> Result<Vec<Todo>, io::Error>;
    fn add(&self, task: String) -> Result<(), io::Error>;
    fn list(&self) -> Result<(), io::Error>;
    fn delete(&self, id: u64) -> Result<(), io::Error>;
}


pub struct JsonStore {
    pub path: PathBuf,
}

impl TodoStore for JsonStore {
    fn save(&self, todos: &[Todo]) -> Result<(), io::Error> {
        fs::write(&self.path, serde_json::to_string_pretty(todos)?)?;
        Ok(())
    } 

    fn load(&self) -> Result<Vec<Todo>, io::Error> {
        if !self.path.exists() {
            return Ok(vec![]);
        }
        let data = fs::read_to_string(&self.path)?;
        let todos = serde_json::from_str(&data)?;
        Ok(todos)
    }

    fn add(&self, task: String) -> Result<(), io::Error> {
        let mut todos = self.load()?;

        let id = Todo::next_id(&todos);
        let todo = Todo::new(id, task);

        todos.push(todo);
        self.save(&todos)?;
        Ok(())
    }

    fn list(&self) -> Result<(), io::Error> {
        let todos = self.load()?;
        let mut table = Table::new();
        table.load_preset(ASCII_FULL)
            .set_header(vec![
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

    fn delete(&self, id: u64) -> Result<(), io::Error> {
        let mut todos = self.load()?;
        if let Some(pos) = todos.iter().position(|t| t.id == id) {
            todos.remove(pos);
        }
        self.save(&todos)?;
        Ok(())
    }
}
