use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, Write};

#[derive(Parser)]
// #[command(name = "Todo List", version = "1.0", about = "A simple CLI To-Do list")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add a new task
    Add { description: String },
    /// List all tasks
    List,
    /// Mark a task as done
    Done { index: usize },
}
#[derive(Serialize, Deserialize, Debug, Clone)]
struct Task {
    description: String,

    done: bool,
}

fn main() {
    let cli = Cli::parse();
    let _ = process_action(cli);
}
fn process_action(cli: Cli) -> Result<(), io::Error> {
    match cli.command {
        Commands::Add { description } => {
            let mut tasks = Vec::new();
            tasks.push(Task {
                description,
                done: false,
            });
            let list_items = save_tasks(&tasks);
            println!("Task added!:{:?}", list_items);
        }
        Commands::List => {
            let (tasks, _) = load_tasks().unwrap();
            for (i, task) in tasks.iter().enumerate() {
                println!("{}: {:?}", i + 1, task);
            }
        }
        Commands::Done { index } => {
            let (mut tasks, file) = load_tasks()?;
            if index == 0 || index > tasks.len() {
                println!("Invalid task index.");
            } else {
                file.set_len(0)?;
                tasks[index - 1].done = true;
                save_tasks(&tasks)?;
                println!("Task marked as done!");
            }
        }
    }

    Ok(())
}

fn load_tasks() -> std::io::Result<(Vec<Task>, File)> {
    let file_path = "tasks.json";
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)?;
    let mut existing_data = String::new();
    file.read_to_string(&mut existing_data)?;

    // Deserialize the existing JSON into a vector of tasks
    let existing_tasks: Vec<Task> = if !existing_data.trim().is_empty() {
        serde_json::from_str(&existing_data).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    };
    let output = (existing_tasks, file);
    Ok(output)
}

fn save_tasks(tasks: &Vec<Task>) -> io::Result<()> {
    // Deserialize the existing JSON into a vector of tasks
    let (mut existing_tasks, mut file) = load_tasks().unwrap();

    // Append the new tasks to the existing tasks
    existing_tasks.extend_from_slice(tasks);

    // Serialize the combined tasks to JSON
    let json = serde_json::to_string(&existing_tasks)?;

    file.set_len(0)?; // Clear the file content
    file.seek(io::SeekFrom::End(0))?;
    write!(file, "{}", json)?;

    Ok(())
}
