use clap::{Parser, Subcommand};
use anyhow::Result;
use tokio;
use indicatif::{ProgressBar};
use std::time::Duration;

// Declare modules
mod commands;

use commands::find_file::find_file;
use commands::open_file::open_file;
use commands::find_pattern_file::find_pattern_file;
use commands::copy_file::copy_file;
use commands::move_file::move_file;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Cli {
    /// Turn debugging information on
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    debug: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Search for a pattern in a file
    Pattern {
        #[arg(long)]
        path: Option<std::path::PathBuf>,

        #[arg(short, long)]
        pattern: String,

        #[arg(short, long)]
        large: bool,
    },
    /// Open the file at the given path
    Open {
        #[arg(short, long)]
        path: std::path::PathBuf,
    },
    /// Find a file on the system
    Find {
        #[arg(short, long)]
        file: std::path::PathBuf,
    },
    /// Copy File
    Copy {
        #[arg(short, long)]
        source: std::path::PathBuf,

        #[arg(short, long)]
        destination: std::path::PathBuf,
    },
    /// Move File
    Move {
        #[arg(short, long)]
        source: std::path::PathBuf,

        #[arg(short, long)]
        destination: std::path::PathBuf,
    },
}

// Progress bar is created in `main` and passed to command functions so
// all branches can report progress consistently.

#[tokio::main]
async fn main() -> Result<()> {
    let args: Cli = Cli::parse();

    let pb = ProgressBar::new_spinner();
    pb.enable_steady_tick(Duration::from_millis(100));

    match args.command {
        Commands::Pattern { path, pattern, large } => {
            find_pattern_file(path, pattern, large, pb.clone())?;
        }
        Commands::Open { path } => {
            open_file(path, pb.clone())?;
        }
        Commands::Find { file } => {
            find_file(file, pb.clone());
        }
        Commands::Copy { source, destination } => {
            copy_file(source, destination, pb.clone())?;
        }
        Commands::Move { source, destination } => {
            move_file(source, destination, pb.clone())?;
        }
    }
    pb.finish_with_message("\nCompleted ✅");
    Ok(())
}