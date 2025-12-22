use anyhow::{Context, Result};
use std::env;
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
pub fn read_file(path: Option<PathBuf>, pattern: String, large: bool) -> Result<()> {
    if pattern.is_empty() {
        println!("Empty pattern; nothing to search.\n");
        return Ok(());
    }

    if large != true {
        println!("\nLarge file search is disabled.")
    } else {
        println!("\nLarge file search is enabled.")
    }

    match path {
        // No path provided: walk the current working directory
        None => {
            let roots: Vec<PathBuf> = match env::current_dir() {
                Ok(p) => vec![p],
                Err(_) => {
                    if cfg!(windows) {
                        vec![PathBuf::from("C:\\")]
                    } else {
                        vec![PathBuf::from("/")]
                    }
                }
            };

            for root in roots.iter() {
                println!("\nScanning directory '{}' for pattern '{}'\n", root.display(), pattern);

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
                    
                    if large != true {
                        if metadata.len() > 10_000_000 {}
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
