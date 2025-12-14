use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;

pub fn read_file(path: PathBuf, pattern: String) -> Result<()> {
    let content: String = fs::read_to_string(&path)
        .with_context(|| format!("could not read file `{}`", path.display()))?;

    println!("Searching for pattern '{}'\n", pattern);
    println!("Reading file '{}'\n", path.display());

    let mut found = false;

    for (index, line) in content.lines().enumerate() {
        if line.contains(&pattern) {
            println!("Line {}: {}", index + 1, line);
            found = true;
        }
    }

    if !found {
        println!("Could not find pattern: '{}'", pattern);
    }

    Ok(())
}
