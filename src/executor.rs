use std::io::{Write, Error, ErrorKind};
use std::process::{Command, Stdio};
use std::result::Result;
use types::ExecutorResult;

pub fn run_stdin(work_dir: &str, stdin: &str, args: &[&str]) -> Result<ExecutorResult, Error> {
    let mut command = Command::new(&args[0]);
    command
        .args(&args[1..])
        .current_dir(work_dir)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());

    let mut child = command
        .spawn()?;
        //.expect("Failed to spawn child process");

    {
        let stdin_stream = child.stdin.as_mut().ok_or(Error::new(ErrorKind::Other, "Failed to open stdin"))?;//.expect("Failed to open stdin");
        stdin_stream.write_all(stdin.as_bytes())?;//.expect("Failed to write to stdin");
    }

    let output = child.wait_with_output()?;//.expect("Failed to read stdout");

    let code = match output.status.code() {
        Some(code) => code,
        None       => -1
    };

    Ok(ExecutorResult{
        stdout: String::from_utf8_lossy(&output.stdout).to_string(),
        stderr: String::from_utf8_lossy(&output.stderr).to_string(),
        status_code: code.to_string()
    })
}

pub fn run(work_dir: &str, args: &[&str]) -> Result<ExecutorResult, Error>{
    run_stdin(work_dir, "", &args)
}

pub fn run_bash_stdin(work_dir: &str, command: &str, stdin: &str) -> Result<ExecutorResult, Error> {
    match run_stdin(work_dir, stdin, &["bash", "-c", command]) {
        Ok(executor_result) => Ok(executor_result),
        Err(e) => {
            if let ErrorKind::NotFound = e.kind() {
                println!("aqui");
                run_stdin(work_dir, stdin, &["sh", "-c", command])
            } else {
                Err(e)
            }
        }
    }
}

/* pub fn run_bash(work_dir: &str, command: &str) -> Result<ExecutorResult, Error> {
    run(work_dir, &["bash", "-c", command])
} */
