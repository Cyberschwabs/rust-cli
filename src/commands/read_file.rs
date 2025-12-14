use anyhow::{Context, Result};

pub fn read_file(path: std::path::PathBuf, pattern: String) -> Result<()> {
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