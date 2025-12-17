use anyhow::{Context, Result};
use std::fs;
use std::path::PathBuf;
use walkdir::{DirEntry, WalkDir};

/// Detect hidden files/directories in a cross-platform way
fn is_hidden(entry: &DirEntry) -> bool {
    // Unix-like systems: names starting with "."
    #[cfg(unix)]
    {
        entry
            .file_name()
            .to_str()
            .map(|s| s.starts_with('.'))
            .unwrap_or(false)
    }

    // Windows: FILE_ATTRIBUTE_HIDDEN
    #[cfg(windows)]
    {
        use std::os::windows::fs::MetadataExt;
        entry
            .metadata()
            .map(|m| m.file_attributes() & 0x2 != 0)
            .unwrap_or(false)
    }
}

/// Read files and search for a pattern
pub fn read_file(path: Option<PathBuf>, pattern: String) -> Result<()> {
    if pattern.is_empty() {
        println!("Empty pattern; nothing to search.\n");
        return Ok(());
    }

    match path {
        // No path provided: walk default roots
        None => {
            let roots: &[&str] = if cfg!(windows) {
                &["C:\\"]
            } else {
                &["/home"]
            };

            for root in roots {
                println!("\nScanning directory '{}' for pattern '{}'\n", root, pattern);

                for entry in WalkDir::new(root)
                    .follow_links(false)
                    .into_iter()
                    .filter_entry(|e| !is_hidden(e))
                    .filter_map(|e| e.ok())
                {
                    if !entry.file_type().is_file() {
                        continue;
                    }

                    let path = entry.path();

                    // Skip large files (>10MB)
                    let metadata = match entry.metadata() {
                        Ok(m) => m,
                        Err(_) => continue,
                    };

                    if metadata.len() > 10_000_000 {
                        continue;
                    }

                    let content = match fs::read_to_string(path) {
                        Ok(c) => c,
                        Err(_) => continue,
                    };

                    // Skip binary files
                    if content.contains('\0') {
                        continue;
                    }

                    for (index, line) in content.lines().enumerate() {
                        if line.contains(&pattern) {
                            println!(
                                "{}: Line: {} - {}\n",
                                path.display(),
                                index + 1,
                                line
                            );
                        }
                    }
                }
            }
        }

        // Path provided: read a single file
        Some(path) => {
            println!("Searching for pattern '{}'\n", pattern);
            println!("Reading file '{}'\n", path.display());

            let content = fs::read_to_string(&path)
                .with_context(|| format!("could not read file '{}\n'", path.display()))?;

            let mut found = false;

            for (index, line) in content.lines().enumerate() {
                if line.contains(&pattern) {
                    println!("Line {}: {}", index + 1, line);
                    found = true;
                }
            }

            if !found {
                println!("Pattern '{}' not found.\n", pattern);
            }
        }
    }

    Ok(())
}
