use phf::phf_map;
use std::io::{Error};
use types::ExecutorResult;
use languages;

type RunnerFunction = fn(&Vec<String>, &str) -> Result<ExecutorResult, Error>;

static RUNNERS: phf::Map<&'static str, RunnerFunction> = phf_map! {
    "javascript" => languages::javascript::run,
    "python" => languages::python::run,
    "go" => languages::golang::run,
    "rust" => languages::rust::run,
};

pub fn is_supported(language: &str) -> bool {
    RUNNERS.contains_key(language)
}

pub fn run(language: &str, files: &Vec<String>, stdin: &str) -> Result<ExecutorResult, Error> {
    RUNNERS[language](files, stdin)
}