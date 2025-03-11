use serde::Deserialize;
#[derive(Deserialize, Clone)]
pub struct Prompt {
    pub id: String,
    pub prompt: String
}
