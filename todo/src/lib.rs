use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "TodoApp")]
#[command(version = "0.1")]
#[command(about = "You won't miss any tasks here", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// List all tasks interactively
    List,

    /// Add a new task
    #[command(arg_required_else_help = true)]
    Add {
        /// Description of the task
        description: String,
    },

    /// Check a task as done
    #[command(arg_required_else_help = true)]
    Check {
        /// task numbers
        indices: Vec<usize>,
    },

    /// Uncheck a task
    #[command(arg_required_else_help = true)]
    Uncheck {
        /// task numbers
        indices: Vec<usize>,
    },

    /// Clears all tasks
    Reset,
}
