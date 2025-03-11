use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct OpenAIResult {
    pub id: String,
    pub msg: String
}

impl OpenAIResult {
    pub fn new(id: String, msg: String) -> OpenAIResult {
        OpenAIResult { id, msg }
    }
}