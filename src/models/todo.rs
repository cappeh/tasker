use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: u64,
    pub task: String,
}

impl Todo {
    pub fn next_id(todos: &[Todo]) -> u64 {
        todos.iter().map(|t| t.id).max().unwrap_or(0)
    }
}
