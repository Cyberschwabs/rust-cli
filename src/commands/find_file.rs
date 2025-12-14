use anyhow::{Context, Result};
use clap::{Parser, Subcommand};
use walkdir::WalkDir;

pub fn find_file(file: std::path::PathBuf) {
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