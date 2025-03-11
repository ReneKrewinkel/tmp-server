use dotenv::{ dotenv };
use super::model::Config;

pub fn read_config() -> Config {

    dotenv().ok();
    let api_token = std::env::var("OPENAI_TOKEN").expect("OPENAI_TOKEN must be set.");
    let api_url = std::env::var("OPENAI_URL").expect("OPENAI_URL must be set.");
    let api_model =  std::env::var("OPENAI_MODEL").expect("OPENAI_MODEL must be set.");
    let firebase_project = std::env::var("FIREBASE_PROJECT").expect("FIREBASE_PROJECT must be set.");
    let firebase_key_file = std::env::var("FIREBASE_KEY_FILE").expect("FIREBASE_KEY_FILE must be set.");

    Config::new(api_token.clone(),
                api_url.clone(),
                api_model.clone(),
                firebase_project.clone(),
                firebase_key_file.clone()
    )

}