use clap::{Parser, Subcommand};

/// The top-level CLI structure
#[derive(Parser)]
#[command(name = "mydocker")]
#[command(about = "A minimal Docker CLI in Rust")]
pub struct Cli {
    /// The main command to execute
    #[command(subcommand)]
    pub command: Command,
}

/// Enum for top-level commands
#[derive(Subcommand)]
pub enum Command {
    /// List resources
    List {
        /// Subcommands for listing resources
        #[command(subcommand)]
        list_command: ListCommands,
    },

    /// Start a container
    Start {
        /// The container name
        container_name: String,
    },
    /// Stop a container
    Stop {
        /// The container Name
        container_name: String,
    },
    /// Pull an image
    Pull {
        /// The image Name
        image_name: String,
    },
}

/// Enum for subcommands under `list`
#[derive(Subcommand)]
pub enum ListCommands {
    /// List containers
    Containers {
        /// Include stopped containers
        #[arg(short, long)]
        all: bool,
    },
    /// List images
    Images,
}
