use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use serde_json;
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, Write};

#[derive(Parser)]
// #[command(name = "Todo List", version = "1.0", about = "A simple CLI To-Do list")]
pub struct Cli {
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
pub fn process_action(cli: Cli) -> Result<(), io::Error> {
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

#[cfg(test)] 
mod tests{
    use super::*;
    use std::fs;
    use tempfile::tempdir;

     // Helper function to reset the tasks.json file for tests
     fn setup_temp_file() -> String {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("tasks.json");
        file_path.to_str().unwrap().to_string()
    }

    fn set_up_file_environment(file_path: &str) {
        let _ = fs::File::create(file_path);
    }
    #[test]
    fn test_add_task() {
        let temp_file = setup_temp_file();
        set_up_file_environment(&temp_file);

        let cli = Cli {
            command: Commands::Add {
                description: "Test task".to_string(),
            },
        };

        process_action(cli).unwrap();

        let (tasks, _) = load_tasks().unwrap();
        assert_eq!(tasks.len(), 1);
        assert_eq!(tasks[0].description, "Test task");
        assert!(!tasks[0].done);
    }

    #[test]
    fn test_mark_task_done() {
        let temp_file = setup_temp_file();
        set_up_file_environment(&temp_file);

        let tasks = vec![Task {
            description: "Task to complete".to_string(),
            done: false,
        }];
        save_tasks(&tasks).unwrap();

        let cli = Cli {
            command: Commands::Done { index: 1 },
        };

        process_action(cli).unwrap();

        let (updated_tasks, _) = load_tasks().unwrap();
        assert_eq!(updated_tasks.len(), 1);
        assert!(updated_tasks[0].done);
    }
}