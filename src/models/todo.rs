use serde::{Deserialize, Serialize};
use time::UtcDateTime;
use time::macros::format_description;

#[derive(Debug, Serialize, Deserialize)]
pub struct Todo {
    pub id: u64,
    pub task: String,
    pub created_at: UtcDateTime,
}

impl Todo {
    pub fn new(id: u64, task: String) -> Self {
        Todo { 
            id, 
            task,
            created_at: UtcDateTime::now(),
        }
    }

    pub fn next_id(todos: &[Todo]) -> u64 {
        todos.iter().map(|t| t.id).max().unwrap_or(0) + 1
    }

    pub fn human_friendly_datetime(&self) -> String {
        let fmt = format_description!("[day]-[month]-[year] [hour]:[minute]:[second]");
        self.created_at.format(&fmt).unwrap()
    }
}
