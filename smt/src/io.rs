use std::fs::read_to_string;
use std::path::Path;
use std::io::{Read, Write};

pub struct File;

impl File {
    pub fn exists(path: &String) -> bool {
        Path::new(path).exists()
    }

    pub fn is_file(path: &String) -> bool {
        File::exists(path) && Path::new(path).is_file()
    }

    pub fn write(path: &str, content: &[u8]) -> std::io::Result<()> {
        let mut file = std::fs::File::create(path)?;
    
        file.write_all(content)?;
    
        Ok(())
    }
    
    pub fn open(path: &str) -> std::io::Result<Vec<u8>> {
        let mut file: std::fs::File = std::fs::File::open(path)?;
        let mut contents: Vec<u8> = Vec::new();

        file.read_to_end(&mut contents)?;
    
        Ok(contents)
    }

    pub fn read_lines(filename: &str) -> Vec<String> {
        let mut result = Vec::new();
    
        for line in read_to_string(filename).unwrap().lines() {
            result.push(line.trim_end().to_string());
        }
    
        result
    }
}
