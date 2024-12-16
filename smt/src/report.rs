use std::fs::File;
use std::io::{Write, Error};
use log::{info, error};

pub fn save(path: &str, content: &str) -> Result<(), String> {
    match write_to_file(path, content) {
        Ok(()) => {
            info!("written to file: {}", path);
            Ok(())
        }
        Err(e) => {
            error!("Failed to write to file: {}", e);
            Err(format!("Failed to write to file: {}", e))
        }
    }
}

fn write_to_file(path: &str, content: &str) -> Result<(), Error> {
    let mut file = File::create(path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}
