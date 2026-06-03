use comfy_table::presets::ASCII_FULL;
use comfy_table::{Cell, Table};
use crate::todo::Todo;

pub fn print_todos(todos: &[Todo]) {
    let mut table = Table::new();
    table.load_preset(ASCII_FULL).set_header(vec![
        Cell::new("Id"),
        Cell::new("Task"),
        Cell::new("Description"),
        Cell::new("Status"),
        Cell::new("Created_At"),
    ]);
    for todo in todos {
        table.add_row(vec![
            Cell::new(todo.id),
            Cell::new(&todo.task),
            Cell::new(&todo.description.clone().unwrap_or("".into())),
            Cell::new(&todo.status),
            Cell::new(todo.human_friendly_datetime()),
        ]);
    }
    println!("{table}");
}