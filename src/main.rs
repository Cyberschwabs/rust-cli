use anyhow::{Context, Result};
use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
    #[arg(long = "pattern")]
    pattern: String,
    /// The path to the file to read
    #[arg(long = "path")]
    path: std::path::PathBuf,
}

fn main() {
    read_file().unwrap();
}

fn read_file() -> Result<()> {
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;   

    println!("Searching for '{}' in file '{}'\n", args.pattern, args.path.to_string_lossy());

    for line in content.lines() {
        // Dont check empty lines
        if line.contains(&args.pattern) {
            println!("{} Found on line: {}", &args.pattern, line);
        }
        else if line.is_empty() || !line.contains(&args.pattern) {
            println!("Pattern: {}; not found in line: {}", &args.pattern, line);
            continue;
        }
    };
    Ok(())
}