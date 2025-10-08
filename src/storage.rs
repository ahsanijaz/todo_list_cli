// src/storage.rs

use crate::task::TodoList;
use std::fs;
use std::io::{self, Write};

pub fn save_tasks(file_path: &str, todo_list: &TodoList) -> io::Result<()> {
    let mut file = fs::File::create(file_path)?;
    for task in todo_list.tasks() {
        writeln!(file, "{}", task)?;
    }
    Ok(())
}

pub fn load_tasks(file_path: &str) -> io::Result<TodoList> {
    let mut todo_list = TodoList::new();
    let content = fs::read_to_string(file_path)?;
    for line in content.lines() {
        todo_list.add_task(line.to_string());
    }
    Ok(todo_list)
}

pub fn check_tasks(todo_list: &TodoList) -> bool {
    true
}
