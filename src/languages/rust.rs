use std::io::{Error};
use types::ExecutorResult;
use std::path::Path;
use executor;

pub fn run(files: &Vec<String>, stdin: &str) -> Result<ExecutorResult, Error> {
    let parent = Path::new(&files[0]).parent();
    let bin_name: String = "a.out".to_string();

    match parent {
        Some(work_dir) => {
            let result = executor::run(&work_dir.to_string_lossy(), &["rustc", "-o", &bin_name, &files[0]]);

            match result {
                Ok(_compiler_result) => {
                    let path = Path::new(work_dir).join(bin_name);
                    executor::run_stdin(&work_dir.to_string_lossy(), &stdin, &[&path.as_path().to_string_lossy()])
                }
                Err(_e) => {
                    panic!("Error to execute rust code {:?}", _e)
                }
            }
        }
        None => {
            panic!("Error finding work dir");
        }
    }
}