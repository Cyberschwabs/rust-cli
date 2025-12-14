use anyhow::{Context, Result};

pub fn open_file(path: std::path::PathBuf) -> Result<()> {
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