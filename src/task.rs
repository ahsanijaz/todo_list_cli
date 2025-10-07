// src/tasks.rs

pub struct TodoList {
    pub tasks: Vec<String>,
}

impl TodoList {
    pub fn new() -> TodoList {
        TodoList { tasks: Vec::new() }
    }

    pub fn add_task(&mut self, task: String) {
        self.tasks.push(task);
    }

    pub fn complete_task(&mut self, index: usize) {
        if index < self.tasks.len() {
            self.tasks[index] = format!("[x] {}", self.tasks[index]);
        }
    }

    pub fn list_tasks(&self) {
        for (index, task) in self.tasks.iter().enumerate() {
            println!("{}: {}", index, task);
        }
    }
}
