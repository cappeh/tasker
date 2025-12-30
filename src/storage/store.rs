use std::io;

use crate::models::todo::Todo;

pub trait TodoStore {
    pub fn load(&self) -> Result<Vec<Todo>, io::Error>;
    pub fn add(&self, todo: Todo) -> Result<(), io::Error>;
    pub fn list(&self) -> Result<(), io::Error>;
}
