use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use walkdir::WalkDir;
// use std::{env::consts::OS, os::linux, slice::Windows};

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

    Find {
        #[arg(long)]
        file: std::path::PathBuf,
    },
}

fn main() -> Result<()> {
    let args = Cli::parse();

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

fn read_file(path: std::path::PathBuf, pattern: String) -> Result<()> {
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("could not read file `{}`", path.display()))?;

    println!("Searching for pattern '{}'\n", pattern);
    println!("Reading file '{}'\n", path.display());

    for (index, line) in content.lines().enumerate() {
        if line.contains(&pattern) {
            println!("Line {}: {}", index + 1, line);
        }
    }

    Ok(())
}

fn open_file(path: std::path::PathBuf) -> Result<()> {
    println!("Opening file '{}'", path.display());

    if path.exists() {
        opener::open(&path)
            .with_context(|| format!("could not open file `{}`", path.display()))?;
        println!("File '{}' opened successfully.", path.display());
    } else {
        anyhow::bail!("file '{}' does not exist", path.display());
    }

    Ok(())
}

fn find_file(file: std::path::PathBuf) {
    let file_name = file
        .file_name()
        .and_then(|n| n.to_str());

    if file_name.is_none() {
        return;
    }

    let file_name = file_name.unwrap();

    let drives: &[&str] = if cfg!(windows) {
        &["C:\\", "D:\\"]
    } else {
        &["/"]
    };

    for drive in drives {
        for entry in WalkDir::new(drive)
            .into_iter()
            .filter_map(Result::ok)
        {
            let path = entry.path();

            if path.is_file() {
                if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                    if name.contains(file_name) {
                        println!("{}", path.display());
                    }
                }
            }
        }
    }
}