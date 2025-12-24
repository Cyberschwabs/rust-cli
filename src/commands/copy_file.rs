use std::fs;
use std::io; 
use indicatif::ProgressBar;

pub fn copy_file(source: std::path::PathBuf, destination: std::path::PathBuf, pb: ProgressBar) -> Result<(), io::Error> {
    fs::copy(&source, &destination)?;

    println!("\nCopying file from {:?} to {:?}...", source, destination);

    pb.inc(1);
    Ok(())
}