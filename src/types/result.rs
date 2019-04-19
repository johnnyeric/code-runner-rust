#[derive(Serialize, Deserialize, Debug)]
pub struct Result {
    pub stdout: String,
    pub stderr: String,
    pub error: String
}

impl Result {
    pub fn new() -> Result {
        Result {stdout: String::new(), stderr: String::new(), error: String::new()}
    }
}