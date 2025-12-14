use clap::{Parser, Subcommand};
use anyhow::Result;

// Declare modules
mod commands;

use commands::find_file::find_file;
use commands::open_file::open_file;
use commands::read_file::read_file;

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
        path: std::path::PathBuf,

        #[arg(long)]
        pattern: String,
    },

    /// Open the file at the given path
    Open {
        #[arg(long)]
        path: std::path::PathBuf,
    },

    /// Find a file on the system
    Find {
        #[arg(long)]
        file: std::path::PathBuf,
    },
}

fn main() -> Result<()> {
    let args: Cli = Cli::parse();

    match args.command {
        Commands::Pattern { path, pattern } => {
            read_file(path, pattern)?;
        }

        Commands::Open { path } => {
            open_file(path)?;
        }

        Commands::Find { file } => {
            find_file(file);
        }
    }

    Ok(())
}
