use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
struct Task {
    id: usize,
    description: String,
    done: bool,
}

#[derive(Debug, Serialize, Deserialize)]
struct TodoList {
    tasks: Vec<Task>,
}

impl TodoList {
    fn new() -> Self {
        Self { tasks: Vec::new() }
    }

    fn add_task(&mut self, description: String) {
        let id = if let Some(last) = self.tasks.last() {
            last.id + 1
        } else {
            1
        };
        let task = Task {
            id,
            description,
            done: false,
        };
        self.tasks.push(task);
        println!("✅ Task added!")
    }

    fn mark_done(&mut self, id: usize) {
        if let Some(task) = self.tasks.get_mut(id) {
            task.done = true;
            println!("✅ Task completed!")
        } else {
            println!("❌ Task not found!");
        }
    }

    fn list_tasks(&self) {
        for task in &self.tasks {
            let status = if task.done { "✅" } else { "❌" };
            println!("{} [{}] - {}", task.id, status, task.description);
        }
    }

    fn remove_task(&mut self, id: usize) {
        if let Some(index) = self.tasks.iter().position(|task| task.id == id) {
            self.tasks.remove(index);
            println!("✅ Task removed!");
        } else {
            println!("⚠️ Task not found!");
        }
    }

    fn remove_completed(&mut self) {
        self.tasks.retain(|task| !task.done);
        println!("✅ Completed tasks removed!");
    }
}

fn save_tasks(todo: &mut TodoList) -> io::Result<()> {
    let data = serde_json::to_string_pretty(todo).unwrap();
    fs::write("todo.json", data)
}

fn load_tasks() -> TodoList {
    if Path::new("todo.json").exists() {
        let data = fs::read_to_string("todo.json").unwrap();
        serde_json::from_str(&data).unwrap_or_else(|_| TodoList::new())
    } else {
        TodoList::new()
    }
}

fn main() {
    let mut todo = load_tasks();

    println!("🦀 Welcome to Rusty Todo!");
    println!("Type 'help' to see available commands.\n");

    loop {
        print!("> ");
        io::stdout().flush().unwrap(); // ensure prompt prints before reading

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0];
        let args = parts[1..].join(" ");

        match command {
            "add" => {
                if args.is_empty() {
                    println!("⚠️  Usage: add <task description>");
                } else {
                    todo.add_task(args);
                }
            }
            "list" => todo.list_tasks(),
            "done" => {
                if let Ok(id) = args.parse::<usize>() {
                    todo.mark_done(id);
                } else {
                    println!("⚠️  Usage: done <task_id>");
                }
            }
            "remove" => {
                if let Ok(id) = args.parse::<usize>() {
                    todo.remove_task(id);
                } else {
                    println!("⚠️  Usage: remove <task_id>");
                }
            }
            "remove_completed" => todo.remove_completed(),
            "help" => {
                println!("Available commands:");
                println!("  add <description>       - Add a new task");
                println!("  list                    - Show all tasks");
                println!("  done <id>               - Mark a task as done");
                println!("  remove <id>             - Delete a task");
                println!("  remove_completed        - Delete all completed tasks");
                println!("  exit / quit             - Save and exit\n");
            }
            "exit" | "quit" => {
                save_tasks(&mut todo).unwrap();
                println!("👋 Goodbye!");
                break;
            }
            _ => println!("❌ Unknown command. Type 'help' for usage."),
        }

        save_tasks(&mut todo).unwrap();
    }
}
