use clap::Parser;
use todo_list::{Cli, process_action};
fn main() {
    let cli = Cli::parse();
    let _ = process_action(cli);
}

