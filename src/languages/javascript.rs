use std::io::{Error};
use types::ExecutorResult;
use std::path::Path;
use executor;

pub fn run(files: &Vec<String>, stdin: &str) -> Result<ExecutorResult, Error> {
    let parent = Path::new(&files[0]).parent();

    match parent {
        Some(work_dir) => {
            executor::run_stdin(&work_dir.to_string_lossy(), &stdin, &["node", &files[0]])
        }
        None => {
            panic!("Error finding work dir");
        }
    }
    
}