use anyhow::{Context, Result};
use indicatif::ProgressBar;

pub fn open_file(path: std::path::PathBuf, pb: ProgressBar) -> Result<()> {
    pb.set_message(format!("Opening file {}", path.display()));
    pb.inc(1);

    if path.exists() {
        opener::open(&path)
            .with_context(|| format!("could not open file `{}`", path.display()))?;
        println!("File '{}' opened successfully.", path.display());
    } else {
        anyhow::bail!("file '{}' does not exist", path.display());
    }

    Ok(())
}