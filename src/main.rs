use clap::parser;
use serde::{Deserialize, Serialize};
#[derive(Serialize, Deserialize, Debug)]
struct Task {
    description: String,
    done: bool,
}


fn main() {

}