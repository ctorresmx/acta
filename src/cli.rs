use clap::{Parser, Subcommand};

pub fn parse_args() -> Args {
    Args::parse()
}

pub fn handle_command(command: Commands) {
    match command {
        Commands::List { completed, pending } => println!("Listing"),
        Commands::Add { todo } => println!("Adding"),
        Commands::Complete { id } => println!("Completing"),
        Commands::Edit { id, todo } => println!("Editing"),
        Commands::Delete { id } => println!("Deleting"),
        Commands::Export {} => println!("Exporting"),
    }
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    List {
        #[arg(short, long, default_value_t = false, conflicts_with = "pending")]
        completed: bool,

        #[arg(short, long, default_value_t = false, conflicts_with = "completed")]
        pending: bool,
    },
    Add {
        #[arg(short, long)]
        todo: String,
    },
    Complete {
        #[arg(short, long)]
        id: u64,
    },
    Edit {
        #[arg(short, long)]
        id: u64,

        #[arg(short, long)]
        todo: String,
    },
    Delete {
        #[arg(short, long)]
        id: u64,
    },
    Export {},
}
