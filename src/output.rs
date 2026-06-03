use comfy_table::presets::ASCII_FULL;
use comfy_table::{Cell, Table, Color};
use crate::todo::{Todo, Status};

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
        let status_cell = match todo.status {
            Status::Complete => Cell::new(&todo.status).fg(Color::Green),
            Status::InProgress => Cell::new(&todo.status).fg(Color::Yellow),
            Status::ToDo => Cell::new(&todo.status).fg(Color::Red),
        };

        table.add_row(vec![
            Cell::new(todo.id),
            Cell::new(&todo.task),
            Cell::new(todo.description.as_deref().unwrap_or("")),
            status_cell,
            Cell::new(todo.human_friendly_datetime()),
        ]);
    }

    println!("{table}");
}
