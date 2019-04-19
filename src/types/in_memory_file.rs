#[derive(Serialize, Deserialize, Debug)]
pub struct InMemoryFile {
    pub name: String,
    pub content: String
}