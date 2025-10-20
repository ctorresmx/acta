mod cli;

use acta::persistence;

use crate::cli::{handle_command, parse_args};

fn main() {
    let args = parse_args();

    let _ = persistence::init();

    match args.command {
        Some(command) => handle_command(command),
        // If no command is passed with default to the TUI flow
        None => panic!("TUI has not yet been implemented"),
    }
}
