// src/main.rs

mod storage;
mod task;

use task::TodoList;

fn main() {
    let mut my_todos = TodoList::new();

    my_todos.add_task("Write a blog post".to_string());
    my_todos.add_task("Learn Rust modules".to_string());

    println!("Initial to-do list:");
    my_todos.list_tasks();

    my_todos.complete_task(1);

    println!("\nUpdated to-do list:");
    my_todos.list_tasks();

    // Save the to-do list to a file
    if let Err(e) = storage::save_tasks("my_todos.txt", &my_todos) {
        eprintln!("Error saving tasks: {}", e);
    }

    // Load the to-do list from the file
    match storage::load_tasks("my_todos.txt") {
        Ok(loaded_todos) => {
            println!("\nLoaded to-do list from file:");
            loaded_todos.list_tasks();
        }
        Err(e) => {
            eprintln!("Error loading tasks: {}", e);
        }
    }
}
