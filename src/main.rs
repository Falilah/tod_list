use serde::{Serialize, Deserialize};
use serde_json;
use std::fs::{OpenOptions, File};
use std::io::{self, Write, Read, Seek};
use clap::{Parser, Subcommand};


#[derive(Parser)]
#[command(name = "Todo List", version = "1.0", about = "A simple CLI To-Do list")]
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

impl Task{
    fn new(des: String) -> Task{
        Task { description: des, done: false }
    }
}


// fn main() {

//     let task1 = Task::new("Lay Bed".to_string());
//     let task2 = Task::new("Brush teeth".to_string());
//     let task3 = Task::new("Audit".to_string());
//     let task4 = Task::new("gym".to_string());
//     let task5 = Task::new("Audit".to_string());

//     let tasks = vec![task1, task2, task3, task4, task5];

//     let r = save_tasks(&tasks);


    

// }

fn main()  {
    let cli = Cli::parse();
    let file_path = "tasks.json";

    match cli.command {
        Commands::Add { description } => {
            let mut tasks = Vec::new();
            tasks.push(Task {
                description,
                done: false,
            });
            save_tasks(&tasks);
            println!("Task added!");
        }
        Commands::List => {
            let tasks: Result<Vec<Task>, io::Error> = load_tasks(file_path);
            for (i, task) in tasks.iter().enumerate() {
                println!(
                    "{}. [{}] {}",
                    i + 1,
                    if task.done { "x" } else { " " },
                    task.description
                );
            }
        }
        Commands::Done { index } => {
            let mut tasks = load_tasks(file_path)?;
            if index == 0 || index > tasks.len() {
                println!("Invalid task index.");
            } else {
                tasks[index - 1].done = true;
                save_tasks(file_path, &tasks)?;
                println!("Task marked as done!");
            }
        }
    }

    Ok(())
}


fn load_tasks(file_path: &str) -> std::io::Result<Vec<Task>> {
    let mut file = match File::open(file_path) {
        Ok(file) => file,
        Err(_) => return Ok(Vec::new()), // If file does not exist, return an empty task list
    };
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let tasks: Vec<Task> = serde_json::from_str(&contents).unwrap_or_else(|_| Vec::new());
    Ok(tasks)
}

fn save_tasks(tasks: &Vec<Task>) -> io::Result<()> {
    let file_path = "tasks.json";
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)?;
    let mut existing_data = String::new();
    file.read_to_string(&mut existing_data)?;

    // Deserialize the existing JSON into a vector of tasks
    let mut existing_tasks: Vec<Task> = if !existing_data.trim().is_empty() {
        serde_json::from_str(&existing_data).unwrap_or_else(|_| Vec::new())
    } else {
        Vec::new()
    };

    // Append the new tasks to the existing tasks
    existing_tasks.extend_from_slice(tasks);

    // Serialize the combined tasks to JSON
    let json = serde_json::to_string(&existing_tasks)?;
    // writeln!(file)?;

    file.set_len(0)?; // Clear the file content
    file.seek(io::SeekFrom::End(0))?;
    write!(file, "{}", json)?;

    Ok(())
}