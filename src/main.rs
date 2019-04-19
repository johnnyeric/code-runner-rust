#![feature(proc_macro_hygiene)]
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate phf;
extern crate tempfile;

mod types;
mod utils;
mod executor;
mod languages;

use std::path::Path;
use std::io::{self, Read};
use utils::parse_payload;
use utils::write_files;
use tempfile::tempdir;

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer)?;

    let temp_dir = tempdir()?;
    let payload = parse_payload(&buffer)?;
    let filepaths = write_files(&temp_dir, &payload.files)?;

    let executor_result = if payload.command == "" {
        languages::run(&payload.language, &filepaths, &payload.stdin)
    } else {
        let parent = Path::new(&filepaths[0]).parent();

        match parent {
            Some(work_dir) => {
                executor::run_bash_stdin(&work_dir.to_string_lossy(), &payload.command, &payload.stdin)
            }
            None => {
                panic!("Error finding work dir");
            }
        }
    };

    let mut result = types::Result::new();
    
    match executor_result {
        Ok(output) => {
            result.stdout = output.stdout;
            result.stderr = output.stderr;

            let json = serde_json::to_string(&result)?;
            println!("{}", json);
        }
        Err(_e) => {
            result.error = format!("{}", _e);

            let json = serde_json::to_string(&result)?;
            println!("{}", json);
        }
    }

    temp_dir.close()?;

    Ok(())
}