use types::InMemoryFile;

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    pub language: String,
    pub files: Vec<InMemoryFile>,
    pub stdin: String,
    pub command: String
}