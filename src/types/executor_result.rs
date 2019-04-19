#[derive(Serialize, Deserialize, Debug)]
pub struct ExecutorResult {
    pub stdout: String,
    pub stderr: String,
    pub status_code: String
}