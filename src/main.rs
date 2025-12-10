use anyhow::{Context, Result};
use clap::Parser;
use human_panic::setup_panic;

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

fn main() -> Result<()> {
    setup_panic!();
    let args = Cli::parse();

    let content = std::fs::read_to_string(&args.path)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;   

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("Searching for '{}' in file '{}'\n", args.pattern, args.path.to_string_lossy());
            println!("{}", line);
        }
    };

    Ok(())
}
