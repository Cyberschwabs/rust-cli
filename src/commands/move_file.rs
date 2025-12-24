use std::fs;
use std::io; 
use indicatif::ProgressBar;

pub fn move_file(source: std::path::PathBuf, destination: std::path::PathBuf, pb: ProgressBar) -> Result<(), io::Error> {
    fs::rename(&source, &destination)?;

    println!("\nMoving file from {:?} to {:?}...", source, destination);

    pb.inc(1);
    Ok(())
}