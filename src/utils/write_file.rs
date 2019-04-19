use std::fs::File;
use std::io::{Write, Error};
use std::result::Result;
use tempfile::TempDir;
use types::InMemoryFile;

pub fn write_file(base_path: &TempDir, in_memory_file: &InMemoryFile) -> Result<String, Error> {
    let absolute_path = base_path.path().join(&in_memory_file.name);

    let mut file = File::create(&absolute_path)?;
    writeln!(file, "{}", in_memory_file.content)?;

    Ok(absolute_path.to_str().unwrap().to_string())
}