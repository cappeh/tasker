use crate::error::TaskerError;
use crate::todo::{Status, Todo};
use std::path::PathBuf;
use std::fs;

pub trait TodoStore {
    fn save(&self, todos: &[Todo]) -> Result<(), TaskerError>;
    fn load(&self) -> Result<Vec<Todo>, TaskerError>;
    fn add(&self, task: String, desc: Option<String>) -> Result<(), TaskerError>;
    fn list(&self) -> Result<Vec<Todo>, TaskerError>;
    fn delete(&self, id: u64) -> Result<(), TaskerError>;
    fn update_status(&self, id: u64, status: Status) -> Result<(), TaskerError>;
}

pub struct JsonStore {
    pub path: PathBuf,
}

impl JsonStore {
    pub fn new() -> Self {
        let dir = match std::env::var_os("_TASKER_DATA_DIR") {
            Some(path) => PathBuf::from(path),
            None => dirs::data_local_dir().unwrap().join("tasker")
        };

        fs::create_dir_all(&dir).unwrap();
        let path = dir.join("db.json");
        Self { path }
    }
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

    fn list(&self) -> Result<Vec<Todo>, TaskerError> {
        Ok(self.load()?)
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
