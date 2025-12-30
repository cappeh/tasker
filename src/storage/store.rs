use std::io;

use crate::models::todo::Todo;

pub trait TodoStore {
    fn load(&self) -> Result<Vec<Todo>, io::Error>;
    fn add(&self, task: String) -> Result<(), io::Error>;
    fn list(&self) -> Result<(), io::Error>;
}
