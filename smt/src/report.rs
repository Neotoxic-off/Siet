use std::io::{Read, Write};
use std::fs::File;
use log::{info, error};

pub fn save(path: &str, content: &str) -> () {
    match write_to_file(file_path, content) {
        Ok(()) => {
            info!("Environment variables written to file: {}", file_path);
            Ok(())
        }
        Err(e) => Err(format!("Failed to write environment variables to file: {}", e)),
    }
}