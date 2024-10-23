use clap::parser;
use serde::{Deserialize, Serialize};
use std::fs::File;
use serde_json;
#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    done: bool,
}


fn main() {

    

}

fn save_tasks(tasks: &Vec<Task>) -> std::io::Result<()> {
    let file = File::create("tasks.json")?;
    let json = serde_json::to_string(tasks)?;
    write!(file, "{}", json)?;
    Ok(())
}