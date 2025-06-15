use std::io::{self, Write};

fn main() {
    let mut todos: Vec<String> = Vec::new();
    println!("📝 Simple To-Do CLI - Rust");

    loop {
        print!("Enter task (or 'exit' to quit): ");
        io::stdout().flush().unwrap();
        let mut task = String::new();
        io::stdin().read_line(&mut task).expect("Failed to read");
        let task = task.trim();
        if task == "exit" {
            break;
        }
        todos.push(task.to_string());
    }

    println!("\nYour Tasks:");
    for (i, t) in todos.iter().enumerate() {
        println!("{}. {}", i + 1, t);
    }
}
