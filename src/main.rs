use clap::parser;
use serde::{Serialize, Deserialize};
use serde_json;
use std::fs::{OpenOptions, File};
use std::io::{self, Write, Read};
// use serde_json;
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


fn main() {

    let task1 = Task::new("Lay Bed".to_string());
    let task2 = Task::new("Brush teeth".to_string());
    let task3 = Task::new("Audit".to_string());
    let task4 = Task::new("gym".to_string());
    let task5 = Task::new("Audit".to_string());

    let tasks = vec![task1, task2, task3, task4, task5];

    let r = save_tasks(&tasks);


    

}

fn save_tasks(tasks: &Vec<Task>) -> io::Result<()> {
    // Attempt to open the file for reading and writing, or create it if it does not exist
    let file_path = "tasks.json";
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(file_path)?;

    // Read the existing contents, if any
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

    // Truncate the file and write the updated JSON
    file.set_len(0)?; // Clear the file content
    write!(file, "{}", json)?;

    Ok(())
}