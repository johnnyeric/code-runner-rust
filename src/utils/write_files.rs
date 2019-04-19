use std::io::{Error};
use std::result::Result;
use types::InMemoryFile;
use utils::write_file;
use tempfile::TempDir;

pub fn write_files(base_path: &TempDir, files: &Vec<InMemoryFile>) -> Result<Vec<String>, Error> {
    let mut paths = Vec::<String>::new();
    for file in files {
        let path = write_file(base_path, &file)?;
        paths.push(path);
    }

    Ok(paths)
}