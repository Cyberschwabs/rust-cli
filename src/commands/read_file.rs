
use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn read_file(path: Option<PathBuf>, pattern: String) -> Result<()> {
    match path {
        // No path provided: walk the root(s) and print matching file paths
        None => {
            let drives: &[&str] = if cfg!(windows) {
                &["C:\\"]
            } else {
                &["/"]
            };

            for drive in drives {
                println!("\nPath not given, reading all files in directory {} for pattern: {}\n",drive, pattern);

                for entry in WalkDir::new(drive)
                    .into_iter()
                    .filter_map(|res| res.ok()) // avoid Result name confusion
                {
                    let path = entry.path();

                    if path.is_file() {
                        if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                            if name.contains(&pattern) {
                                println!("{}", path.display());
                            }
                        }
                    }
                }
            }
        }

        // Path provided: read file content and search lines
        Some(p) => {
            // optional: early return if pattern is empty
            if pattern.is_empty() {
                println!("Empty pattern; nothing to search.");
                return Ok(());
            }

            let content: String = fs::read_to_string(&p)
                .with_context(|| format!("could not read file `{}`", p.display()))?;

            println!("Searching for pattern '{}'\n", pattern);
            println!("Reading file '{}'\n", p.display());

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
        }
    }

    Ok(())
}