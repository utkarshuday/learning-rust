use std::error::Error;

use clap::Parser;
use todo::{Cli, Commands};

use crate::todo_list::TodoList;

mod todo_list;

fn main() {
    let cli = Cli::parse();
    if let Err(e) = run(cli) {
        eprintln!("Error: {}", e);
    };
}

fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    let mut todo_list = TodoList::new()?;
    match cli.command {
        Commands::Add { description } => {
            todo_list.add(description)?;
            println!("New task added!");
            todo_list.list();
        }
        Commands::Reset => {
            todo_list.clear()?;
            println!("All tasks cleared");
        }
        Commands::List => {
            todo_list.list();
        }
        Commands::Check { indices } => {
            todo_list.check(indices)?;
            todo_list.list();
        }
        Commands::Uncheck { indices } => {
            todo_list.uncheck(indices)?;
            todo_list.list();
        }
    }
    Ok(())
}
