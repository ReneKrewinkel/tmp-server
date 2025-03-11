#[derive(Debug, Clone)]
pub struct Config {
    pub api_token: String,
    pub api_url: String,
    pub api_model: String,
    pub firebase_project: String,
    pub firebase_key_file: String
}

impl Config {
    pub fn new(api_token: String,
               api_url: String,
               api_model: String,
               firebase_project: String,
               firebase_key_file: String) -> Config {
        Config { api_token, api_url, api_model, firebase_project, firebase_key_file }
    }
}